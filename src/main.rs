mod config;

use ethers::prelude::*;
use rayon::prelude::*;

use ethers::{utils::keccak256};
use hex::FromHex;
use std::{thread,sync::Arc, error::Error, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};
use crate::config::{Config, PatternRule};


#[inline(always)]
pub fn is_valid_address(rules: &Vec<PatternRule>, address: &str) -> bool {
    rules.iter().any(|rule| {
        address.starts_with(&rule.start_with) &&
            address.ends_with(&rule.end_with)
    })
}
#[inline(always)]
fn generate_batch_salts(size: usize) -> Vec<[u8; 32]> {
    let mut salts = Vec::with_capacity(size);
    for _ in 0..size {
        salts.push(rand::random());
    }
    salts
}

#[inline(never)]
fn compute_create2_address(sender_address: Address, salt: &[u8; 32], deployment_data: &[u8]) -> Address {
    let bytecode_hash = keccak256(deployment_data);
    let mut data = Vec::with_capacity(85);
    data.push(0xff);
    data.extend_from_slice(sender_address.as_bytes());
    data.extend_from_slice(salt);
    data.extend_from_slice(&bytecode_hash);
    Address::from_slice(&keccak256(&data)[12..])
}

fn main() -> Result<(), Box<dyn Error>> {
    let config =  Arc::new(Config::load("config.json")?);
    let bytecode_with_args = Arc::new(Bytes::from(Vec::from_hex(config.bytecode.trim_start_matches("0x"))?));
    let create_call_address: Address = config.get_create2_address();
    let found = Arc::new(AtomicBool::new(false));
    let counter = Arc::new(AtomicUsize::new(0));

    // Spawn optimized worker threads
    let handles: Vec<_> = (0..config.get_threads()).map(|_| {
        let bytecode = bytecode_with_args.clone();
        let found = found.clone();
        let counter = counter.clone();
        let config = config.clone();
        thread::spawn(move || {
            let mut batch = generate_batch_salts(config.get_batch_size());
            while !found.load(Ordering::Relaxed) {

                if let Some((salt, addr)) = batch.par_iter().find_map_first(|salt| {
                    let address = compute_create2_address(
                        create_call_address,
                        salt,
                        &bytecode
                    );
                    let addr_str =  format!("0x{}", hex::encode(address.to_fixed_bytes())).to_lowercase();
                    println!("\rLook: {}", addr_str);
                    if is_valid_address(&config.rules, &addr_str) {
                        return Some((salt.to_vec(), addr_str));
                    }
                    None
                }) {
                    found.store(true, Ordering::Relaxed);
                    println!("\nFound perfect match!");
                    println!("Salt: 0x{}", hex::encode(salt));
                    println!("Address: {}", addr);
                    return Some(());
                }

                // Generate new batch
                batch.par_iter_mut().for_each(|salt| {
                    *salt = rand::random();
                });

                counter.fetch_add(config.get_batch_size(), Ordering::Relaxed);
                if counter.load(Ordering::Relaxed) % 100_000 == 0 {
                    println!("Processed: {} addresses", counter.load(Ordering::Relaxed));
                }
            }
            None
        })
    }).collect();

    for handle in handles {
        if handle.join().unwrap().is_some() {
            break;
        }
    }

    Ok(())
}