#!/bin/bash

# Build script for Nexus CLI with parallel execution support
# This script builds and optionally installs the modified nexus-cli

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "clients/cli/Cargo.toml" ]; then
    print_error "This script must be run from the nexus-cli root directory"
    exit 1
fi

print_status "Building Nexus CLI with Parallel Execution Support..."
echo ""

# Display system information
print_status "System Information:"
echo "  CPU Cores: $(nproc)"
echo "  Memory: $(free -h | awk '/^Mem:/ {print $2}')"
echo "  OS: $(uname -a)"
echo ""

# Navigate to CLI directory
cd clients/cli

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

print_status "Rust version: $(rustc --version)"
print_status "Cargo version: $(cargo --version)"
echo ""

# Build options
BUILD_TYPE="release"
INSTALL_FLAG=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            BUILD_TYPE="debug"
            shift
            ;;
        --install)
            INSTALL_FLAG="--install"
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --debug     Build in debug mode (faster compilation)"
            echo "  --install   Install the binary to ~/.cargo/bin/"
            echo "  --help      Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                    # Build release version"
            echo "  $0 --debug           # Build debug version"
            echo "  $0 --install         # Build and install"
            echo "  $0 --debug --install # Build debug version and install"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Build the project
print_status "Building nexus-cli (${BUILD_TYPE} mode)..."
if [ "$BUILD_TYPE" = "release" ]; then
    if ! cargo build --release; then
        print_error "Build failed!"
        exit 1
    fi
    BINARY_PATH="target/release/nexus-network"
else
    if ! cargo build; then
        print_error "Build failed!"
        exit 1
    fi
    BINARY_PATH="target/debug/nexus-network"
fi

print_success "Build completed successfully!"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    print_error "Binary not found at $BINARY_PATH"
    exit 1
fi

# Display binary information
BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
print_status "Binary location: $BINARY_PATH"
print_status "Binary size: $BINARY_SIZE"

# Test the binary
print_status "Testing the binary..."
if ! "$BINARY_PATH" --version &> /dev/null; then
    print_warning "Could not get version information (this may be normal)"
else
    print_success "Binary test passed"
fi

# Installation
if [ "$INSTALL_FLAG" = "--install" ]; then
    print_status "Installing nexus-cli..."
    
    # Create ~/.cargo/bin if it doesn't exist
    mkdir -p ~/.cargo/bin
    
    # Copy binary
    cp "$BINARY_PATH" ~/.cargo/bin/nexus
    chmod +x ~/.cargo/bin/nexus
    
    print_success "Installed to ~/.cargo/bin/nexus"
    
    # Check if ~/.cargo/bin is in PATH
    if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
        print_warning "~/.cargo/bin is not in your PATH"
        print_warning "Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo '  export PATH="$HOME/.cargo/bin:$PATH"'
    fi
    
    print_status "You can now run: nexus --help"
else
    print_status "Binary built successfully. To install, run:"
    echo "  $0 --install"
    echo ""
    print_status "Or manually copy the binary:"
    echo "  sudo cp $BINARY_PATH /usr/local/bin/nexus"
    echo "  chmod +x /usr/local/bin/nexus"
fi

echo ""
print_success "Build process completed!"
print_status "Parallel execution features:"
echo "  ✓ Automatic CPU core detection"
echo "  ✓ Memory-aware worker scaling"
echo "  ✓ Parallel proof generation"
echo "  ✓ Load balancing across workers"
echo ""
print_status "Usage examples:"
echo "  nexus start                    # Use all CPU cores"
echo "  nexus start --max-threads 4   # Limit to 4 threads"
echo "  nexus start --headless         # Run without UI"
echo ""
print_status "For more information, see PARALLEL_IMPLEMENTATION.md"
