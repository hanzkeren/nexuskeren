# Nexus CLI - Parallel Execution Edition

Enhanced Nexus CLI with parallel execution support for maximum throughput on multi-core systems.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/your-username/nexus-cli)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#license)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange)](https://www.rust-lang.org/)

## 🚀 Features

- **🔥 Parallel Execution**: Utilize all CPU cores for maximum throughput
- **🧠 Smart Resource Management**: Automatic CPU and memory detection
- **⚡ Load Balancing**: Distribute tasks efficiently across workers
- **🎯 Task Control**: Limit tasks, difficulty levels, and execution time
- **📊 Real-time Monitoring**: Live dashboard with performance metrics
- **🔧 Flexible Configuration**: Customize worker count, task limits, and more

## 📋 System Requirements

- **OS**: Linux, macOS, Windows
- **CPU**: Multi-core recommended (automatic detection)
- **Memory**: 4GB+ RAM (8GB+ recommended for optimal performance)
- **Rust**: 1.88.0+ (nightly recommended)
- **Network**: Stable internet connection

## 🛠 Installation

### Quick Install (Recommended)

```bash
# Clone the repository
git clone https://github.com/your-username/nexus-cli.git
cd nexus-cli

# Run the automated build script
./build-parallel.sh --install
```

### Manual Installation

```bash
# Clone and build manually
git clone https://github.com/your-username/nexus-cli.git
cd nexus-cli/clients/cli

# Build in release mode
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Docker Installation

```bash
# Build Docker image
docker build -t nexus-cli .

# Run in container
docker run -it nexus-cli nexus start --headless
```

## 🎮 Quick Start

### Basic Usage

```bash
# Start with automatic CPU detection (uses all cores)
nexus start --node-id YOUR_NODE_ID

# Start with custom thread count
nexus start --node-id YOUR_NODE_ID --max-threads 4

# Run in headless mode (no UI)
nexus start --node-id YOUR_NODE_ID --headless
```

### Task Control

```bash
# Limit total tasks
nexus start --node-id YOUR_NODE_ID --max-tasks 100

# Set difficulty level
nexus start --node-id YOUR_NODE_ID --max-difficulty MEDIUM

# Combined configuration
nexus start --node-id YOUR_NODE_ID --max-tasks 500 --max-difficulty LARGE --max-threads 8
```

## 📖 Usage Guide

### Command Line Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--node-id` | Your unique node identifier | Required | `--node-id 36064546` |
| `--max-threads` | Maximum worker threads | All CPU cores | `--max-threads 4` |
| `--max-tasks` | Maximum tasks to process | Unlimited | `--max-tasks 100` |
| `--max-difficulty` | Maximum task difficulty | SMALL_MEDIUM | `--max-difficulty LARGE` |
| `--headless` | Run without UI | false | `--headless` |
| `--with-background` | Enable background colors | false | `--with-background` |

### Difficulty Levels

| Level | Description | CPU Usage | Recommended For |
|-------|-------------|-----------|-----------------|
| `SMALL` | Easiest tasks | Low | Testing, low-end hardware |
| `SMALL_MEDIUM` | Default difficulty | Medium | Most users |
| `MEDIUM` | Moderate difficulty | Medium-High | Good hardware |
| `LARGE` | Hard tasks | High | High-end systems |
| `EXTRA_LARGE` | Hardest tasks | Very High | Server-grade hardware |

### Performance Optimization

#### CPU Core Recommendations

```bash
# For 4-core systems
nexus start --node-id YOUR_NODE_ID --max-threads 3  # Leave 1 core for system

# For 8-core systems  
nexus start --node-id YOUR_NODE_ID --max-threads 6  # Leave 2 cores for system

# For 12+ core systems
nexus start --node-id YOUR_NODE_ID --max-threads 10  # Leave 2+ cores for system
```

#### Memory Considerations

```bash
# For systems with 8GB RAM
nexus start --node-id YOUR_NODE_ID --max-threads 4 --max-difficulty MEDIUM

# For systems with 16GB+ RAM
nexus start --node-id YOUR_NODE_ID --max-threads 8 --max-difficulty LARGE

# For servers with 32GB+ RAM
nexus start --node-id YOUR_NODE_ID --max-threads 12 --max-difficulty EXTRA_LARGE
```

## 🖥 VPS Deployment

### Ubuntu/Debian Setup

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install build dependencies
sudo apt install -y build-essential pkg-config libssl-dev

# Clone and build
git clone https://github.com/your-username/nexus-cli.git
cd nexus-cli
./build-parallel.sh --install

# Run in background with screen/tmux
screen -S nexus
nexus start --node-id YOUR_NODE_ID --headless --max-threads 4 --max-tasks 1000
# Press Ctrl+A, then D to detach
```

### Systemd Service (Production)

Create `/etc/systemd/system/nexus.service`:

```ini
[Unit]
Description=Nexus CLI Parallel Runner
After=network.target

[Service]
Type=simple
User=nexus
WorkingDirectory=/home/nexus
ExecStart=/home/nexus/.cargo/bin/nexus start --node-id YOUR_NODE_ID --headless --max-threads 4
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable nexus
sudo systemctl start nexus
sudo systemctl status nexus
```

## 📊 Monitoring & Logging

### Performance Metrics

The CLI provides real-time metrics:

- **Tasks Completed**: Total successful proofs
- **Tasks Per Hour**: Current throughput rate
- **CPU Usage**: Per-core utilization
- **Memory Usage**: RAM consumption
- **Network Stats**: Bandwidth usage
- **Success Rate**: Task completion percentage

### Log Levels

```bash
# Debug logging
RUST_LOG=debug nexus start --node-id YOUR_NODE_ID

# Info logging (default)
RUST_LOG=info nexus start --node-id YOUR_NODE_ID

# Error logging only
RUST_LOG=error nexus start --node-id YOUR_NODE_ID
```

## 🧪 Testing

### Run the Test Suite

```bash
# Basic functionality test
./test-parallel.sh

# Smoke test with specific node ID
./test-parallel.sh --node-id YOUR_NODE_ID

# Performance benchmark
cargo test --release -- --nocapture
```

### Manual Testing

```bash
# Quick 10-task test
nexus start --node-id YOUR_NODE_ID --max-tasks 10 --max-difficulty SMALL

# 5-minute endurance test
timeout 300 nexus start --node-id YOUR_NODE_ID --headless

# Stress test (high difficulty)
nexus start --node-id YOUR_NODE_ID --max-tasks 50 --max-difficulty EXTRA_LARGE
```

## 🔧 Configuration

### Environment Variables

```bash
# Set default node ID
export NEXUS_NODE_ID=YOUR_NODE_ID

# Set custom log level
export RUST_LOG=info

# Custom data directory
export NEXUS_DATA_DIR=~/.nexus
```

### Config File (Optional)

Create `~/.nexus/config.toml`:

```toml
[default]
node_id = "YOUR_NODE_ID"
max_threads = 4
max_difficulty = "MEDIUM"
headless = false

[production]
node_id = "YOUR_NODE_ID"
max_threads = 8
max_difficulty = "LARGE"
headless = true
max_tasks = 1000
```

## 🐛 Troubleshooting

### Common Issues

**1. Build Failures**
```bash
# Clean and rebuild
cargo clean
./build-parallel.sh --install
```

**2. Permission Errors**
```bash
# Fix cargo permissions
sudo chown -R $USER:$USER ~/.cargo
```

**3. Memory Issues**
```bash
# Reduce worker count
nexus start --node-id YOUR_NODE_ID --max-threads 2
```

**4. Network Connectivity**
```bash
# Test network connection
curl -I https://api.nexus.xyz/health
```

### Performance Issues

**Low Throughput:**
- Increase `--max-threads` up to CPU core count
- Upgrade to higher `--max-difficulty` 
- Ensure stable internet connection
- Check system resources with `htop`

**High Memory Usage:**
- Reduce `--max-threads`
- Lower `--max-difficulty`
- Add memory monitoring: `watch -n 1 free -h`

**CPU Throttling:**
- Monitor temperature: `sensors`
- Reduce worker count during peak hours
- Consider thermal management

## 📈 Performance Benchmarks

### Typical Performance (Tasks/Hour)

| Hardware | Threads | Difficulty | Tasks/Hour |
|----------|---------|------------|------------|
| 4-core VPS | 3 | SMALL_MEDIUM | 150-200 |
| 8-core VPS | 6 | MEDIUM | 300-400 |
| 12-core Server | 10 | LARGE | 500-700 |
| 16-core Server | 14 | EXTRA_LARGE | 800-1000+ |

*Results may vary based on task complexity and network conditions*

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and test thoroughly
4. Commit: `git commit -m 'Add amazing feature'`
5. Push: `git push origin feature/amazing-feature`
6. Submit a Pull Request

### Development Setup

```bash
# Clone for development
git clone https://github.com/your-username/nexus-cli.git
cd nexus-cli

# Install development dependencies
rustup component add rustfmt clippy

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 📄 License

This project is dual-licensed under:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

## 📞 Support

- **Documentation**: [PARALLEL_IMPLEMENTATION.md](PARALLEL_IMPLEMENTATION.md)
- **Issues**: [GitHub Issues](https://github.com/your-username/nexus-cli/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/nexus-cli/discussions)

## 🎯 Roadmap

- [ ] GPU acceleration support
- [ ] Dynamic worker scaling
- [ ] Advanced load balancing algorithms
- [ ] Web-based monitoring dashboard
- [ ] Kubernetes deployment templates
- [ ] Performance analytics and reporting

---

**Made with ❤️ for the Nexus Network Community**

*Star ⭐ this repository if you find it useful!*
