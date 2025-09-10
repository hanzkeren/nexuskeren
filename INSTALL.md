# Nexus CLI - Installation Guide

Quick installation guide for the Nexus CLI with parallel execution support.

## 🚀 One-Line Installation

```bash
git clone https://github.com/your-username/nexus-cli.git && cd nexus-cli && ./build-parallel.sh --install
```

## 📋 Prerequisites

### System Requirements
- **OS**: Linux (Ubuntu/Debian recommended), macOS, or Windows WSL
- **CPU**: Multi-core processor (4+ cores recommended)
- **Memory**: 8GB RAM minimum, 16GB+ recommended
- **Storage**: 2GB free space
- **Network**: Stable internet connection

### Required Software
- **Rust**: 1.88.0+ (nightly recommended)
- **Git**: For cloning the repository
- **Build tools**: GCC, pkg-config, OpenSSL

## 🛠 Detailed Installation

### Step 1: Install Rust

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Step 2: Install System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev git
```

**CentOS/RHEL/Fedora:**
```bash
sudo yum install -y gcc openssl-devel pkg-config git
# Or for newer versions:
sudo dnf install -y gcc openssl-devel pkg-config git
```

**macOS:**
```bash
# Install Xcode command line tools
xcode-select --install

# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install openssl pkg-config
```

### Step 3: Clone and Build

```bash
# Clone the repository
git clone https://github.com/your-username/nexus-cli.git
cd nexus-cli

# Run the automated build script
./build-parallel.sh --install
```

### Step 4: Verify Installation

```bash
# Check if nexus is installed globally
nexus --help

# Or use the local binary
./clients/cli/target/release/nexus-network --help
```

## 🐳 Docker Installation (Alternative)

If you prefer Docker:

```bash
# Clone repository
git clone https://github.com/your-username/nexus-cli.git
cd nexus-cli

# Build Docker image
docker build -t nexus-cli .

# Run with Docker
docker run -it nexus-cli nexus start --node-id YOUR_NODE_ID --headless
```

## 🖥 VPS/Server Installation

### Quick VPS Setup Script

```bash
#!/bin/bash
# Save as install-nexus.sh and run: chmod +x install-nexus.sh && ./install-nexus.sh

# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential pkg-config libssl-dev git curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Clone and build Nexus CLI
git clone https://github.com/your-username/nexus-cli.git
cd nexus-cli
./build-parallel.sh --install

echo "✅ Installation complete!"
echo "Run: nexus start --node-id YOUR_NODE_ID --headless"
```

### Production Systemd Service

Create `/etc/systemd/system/nexus.service`:

```ini
[Unit]
Description=Nexus CLI Parallel Runner
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=nexus
Group=nexus
WorkingDirectory=/home/nexus
ExecStart=/home/nexus/.cargo/bin/nexus start --node-id YOUR_NODE_ID --headless --max-threads 4 --max-tasks 1000
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/home/nexus

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable nexus
sudo systemctl start nexus
```

## 🚦 First Run

### Interactive Mode
```bash
nexus start --node-id YOUR_NODE_ID
```

### Headless Mode (VPS/Server)
```bash
nexus start --node-id YOUR_NODE_ID --headless --max-threads 4
```

### Test Run (Limited Tasks)
```bash
nexus start --node-id YOUR_NODE_ID --max-tasks 10 --max-difficulty SMALL
```

## 🐛 Troubleshooting

### Common Issues

**1. "command not found: nexus"**
```bash
# Check if ~/.cargo/bin is in PATH
echo $PATH | grep cargo

# Add to PATH if missing
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**2. Build fails with "linker not found"**
```bash
# Install build essentials
sudo apt install -y build-essential

# Or on macOS
xcode-select --install
```

**3. OpenSSL errors**
```bash
# Ubuntu/Debian
sudo apt install -y libssl-dev pkg-config

# CentOS/RHEL
sudo yum install -y openssl-devel

# macOS
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
```

**4. Permission denied**
```bash
# Fix cargo permissions
sudo chown -R $USER:$USER ~/.cargo
```

### Performance Issues

**Low performance:**
- Increase `--max-threads` (up to CPU core count)
- Use higher `--max-difficulty`
- Check system resources: `htop`

**High memory usage:**
- Reduce `--max-threads`
- Lower `--max-difficulty`
- Monitor with: `watch -n 1 free -h`

## 📊 Quick Performance Test

```bash
# 5-minute test run
timeout 300 nexus start --node-id YOUR_NODE_ID --headless --max-threads 2

# Check system resources during test
htop  # In another terminal
```

## 🔄 Updates

```bash
cd nexus-cli
git pull origin main
./build-parallel.sh --install
```

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-username/nexus-cli/issues)
- **Documentation**: [README.md](README.md)
- **Performance Guide**: [PARALLEL_IMPLEMENTATION.md](PARALLEL_IMPLEMENTATION.md)

---

**Quick Commands Reference:**
```bash
# Basic start
nexus start --node-id YOUR_NODE_ID

# Production (headless)
nexus start --node-id YOUR_NODE_ID --headless --max-threads 4

# Test run
nexus start --node-id YOUR_NODE_ID --max-tasks 10

# High performance
nexus start --node-id YOUR_NODE_ID --max-difficulty LARGE --max-threads 8
```
