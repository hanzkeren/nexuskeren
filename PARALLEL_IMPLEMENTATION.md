# Nexus CLI Parallel Implementation

This document describes the modifications made to nexus-cli to utilize all available CPU cores for optimal performance on VPS and dedicated servers.

## Overview

The nexus-cli has been enhanced with a parallel execution engine that automatically detects and utilizes all CPU cores on the system, significantly improving throughput for proof generation tasks.

## Key Features

### 1. Automatic CPU Detection
- Automatically detects the number of logical CPU cores
- Considers available memory when determining optimal worker count
- Applies intelligent resource constraints to prevent OOM conditions

### 2. Parallel Worker Architecture
- Spawns one worker thread per CPU core by default
- Each worker operates independently with its own task queue
- Load balancing across all workers for optimal resource utilization

### 3. Memory-Aware Scaling
- Estimates memory usage per worker (conservative 2GB per worker)
- Automatically limits worker count based on available system memory
- Prevents system instability due to excessive memory usage

### 4. Backward Compatibility
- Maintains existing CLI interface (`nexus start`, etc.)
- Honors existing `--max-threads` parameter for manual limiting
- Falls back gracefully on systems with limited resources

## Usage

### Default Behavior (Recommended)
```bash
# Uses all available CPU cores automatically
nexus start
```

### Manual Thread Limiting
```bash
# Limit to 4 threads
nexus start --max-threads 4

# Limit to 1 thread (single-threaded mode)
nexus start --max-threads 1
```

### Other Options
```bash
# Headless mode with all cores
nexus start --headless

# With task limits distributed across workers
nexus start --max-tasks 100  # Tasks distributed evenly across all workers

# With difficulty override
nexus start --max-difficulty LARGE
```

## Implementation Details

### New Components

#### 1. ParallelRunner (`src/workers/parallel_runner.rs`)
- Main orchestrator for parallel execution
- Handles worker lifecycle management
- Provides system resource detection
- Manages event aggregation from multiple workers

#### 2. Enhanced Runtime (`src/runtime.rs`)
- New `start_parallel_authenticated_workers()` function
- Backward-compatible `start_authenticated_worker()` wrapper
- System information logging

#### 3. Updated Session Management (`src/session/setup.rs`)
- Removed arbitrary 8-worker limit
- Integrated CPU core detection
- Enhanced memory warning system

### Architecture

```
Main Process
├── ParallelRunner
│   ├── Worker 1 (CPU Core 1)
│   ├── Worker 2 (CPU Core 2)
│   ├── ...
│   └── Worker N (CPU Core N)
├── Event Aggregator
└── UI/Dashboard (shows combined metrics)
```

### Event Handling
- Each worker generates events independently
- Events are tagged with worker ID for debugging
- Combined event stream fed to UI/dashboard
- Maintains real-time status updates across all workers

## Performance Benefits

### Before (Single Worker)
- CPU utilization: ~12.5% on 8-core system
- Memory utilization: ~2GB
- Tasks/hour: X

### After (Parallel Workers)
- CPU utilization: ~100% on 8-core system
- Memory utilization: Scales with worker count
- Tasks/hour: ~8X (linear scaling with cores)

## System Requirements

### Minimum
- 1 CPU core
- 2GB RAM
- Original nexus-cli dependencies

### Recommended for Parallel Execution
- 4+ CPU cores
- 8GB+ RAM (2GB per worker + OS overhead)
- SSD storage for optimal I/O

### VPS Optimization
The parallel implementation is specifically optimized for VPS environments:
- Automatic resource detection prevents over-provisioning
- Memory-aware scaling prevents OOM kills
- Efficient subprocess management minimizes overhead

## Build Instructions

### Standard Build
```bash
cd nexus-cli/clients/cli
cargo build --release
```

### Development Build
```bash
cd nexus-cli/clients/cli
cargo build
```

### Installation
```bash
cd nexus-cli/clients/cli
cargo install --path .
```

### Alternative: Local Development
```bash
cd nexus-cli/clients/cli
cargo build --release
sudo cp target/release/nexus-network /usr/local/bin/nexus
```

## Testing

### Unit Tests
```bash
cd nexus-cli/clients/cli
cargo test
```

### Integration Testing
```bash
# Test with single worker
nexus start --max-threads 1 --max-tasks 5

# Test with multiple workers
nexus start --max-threads 4 --max-tasks 20

# Test with auto-detection
nexus start --max-tasks 10
```

### Performance Benchmarking
```bash
# Measure single-threaded performance
time nexus start --max-threads 1 --max-tasks 10

# Measure multi-threaded performance
time nexus start --max-tasks 10
```

## Monitoring

### System Information
The CLI now displays system information at startup:
```
System Info: 8 logical cores, 16 GB total memory, 12 GB available memory
Starting 8 parallel workers across 8 CPU cores
```

### Worker Identification
Events are tagged with worker IDs in the UI:
```
[W0] Fetching task...
[W1] Proving task 12345...
[W2] Step 3 of 4: Proof generated for task 12346
```

### Resource Monitoring
- CPU usage displays aggregate across all workers
- Memory usage scales with active worker count
- Real-time throughput metrics in dashboard

## Troubleshooting

### High Memory Usage
If experiencing OOM conditions:
```bash
# Reduce worker count
nexus start --max-threads 4

# Check system resources
free -h
cat /proc/cpuinfo | grep processor | wc -l
```

### Performance Issues
If not seeing expected speedup:
```bash
# Check CPU binding
htop  # Verify all cores are active

# Check I/O bottlenecks
iostat -x 1

# Check network latency
ping orchestrator-url
```

### Worker Failures
If individual workers fail:
- Check logs for worker-specific errors
- Verify network connectivity
- Ensure sufficient memory per worker

## Security Considerations

### Process Isolation
- Each worker runs in subprocess for memory isolation
- Worker failures don't affect other workers
- Main process remains stable during worker crashes

### Resource Limits
- Automatic memory constraints prevent system DOS
- CPU usage respects system cgroup limits
- Network requests are rate-limited per worker

## Future Enhancements

### Planned Features
1. Dynamic worker scaling based on workload
2. NUMA-aware worker placement
3. GPU acceleration support
4. Distributed computing across multiple machines

### Configuration Options
1. Custom memory limits per worker
2. CPU affinity settings
3. I/O priority configuration
4. Network optimization parameters

## Contributing

When contributing to the parallel implementation:

1. Maintain backward compatibility
2. Add unit tests for new components
3. Update documentation for API changes
4. Test on various system configurations
5. Verify memory usage patterns

## Support

For issues specific to the parallel implementation:
1. Include system specifications (CPU, RAM, OS)
2. Provide worker count and configuration used
3. Include relevant log excerpts with worker IDs
4. Report performance metrics if available
