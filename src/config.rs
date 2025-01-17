use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use ethers::prelude::*;
use serde_json::Value;
use thiserror::Error;

const DEFAULT_BATCH_SIZE: usize = 10_000;
const DEFAULT_THREADS: usize = 8;

#[derive(Debug, Deserialize, Clone)]
pub struct PatternRule {
    pub start_with: String,
    pub end_with: String,
}

#[derive(Debug, Deserialize,Clone)]
pub struct Config {
    #[allow(unused)]
    pub abi: Value,
    pub bytecode: String,
    pub operator: String,
    pub create2: String,
    pub entrypoint: String,
    pub rpc: String,
    pub batch_size: Option<usize>,
    pub threads: Option<usize>,
    pub rules: Vec<PatternRule>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid operator address: {0}")]
    InvalidOperator(String),
    #[error("Invalid entry point address: {0}")]
    InvalidEntryPoint(String),
    #[error("Bytecode is empty or invalid")]
    InvalidBytecode,
    #[error("Invalid RPC URL")]
    InvalidRPC,
    #[error("No pattern rules defined")]
    NoRules,
    #[error("Invalid pattern rule")]
    InvalidRule,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if !self.operator.starts_with("0x") || self.operator.len() != 42 {
            return Err(ConfigError::InvalidOperator(self.operator.clone()));
        }
        if let Err(_) = self.operator.parse::<Address>() {
            return Err(ConfigError::InvalidOperator(self.operator.clone()));
        }
        if !self.create2.starts_with("0x") || self.create2.len() != 42 {
            return Err(ConfigError::InvalidOperator(self.create2.clone()));
        }
        if let Err(_) = self.create2.parse::<Address>() {
            return Err(ConfigError::InvalidOperator(self.create2.clone()));
        }
        if !self.entrypoint.starts_with("0x") || self.entrypoint.len() != 42 {
            return Err(ConfigError::InvalidEntryPoint(self.entrypoint.clone()));
        }
        if let Err(_) = self.entrypoint.parse::<Address>() {
            return Err(ConfigError::InvalidEntryPoint(self.entrypoint.clone()));
        }
        if self.bytecode.is_empty() || !self.bytecode.starts_with("0x") {
            return Err(ConfigError::InvalidBytecode);
        }
        if !self.rpc.starts_with("http") || self.rpc.len() < 10 {
            return Err(ConfigError::InvalidRPC);
        }

        if self.rules.is_empty() {
            return Err(ConfigError::NoRules);
        }

        for rule in &self.rules {
            if rule.start_with.is_empty() || rule.end_with.is_empty() {
                return Err(ConfigError::InvalidRule);
            }
        }
        Ok(())
    }


    #[allow(unused)]
    pub fn get_args_addresses(&self) ->(Address, Address) {
        let operator = self.operator.parse::<Address>().unwrap();
        let entrypoint = self.entrypoint.parse::<Address>().unwrap();
        (entrypoint,operator)
    }

    pub fn get_create2_address(&self) -> Address {
      self.operator.parse::<Address>().unwrap()
    }
    pub fn get_batch_size(&self) -> usize {
        self.batch_size.unwrap_or(DEFAULT_BATCH_SIZE)
    }

    pub fn get_threads(&self) -> usize {
        self.threads.unwrap_or(DEFAULT_THREADS)
    }
}
