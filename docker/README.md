# Docker Deployment Guide

This directory contains Docker configuration for running Unified Lightning Wallet in a containerized environment.

## Quick Start

### Build and Run

```bash
# Build the Docker image
docker build -t unified-lightning-wallet -f docker/Dockerfile .

# Run the wallet
docker run --rm unified-lightning-wallet --help

# Run with volume for persistent data
docker run --rm -v ulw-data:/home/ulw/.ulw unified-lightning-wallet init
```

### Using Docker Compose

The `docker-compose.yml` includes:
- ULW wallet
- Bitcoin Core (regtest)
- Electrum server

```bash
# Start all services
docker-compose -f docker/docker-compose.yml up -d

# Initialize wallet
docker-compose -f docker/docker-compose.yml exec ulw ulw init

# Check balance
docker-compose -f docker/docker-compose.yml exec ulw ulw balance

# Generate address
docker-compose -f docker/docker-compose.yml exec ulw ulw receive

# Stop all services
docker-compose -f docker/docker-compose.yml down
```

## Configuration

### Environment Variables

- `RUST_LOG`: Logging level (debug, info, warn, error)
- `ULW_NETWORK`: Bitcoin network (bitcoin, testnet, regtest)

### Volumes

- `ulw-data`: Wallet data and configuration (`~/.ulw`)
- `bitcoin-data`: Bitcoin Core data (optional)

### Ports

- `9735`: Lightning Network port
- `18443`: Bitcoin Core RPC (regtest)
- `50001/50002`: Electrum server

## Development

### Building

```bash
# Build with docker-compose
docker-compose -f docker/docker-compose.yml build

# Build specific service
docker-compose -f docker/docker-compose.yml build ulw
```

### Testing

```bash
# Run tests in container
docker run --rm unified-lightning-wallet cargo test --all
```

## Production Notes

⚠️ **This is development software. Do NOT use in production.**

For production deployment:
1. Use specific image tags, not `latest`
2. Properly secure RPC credentials
3. Use SSL/TLS for Electrum connections
4. Implement proper backup strategies
5. Monitor logs and metrics
6. Use hardware security modules (HSM) for keys

## Troubleshooting

### Check logs

```bash
# View wallet logs
docker-compose -f docker/docker-compose.yml logs ulw

# View Bitcoin Core logs
docker-compose -f docker/docker-compose.yml logs bitcoind

# Follow logs
docker-compose -f docker/docker-compose.yml logs -f
```

### Reset data

```bash
# Stop services
docker-compose -f docker/docker-compose.yml down

# Remove volumes
docker volume rm docker_ulw-data docker_bitcoin-data

# Start fresh
docker-compose -f docker/docker-compose.yml up -d
```

### Access container shell

```bash
# Access ulw container
docker-compose -f docker/docker-compose.yml exec ulw /bin/bash

# Access as root
docker-compose -f docker/docker-compose.yml exec -u root ulw /bin/bash
```

## Architecture

```
┌─────────────────┐
│   ULW Wallet    │ ← User Interface
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│ Electrum Server │ ← Blockchain Data
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  Bitcoin Core   │ ← Full Node (Regtest)
└─────────────────┘
```

## Security

- Wallet runs as non-root user (`ulw`)
- Minimal runtime dependencies
- Data persisted in Docker volumes
- Network isolation with bridge network
- No external network access required for regtest

## Support

For issues related to Docker deployment, please open an issue at:
https://github.com/chaitanya21kumar/unified-lightning-wallet/issues
