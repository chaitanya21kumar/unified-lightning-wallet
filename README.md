# 🌩️ Unified Lightning Wallet

> Production-ready self-custodial Bitcoin Lightning Network wallet built with Rust

[![CI](https://github.com/chaitanya21kumar/unified-lightning-wallet/workflows/CI/badge.svg)](https://github.com/chaitanya21kumar/unified-lightning-wallet/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)

A self-custodial Bitcoin wallet combining on-chain (BDK) and Lightning Network (LDK) functionality into a single, easy-to-use CLI application.

## 🚀 Features

### ✅ Implemented
- **On-Chain Wallet (BDK)**
  - HD wallet with BIP39 mnemonic support
  - Address generation and management
  - Transaction creation and broadcasting
  - Balance tracking
  - Transaction history

- **CLI Interface**
  - Interactive wallet initialization
  - Network selection (Bitcoin, Testnet, Regtest)
  - Complete command-line interface
  - Verbose logging support

- **Storage Layer**
  - SQLite-based persistence
  - Payment and channel tracking
  - Thread-safe operations
  - Automatic schema management

- **Development Infrastructure**
  - Comprehensive CI/CD with GitHub Actions
  - Multi-platform release builds (Linux, macOS, Windows)
  - Automated testing and linting
  - Security audit integration

### 🚧 In Development
- **Lightning Network (LDK)**
  - Channel management
  - Lightning payments (send/receive)
  - Invoice creation and payment
  - Node connectivity

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/chaitanya21kumar/unified-lightning-wallet.git
cd unified-lightning-wallet

# Build in release mode
cargo build --release

# The binary will be at target/release/ulw
./target/release/ulw --help
```

### From GitHub Releases

Download pre-built binaries for your platform from the [Releases](https://github.com/chaitanya21kumar/unified-lightning-wallet/releases) page.

## 🎯 Quick Start

### 1. Initialize Wallet

```bash
# Initialize a new wallet
ulw init

# Follow the interactive prompts to:
# - Select network (Bitcoin/Testnet/Regtest)
# - Configure Electrum server
# - Set Lightning port
```

### 2. Generate Receiving Address

```bash
# Get a new Bitcoin address
ulw receive
```

### 3. Check Balance

```bash
# View wallet balance
ulw balance
```

### 4. Send Bitcoin

```bash
# Send Bitcoin to an address
ulw send <address> <amount_in_sats>

# Example:
ulw send bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh 10000
```

### 5. Sync with Blockchain

```bash
# Sync wallet state with blockchain
ulw sync
```

### 6. View Transactions

```bash
# List recent transactions
ulw transactions

# Limit number of transactions
ulw transactions --limit 5
```

## 📖 Usage

### Commands

```
ulw <COMMAND>

Commands:
  init          Initialize a new wallet
  balance       Get wallet balance
  receive       Generate a new receiving address
  send          Send on-chain payment
  sync          Sync wallet with blockchain
  transactions  List transactions
  channels      Manage Lightning channels
  invoice       Create a Lightning invoice
  pay           Pay a Lightning invoice
  help          Print help information
```

### Lightning Commands (Coming Soon)

```bash
# List Lightning channels
ulw channels list

# Open a Lightning channel
ulw channels open <node_id> <amount_sats>

# Close a channel
ulw channels close <channel_id>

# Create an invoice
ulw invoice <amount_sats> --description "Payment for services"

# Pay an invoice
ulw pay <bolt11_invoice>
```

### Global Options

```bash
-v, --verbose    Enable verbose logging
-h, --help       Print help
-V, --version    Print version
```

## 🏗️ Architecture

### Project Structure

```
unified-lightning-wallet/
├── crates/
│   ├── core/              # Core domain types and traits
│   ├── bdk-integration/   # On-chain wallet (BDK)
│   ├── ldk-integration/   # Lightning Network (LDK)
│   ├── storage/           # SQLite persistence layer
│   ├── sync/              # Synchronization protocol
│   └── cli/               # Command-line interface
├── tests/                 # Integration tests
├── docs/                  # Documentation
└── examples/              # Usage examples
```

### Key Components

#### Core (`ulw-core`)
- Domain types (Balance, Payment, Channel, etc.)
- Error handling
- Trait definitions for wallet components

#### BDK Integration (`ulw-bdk`)
- Bitcoin on-chain wallet
- PSBT transaction building
- Electrum client integration
- Address management

#### LDK Integration (`ulw-ldk`)
- Lightning node implementation
- Channel management
- Payment routing
- Invoice handling

#### Storage (`ulw-storage`)
- SQLite database
- Payment persistence
- Channel state tracking
- Thread-safe operations

#### CLI (`ulw-cli`)
- User interface
- Command handling
- Configuration management
- Interactive setup

## 🔧 Development

### Prerequisites

- Rust 1.75 or higher
- SQLite 3
- (Optional) Bitcoin Core for regtest

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test --all

# Run lints
cargo clippy --all-features --workspace

# Format code
cargo fmt --all
```

### Running in Regtest Mode

```bash
# Initialize wallet for regtest
ulw init
# Select "Regtest" when prompted

# The default Electrum URL for regtest is tcp://localhost:50001
# Make sure you have an Electrum server running on regtest
```

## 🧪 Testing

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p ulw-core
cargo test -p ulw-bdk
cargo test -p ulw-storage

# Run with verbose output
cargo test --all -- --nocapture
```

## 📝 Configuration

Wallet configuration is stored in `~/.ulw/config.json`:

```json
{
  "data_dir": "/Users/username/.ulw",
  "network": {
    "network": "Regtest",
    "electrum_url": "tcp://localhost:50001",
    "lightning_port": 9735
  },
  "wallet_name": "default"
}
```

## 🔒 Security

- **Self-Custodial**: You control your private keys
- **No External Dependencies**: Minimal attack surface
- **Secure Storage**: SQLite with proper permissions
- **BIP39 Mnemonics**: Industry-standard seed phrases
- **HD Wallets**: Hierarchical deterministic key derivation

### Important Security Notes

⚠️ **This is development software. Do NOT use on Bitcoin mainnet with real funds.**

- Keep your mnemonic seed phrase secure and backed up
- Never share your seed phrase or private keys
- Test thoroughly on testnet/regtest before considering mainnet use
- Audit the code yourself before trusting it with funds

## 🤝 Contributing

Contributions are welcome! This project is being developed for **Summer of Bitcoin 2026**.

### Development Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and lints
5. Submit a pull request

### Code Style

- Follow Rust conventions
- Use `cargo fmt` for formatting
- Pass `cargo clippy` without warnings
- Add tests for new functionality
- Update documentation

## 📊 Project Status

### Completion Status

- [x] Project structure and workspace setup
- [x] Core domain types and error handling
- [x] BDK integration for on-chain wallet
- [x] CLI interface with all major commands
- [x] SQLite storage layer
- [x] CI/CD pipeline
- [ ] LDK Lightning Network integration (in progress)
- [ ] Comprehensive test suite
- [ ] Docker deployment
- [ ] Complete documentation

## 🗺️ Roadmap

### Phase 1: Core Functionality (Current)
- ✅ BDK on-chain wallet
- ✅ CLI interface
- ✅ Storage layer
- 🚧 LDK Lightning integration

### Phase 2: Enhanced Features
- Channel backup and recovery
- Multi-signature support
- Hardware wallet integration
- Mobile app (Flutter/React Native)

### Phase 3: Advanced Features
- Submarine swaps
- Lightning Service Provider (LSP) integration
- Watchtower support
- Advanced privacy features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Built with:
- [BDK](https://bitcoindevkit.org/) - Bitcoin Development Kit
- [LDK](https://lightningdevkit.org/) - Lightning Development Kit
- [Rust](https://www.rust-lang.org/) - Systems programming language

## 📞 Contact

- **Author**: Chaitanya Kumar
- **GitHub**: [@chaitanya21kumar](https://github.com/chaitanya21kumar)
- **Project**: [Unified Lightning Wallet](https://github.com/chaitanya21kumar/unified-lightning-wallet)

---

**⚡ Built for Summer of Bitcoin 2026**

**⚠️ Educational/Development Software - Not for production use with real funds**
