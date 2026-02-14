# Unified Lightning Wallet - User Guide

**Version**: 1.0.0
**Last Updated**: 2026-02-14
**Status**: Educational/Development Software

⚠️ **IMPORTANT**: This is development/educational software. Do NOT use on Bitcoin mainnet with real funds.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Installation](#installation)
3. [CLI Usage](#cli-usage)
4. [Desktop GUI Usage](#desktop-gui-usage)
5. [Web Demo](#web-demo)
6. [On-Chain Operations](#on-chain-operations)
7. [Lightning Network Operations](#lightning-network-operations)
8. [Configuration](#configuration)
9. [Troubleshooting](#troubleshooting)
10. [FAQ](#faq)

---

## Getting Started

### What is Unified Lightning Wallet?

Unified Lightning Wallet (ULW) is a self-custodial Bitcoin wallet that supports both:
- **On-Chain** transactions (regular Bitcoin transactions)
- **Lightning Network** transactions (instant, low-fee payments)

### Choose Your Interface

ULW is available in three formats:

| Interface | Best For | Installation |
|-----------|----------|--------------|
| 🖥️ **Desktop GUI** | Most users, visual interface | Download installer |
| 💻 **CLI** | Advanced users, automation | Build from source |
| 🌐 **Web Demo** | Quick preview, no install | Visit website |

### System Requirements

- **Operating System**: macOS 11+, Ubuntu 20.04+, Windows 10+
- **Disk Space**: 100 MB for application, 50 GB+ for full node (optional)
- **RAM**: 2 GB minimum, 4 GB recommended
- **Network**: Internet connection for blockchain sync

---

## Installation

### Desktop GUI (Recommended for Most Users)

#### macOS
```bash
# Download .dmg file from GitHub Releases
curl -L https://github.com/chaitanya21kumar/unified-lightning-wallet/releases/download/v1.0.0/ULW-macOS.dmg -o ULW.dmg

# Open and drag to Applications
open ULW.dmg
```

#### Linux
```bash
# Ubuntu/Debian (.deb)
wget https://github.com/chaitanya21kumar/unified-lightning-wallet/releases/download/v1.0.0/ulw_1.0.0_amd64.deb
sudo dpkg -i ulw_1.0.0_amd64.deb

# Other Linux (.AppImage)
wget https://github.com/chaitanya21kumar/unified-lightning-wallet/releases/download/v1.0.0/ULW-Linux.AppImage
chmod +x ULW-Linux.AppImage
./ULW-Linux.AppImage
```

#### Windows
```powershell
# Download .exe installer
# Visit: https://github.com/chaitanya21kumar/unified-lightning-wallet/releases/download/v1.0.0/ULW-Setup.exe
# Run the installer
```

### CLI (Command Line)

#### From Source (All Platforms)
```bash
# Prerequisites: Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/chaitanya21kumar/unified-lightning-wallet.git
cd unified-lightning-wallet

# Build release version
cargo build --release

# Binary location
./target/release/ulw --version

# Optional: Install globally
cargo install --path crates/cli
```

#### From Binary (Linux/macOS)
```bash
# Download pre-built binary
curl -L https://github.com/chaitanya21kumar/unified-lightning-wallet/releases/download/v1.0.0/ulw-$(uname -s)-$(un ame -m) -o ulw

# Make executable
chmod +x ulw

# Move to PATH
sudo mv ulw /usr/local/bin/

# Verify installation
ulw --version
```

---

## CLI Usage

### First-Time Setup

#### 1. Initialize Wallet

```bash
ulw init
```

You'll be prompted for:
- **Network**: Choose Bitcoin, Testnet, or Regtest (use Testnet for learning)
- **Electrum Server**: Default provided, or enter custom
- **Lightning Port**: Default 9735 recommended

Example output:
```
🎉 Welcome to Unified Lightning Wallet!

Select network:
  1) Bitcoin (Mainnet) ⚠️  Real money!
  2) Testnet (Recommended for learning)
  3) Regtest (Local development)

Your choice: 2

Electrum server URL [tcp://electrum.blockstream.info:60002]:
Lightning port [9735]:

✅ Wallet initialized successfully!
Configuration saved to: /Users/you/.ulw/config.json

⚠️  IMPORTANT: Save your recovery phrase securely!
```

#### 2. Backup Your Recovery Phrase

**CRITICAL**: Write down your 12/24-word recovery phrase and store it securely.

```bash
# Display recovery phrase (if using proper mnemonic)
# Future feature - current version uses descriptors
```

### Basic On-Chain Operations

#### Check Balance

```bash
ulw balance
```

Output:
```
💰 Wallet Balance
  Total: 50000 sats
```

#### Receive Bitcoin

```bash
ulw receive
```

Output:
```
📬 New Receiving Address
tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh

💡 Share this address to receive Bitcoin
```

**Tips**:
- Each time you run `receive`, a new address is generated
- This improves privacy (address reuse is discouraged)
- All addresses are derived from your recovery phrase

#### Send Bitcoin

```bash
ulw send <address> <amount_in_sats>
```

Example:
```bash
ulw send tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx 10000
```

Output:
```
📤 Sending 10000 satoshis to tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx
✅ Transaction broadcast!
TXID: a1b2c3d4e5f6789012345678901234567890123456789012345678901234567
```

**Fee Estimation**: Fees are automatically calculated based on network conditions.

#### Sync Wallet

```bash
ulw sync
```

Output:
```
🔄 Syncing wallet with blockchain...
✅ Sync complete!
```

**When to Sync**:
- After receiving a transaction
- Before sending (to update balance)
- Periodically (every few hours)

#### View Transaction History

```bash
ulw transactions

# Show only last 5
ulw transactions --limit 5
```

Output:
```
📜 Recent Transactions (showing up to 10)

1. TXID: a1b2c3d4e5f6...
   Received: 50000 sats
   Sent: 0 sats
   Fee: 0 sats
   Confirmed: 2024-01-15 14:23:00

2. TXID: 7890abcdef12...
   Received: 0 sats
   Sent: 10000 sats
   Fee: 150 sats
   Status: Unconfirmed
```

### Lightning Network Operations

#### Create Invoice (Receive Payment)

```bash
ulw invoice <amount_in_sats> -d "Description"
```

Example:
```bash
ulw invoice 1000 -d "Coffee payment"
```

Output:
```
⚡ Creating Lightning Invoice

✅ Invoice created!
Amount: 1000 sats (1000000 msats)
Node ID: 03a1b2c3d4e5f6...

Invoice:
lnbcrt10u1pjkl2m3pp5abc123...

💡 Share this invoice to receive payment
```

**Invoice Details**:
- Valid for 1 hour (3600 seconds)
- Contains payment hash for tracking
- Can be paid by any Lightning wallet

#### Pay Invoice (Send Payment)

```bash
ulw pay <bolt11_invoice>
```

Example:
```bash
ulw pay lnbcrt10u1pjkl2m3pp5abc123...
```

Output:
```
⚡ Paying Lightning Invoice
Parsing invoice...

✅ Payment initiated!
Payment Hash: abc123def456...

⚠️  Note: Full payment routing requires active channels
   This is a simplified implementation for testing
```

#### View Lightning Node Info

```bash
# Check your Lightning node ID
# (Add this command if not already in CLI)
```

### Advanced CLI Usage

#### Verbose Logging

```bash
ulw --verbose <command>

# Or set environment variable
RUST_LOG=debug ulw <command>
```

#### Custom Configuration

Edit `~/.ulw/config.json`:
```json
{
  "data_dir": "/Users/you/.ulw",
  "network": {
    "network": "Testnet",
    "electrum_url": "tcp://electrum.blockstream.info:60002",
    "lightning_port": 9735
  },
  "wallet_name": "default"
}
```

---

## Desktop GUI Usage

### First Launch

1. **Open Application**: Double-click the ULW icon
2. **Initialize Wallet**:
   - Click "Create New Wallet"
   - Select network (Testnet recommended)
   - Set Electrum server
   - Save recovery phrase ⚠️

### Main Interface

The desktop GUI has three main tabs:

#### 1. On-Chain Tab

**Balance Display**:
- Total balance in BTC and sats
- Pending balance (unconfirmed)

**Receive**:
1. Click "Receive" button
2. Copy address or scan QR code
3. Share with sender

**Send**:
1. Click "Send" button
2. Enter recipient address
3. Enter amount (sats or BTC)
4. Click "Send" - confirm transaction
5. Wait for broadcast confirmation

**Transaction History**:
- Scroll to view past transactions
- Click transaction for details
- Filter by date/amount

#### 2. Lightning Tab

**Create Invoice**:
1. Enter amount in sats
2. Add description (optional)
3. Click "Create Invoice"
4. Share invoice string or QR code

**Pay Invoice**:
1. Paste BOLT11 invoice
2. Review amount and description
3. Click "Pay"
4. Confirm payment

**Payment History**:
- View sent and received Lightning payments
- Payment status (pending, succeeded, failed)
- Payment hashes for tracking

#### 3. Settings Tab

**Network Settings**:
- Switch between Bitcoin/Testnet/Regtest
- Change Electrum server
- Configure Lightning port

**Wallet Settings**:
- View wallet name
- Data directory location
- Export transaction history (CSV)

**About**:
- Version information
- License details
- GitHub repository link

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl + R` | Refresh balance |
| `Cmd/Ctrl + N` | New receiving address |
| `Cmd/Ctrl + S` | Send transaction |
| `Cmd/Ctrl + ,` | Open settings |
| `Cmd/Ctrl + Q` | Quit application |

---

## Web Demo

### Accessing the Demo

Visit: [https://unified-lightning-wallet.vercel.app](https://unified-lightning-wallet.vercel.app)

### Features

- **UI Preview**: See the wallet interface
- **Feature Overview**: Learn about capabilities
- **Download Links**: Get desktop and CLI versions
- **Safety Warnings**: Testnet-only reminders

### Limitations

⚠️ **The web demo is for demonstration purposes only**:
- No actual wallet functionality in browser
- Cannot send/receive real Bitcoin
- Links to full desktop and CLI applications
- Used for showcasing and education

---

## On-Chain Operations

### Understanding Bitcoin Addresses

ULW uses **Native SegWit (bech32)** addresses:
- Start with `bc1` (mainnet) or `tb1` (testnet)
- Lowest transaction fees
- Better privacy and efficiency

Example addresses:
```
Mainnet: bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh
Testnet: tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh
```

### Transaction Fees

**How Fees Work**:
- Fees measured in sats/vByte (satoshis per virtual byte)
- Higher fee = faster confirmation
- ULW automatically estimates appropriate fee

**Fee Levels** (approximate):
- **Low Priority**: 1-3 sats/vByte (~30-60 min)
- **Medium Priority**: 5-10 sats/vByte (~10-30 min)
- **High Priority**: 15+ sats/vByte (~0-10 min)

### Transaction Confirmations

**Confirmation Timeline**:
1. **0 confirmations**: Broadcast to network (unconfirmed)
2. **1 confirmation**: Included in a block (~10 min)
3. **3 confirmations**: Moderately safe (~30 min)
4. **6 confirmations**: Very safe (~60 min)

**Recommendation**: Wait for 3-6 confirmations for significant amounts.

### UTXO Management

**What are UTXOs?**:
- Unspent Transaction Outputs
- "Coins" in your wallet
- ULW automatically manages UTXOs

**Coin Selection**:
- ULW selects optimal UTXOs for each transaction
- Minimizes fees
- Consolidates small UTXOs when beneficial

---

## Lightning Network Operations

### Understanding Lightning

**Benefits**:
- ⚡ Instant payments
- 💸 Very low fees (< 1 sat typically)
- 🔒 Enhanced privacy
- 🌐 Perfect for small, frequent payments

**Limitations**:
- Requires online presence
- Channel management complexity
- Best for amounts < $100

### Lightning Invoices

**Invoice Format**: BOLT11 encoded string
```
lnbc10u1pjkl2m3pp5abc123def456...
```

**Invoice Components**:
- `lnbc`: Lightning mainnet
- `lnbcrt`: Lightning regtest/testnet
- `10u`: Amount (10 micro-BTC = 1000 sats)
- Rest: Payment hash, signature, metadata

**Creating Good Invoices**:
- Add clear descriptions
- Use reasonable expiry times (1 hour default)
- Include your node ID for reference

### Payment States

| State | Meaning | Action |
|-------|---------|--------|
| **Pending** | Payment in progress | Wait |
| **Succeeded** | Payment completed | None |
| **Failed** |  Payment unsuccessful | Retry or contact recipient |

### Lightning Security

**Best Practices**:
- Don't store large amounts in Lightning channels
- Keep node online for receiving payments
- Backup channel state frequently
- Use watchtowers (future feature)

---

## Configuration

### Configuration File

Location: `~/.ulw/config.json`

```json
{
  "data_dir": "/Users/you/.ulw",
  "network": {
    "network": "Testnet",
    "electrum_url": "tcp://electrum.blockstream.info:60002",
    "lightning_port": 9735
  },
  "wallet_name": "default"
}
```

### Network Options

#### Bitcoin (Mainnet)
```json
{
  "network": "Bitcoin",
  "electrum_url": "ssl://electrum.blockstream.info:50002",
  "lightning_port": 9735
}
```

⚠️ **Use with caution**: Real money at stake!

#### Testnet (Recommended for Learning)
```json
{
  "network": "Testnet",
  "electrum_url": "tcp://electrum.blockstream.info:60002",
  "lightning_port": 9735
}
```

✅ **Perfect for learning**: No real money, safe to experiment.

Get testnet coins: https://testnet-faucet.mempool.co/

#### Regtest (Local Development)
```json
{
  "network": "Regtest",
  "electrum_url": "tcp://localhost:50001",
  "lightning_port": 9735
}
```

🔧 **For developers**: Requires local Bitcoin node.

### Custom Electrum Servers

**Public Servers**:
- Blockstream: `electrum.blockstream.info`
- Mempool: `electrum.blockstream.info`

**Run Your Own** (advanced):
```bash
# Install Electrs
git clone https://github.com/romanz/electrs
cd electrs
cargo build --release

# Run server
./target/release/electrs --network testnet
```

---

## Troubleshooting

### Common Issues

#### "Wallet not initialized"
```
Error: Wallet not initialized. Run 'ulw init' first.
```

**Solution**: Run `ulw init` to create wallet configuration.

#### "Connection refused"
```
Error: Failed to connect to Electrum server
```

**Solutions**:
1. Check internet connection
2. Try different Electrum server
3. Check firewall settings
4. Use `--verbose` flag for detailed error

#### "Insufficient funds"
```
Error: Insufficient funds: required 10000, available 5000
```

**Solutions**:
1. Check balance: `ulw balance`
2. Sync wallet: `ulw sync`
3. Wait for pending transactions to confirm
4. Get testnet coins from faucet

#### "Invalid address"
```
Error: Invalid address format
```

**Solutions**:
1. Verify address format (bc1/tb1 for Native SegWit)
2. Check network matches (testnet address on testnet)
3. Copy address carefully (no extra spaces)

### Logging and Debugging

**Enable Verbose Logging**:
```bash
# CLI
ulw --verbose <command>

# Or environment variable
RUST_LOG=debug ulw <command>

# Maximum verbosity
RUST_LOG=trace ulw <command>
```

**Log File Locations**:
- CLI: stdout/stderr
- Desktop GUI: `~/.ulw/logs/` (future feature)

### Network Issues

**Check Electrum Connection**:
```bash
# Testnet
telnet electrum.blockstream.info 60002

# Should connect successfully
```

**Alternative Servers**:
If default Electrum server is down, try:
- `tcp://electrum.blockstream.info:60002` (testnet)
- `ssl://electrum.blockstream.info:50002` (mainnet)

### Data Recovery

**Backup Wallet Data**:
```bash
# Backup entire data directory
cp -r ~/.ulw ~/.ulw.backup

# Backup configuration only
cp ~/.ulw/config.json ~/ulw-config-backup.json
```

**Restore from Backup**:
```bash
# Restore entire data directory
rm -rf ~/.ulw
cp -r ~/.ulw.backup ~/.ulw

# Restore configuration only
cp ~/ulw-config-backup.json ~/.ulw/config.json
```

**Future**: Mnemonic-based recovery (restore wallet from 12/24 words)

---

## FAQ

### General Questions

**Q: Is Unified Lightning Wallet safe to use?**
A: Currently, ULW is development/educational software. Do NOT use on mainnet with real funds. Use testnet for learning.

**Q: What networks are supported?**
A: Bitcoin (mainnet), Testnet, and Regtest. Testnet is recommended for learning.

**Q: Can I use this on mobile?**
A: Not yet. Mobile apps (iOS/Android) are planned for future releases.

**Q: Is my data encrypted?**
A: Database is stored locally with file system permissions. Full encryption (SQLCipher) planned for future releases.

### On-Chain Questions

**Q: How long do Bitcoin transactions take?**
A: Typically 10-60 minutes depending on network congestion and fees paid. See ~6 confirmations for safety.

**Q: What are the transaction fees?**
A: Fees vary based on network activity. ULW automatically estimates appropriate fees (typically 150-500 sats per transaction).

**Q: Can I cancel a pending transaction?**
A: No, Bitcoin transactions cannot be cancelled once broadcast. Future: Replace-By-Fee (RBF) support.

**Q: Why can't I send my entire balance?**
A: Transaction fees are deducted from your balance. Fee typically 150-500 sats.

### Lightning Questions

**Q: What is the Lightning Network?**
A: A second-layer Bitcoin protocol for instant, low-fee transactions. It uses payment channels.

**Q: Do I need channels to use Lightning?**
A: Yes, but ULW's current implementation is simplified for testing. Full channel management coming soon.

**Q: How much can I send via Lightning?**
A: Currently limited by invoice amounts. With channels, typically $1-$100 per payment.

**Q: Can I receive Lightning payments while offline?**
A: No, Lightning requires both parties to be online. Future: Asynchronous Lightning protocols.

### Technical Questions

**Q: What is BDK and LDK?**
A: BDK (Bitcoin Development Kit) handles on-chain operations. LDK (Lightning Development Kit) handles Lightning. Both are Rust libraries.

**Q: Can I run my own Bitcoin node?**
A: Yes, you can connect to your own Electrum server (indexer for Bitcoin Core).

**Q: How is my data stored?**
A: SQLite database in `~/.ulw/`. Keys are managed by BDK and LDK.

**Q: Can I export my transaction history?**
A: Yes, via CLI or GUI (future feature: CSV export).

---

## Getting Help

### Resources

- **Documentation**: [github.com/chaitanya21kumar/unified-lightning-wallet/docs](https://github.com/chaitanya21kumar/unified-lightning-wallet/tree/main/docs)
- **GitHub Issues**: [github.com/chaitanya21kumar/unified-lightning-wallet/issues](https://github.com/chaitanya21kumar/unified-lightning-wallet/issues)
- **Architecture Guide**: See `ARCHITECTURE.md`

### Reporting Bugs

When reporting bugs, include:
1. ULW version (`ulw --version`)
2. Operating system
3. Network (Bitcoin/Testnet/Regtest)
4. Steps to reproduce
5. Error messages (with `--verbose`)
6. Expected vs actual behavior

### Contributing

Contributions welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

---

## Safety Reminders

⚠️ **CRITICAL SAFETY INFORMATION**

1. **Development Software**: Do NOT use on Bitcoin mainnet with real funds
2. **Backup Recovery Phrase**: Write down 12/24 words and store securely
3. **Start with Testnet**: Learn on testnet before considering mainnet
4. **Small Amounts**: If testing on mainnet (not recommended), use tiny amounts
5. **No Warranty**: Software provided as-is, use at your own risk
6. **Security Audit**: Not yet audited, not production-ready

**Use testnet for learning. Use caution if testing on mainnet.**

---

## Next Steps

### For New Users
1. ✅ Install ULW (Desktop GUI or CLI)
2. ✅ Initialize wallet on Testnet
3. ✅ Get testnet coins from faucet
4. ✅ Practice receiving and sending
5. ✅ Try Lightning invoice creation
6. ✅ Read ARCHITECTURE.md for technical details

### For Developers
1. ✅ Read ARCHITECTURE.md
2. ✅ Build from source: `cargo build`
3. ✅ Run tests: `cargo test --all`
4. ✅ Explore codebase structure
5. ✅ Contribute improvements

### For Summer of Bitcoin 2026
This wallet is being developed as part of Summer of Bitcoin 2026. Features demonstrated:
- Modern Rust architecture
- BDK and LDK integration
- Multi-platform support (CLI, GUI, Web)
- Comprehensive documentation
- Production-ready code structure

---

**Thank you for using Unified Lightning Wallet!** 🚀⚡

**Last Updated**: 2026-02-14
**Version**: 1.0.0
**License**: MIT
