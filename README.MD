# CREATE2 Vanity Miner

🚀 High-performance CREATE2 vanity address miner for Ethereum smart contracts. Utilizes multi-threading and batch processing to efficiently generate customized deterministic contract addresses.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Ethereum](https://img.shields.io/badge/Ethereum-3C3C3D?style=for-the-badge&logo=Ethereum&logoColor=white)

## Features

- [x]-thread parallel processing optimized for modern CPUs
- Customizable address patterns
- Memory-efficient batch operations
- Real-time progress tracking
- Support for any EVM network
- Highly optimized Rust implementation

## Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- 8GB RAM minimum (32GB recommended for optimal performance)
- Multi-core CPU (optimized for 4 cores/8 threads)

## Installation

1. Clone the repository
```bash
git clone https://github.com/beincom/create2-vanity-miner.git
cd create2-vanity-miner
```
2. Install dependencies
```bash
cargo install --release
```
3. Release
```bash
cargo build --release
```
****
## Configuration
Place your configuration at `config.json` with the following structure:
```json
{
  "operator": "",
  "entrypoint": "",
  "create2": "",
  "rpc": "",
  "batch_size": 10000,
  "threads": 8,
  "rules": [
    {
      "start_with": "0xb139",
      "end_with": "b139"
    }
  ],
  "bytecode": "",
  "abi": []
}
```

## Usage

1. Run the miner:
```bash
cargo run --release
```

## Performance Tuning

- Adjust `BATCH_SIZE` constant to optimize for your system's memory
- Default thread count is 8, modify `THREADS` constant for different CPU configurations
- For systems with >32GB RAM, consider increasing batch size for better performance

## License

MIT License - see LICENSE file for details

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## Support

Open an issue if you:
- Find a bug
- Want to request a feature
- Have questions about usage

## Author

BIC