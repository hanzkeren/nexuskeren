#!/bin/bash

echo "=== SMOKE TEST FOR NODE ID 36064546 ==="
echo "Testing Nexus CLI Parallel Implementation"
echo ""

# Change to project directory
cd /root/nexus-cli

# Check if binary exists
BINARY_PATH="clients/cli/target/release/nexus-network"
echo "Checking for binary at: $BINARY_PATH"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Binary not found. Building..."
    cd clients/cli
    cargo build --release
    cd ../..
fi

if [ -f "$BINARY_PATH" ]; then
    echo "✓ Binary found at $BINARY_PATH"
    
    # Test basic functionality
    echo ""
    echo "=== Testing Basic Commands ==="
    
    # Test version
    echo "Testing --version:"
    $BINARY_PATH --version || echo "Version command not available"
    
    # Test help
    echo ""
    echo "Testing --help:"
    $BINARY_PATH --help | head -10
    
    # Test start command help
    echo ""
    echo "Testing start --help (looking for max-threads):"
    $BINARY_PATH start --help | grep -E "(max-threads|parallel|worker)" || echo "No parallel options found in help"
    
    echo ""
    echo "=== System Information ==="
    echo "CPU Cores: $(nproc)"
    echo "Memory: $(free -h | awk '/^Mem:/ {print $2}')"
    
    echo ""
    echo "=== TEST WITH NODE ID 36064546 ==="
    echo "Starting nexus-network with node ID 36064546..."
    echo "Command: $BINARY_PATH start --node-id 36064546 --max-threads 2 --headless"
    
    # Run for just a few seconds to test startup
    timeout 10s $BINARY_PATH start --node-id 36064546 --max-threads 2 --headless || echo "Test completed (timeout expected)"
    
    echo ""
    echo "✓ Smoke test completed for node ID 36064546"
    
else
    echo "✗ Failed to build or find binary"
    exit 1
fi
