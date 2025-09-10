# Nexus CLI - Usage Examples

Real-world examples for running Nexus CLI with parallel execution support.

## 🎯 Quick Examples

### Basic Usage
```bash
# Start with your node ID (uses all CPU cores)
nexus start --node-id 36064546

# Start with specific thread count
nexus start --node-id 36064546 --max-threads 4

# Run without UI (headless mode)
nexus start --node-id 36064546 --headless
```

### Task Control
```bash
# Limit to 100 tasks total
nexus start --node-id 36064546 --max-tasks 100

# Set difficulty to MEDIUM
nexus start --node-id 36064546 --max-difficulty MEDIUM

# Combined: 50 tasks, LARGE difficulty, 4 threads
nexus start --node-id 36064546 --max-tasks 50 --max-difficulty LARGE --max-threads 4
```

## 🖥 System-Specific Configurations

### Low-End Hardware (2-4 cores, 8GB RAM)
```bash
# Conservative settings
nexus start --node-id 36064546 --max-threads 2 --max-difficulty SMALL

# Test run
nexus start --node-id 36064546 --max-threads 2 --max-tasks 20 --max-difficulty SMALL_MEDIUM

# Background processing
nexus start --node-id 36064546 --max-threads 1 --headless --max-difficulty SMALL
```

### Mid-Range Hardware (4-8 cores, 16GB RAM)
```bash
# Balanced performance
nexus start --node-id 36064546 --max-threads 6 --max-difficulty MEDIUM

# Production run
nexus start --node-id 36064546 --max-threads 4 --max-tasks 500 --headless

# High throughput
nexus start --node-id 36064546 --max-threads 6 --max-difficulty LARGE
```

### High-End Hardware (8+ cores, 32GB+ RAM)
```bash
# Maximum performance
nexus start --node-id 36064546 --max-threads 12 --max-difficulty EXTRA_LARGE

# Server deployment
nexus start --node-id 36064546 --max-threads 10 --max-tasks 2000 --headless

# Stress test
nexus start --node-id 36064546 --max-threads 14 --max-difficulty EXTRA_LARGE --max-tasks 100
```

## 🌐 VPS/Cloud Deployment Examples

### Small VPS (2 vCPU, 4GB RAM)
```bash
# Conservative production
nexus start --node-id 36064546 --max-threads 1 --headless --max-difficulty SMALL_MEDIUM

# Testing configuration
nexus start --node-id 36064546 --max-threads 2 --max-tasks 50 --headless
```

### Medium VPS (4 vCPU, 8GB RAM)
```bash
# Standard production
nexus start --node-id 36064546 --max-threads 3 --headless --max-difficulty MEDIUM

# High availability
nexus start --node-id 36064546 --max-threads 2 --max-tasks 1000 --headless

# Burst processing
nexus start --node-id 36064546 --max-threads 4 --max-difficulty LARGE --max-tasks 200
```

### Large VPS (8+ vCPU, 16GB+ RAM)
```bash
# High-performance production
nexus start --node-id 36064546 --max-threads 6 --headless --max-difficulty LARGE

# Dedicated proving
nexus start --node-id 36064546 --max-threads 8 --max-difficulty EXTRA_LARGE --headless

# Batch processing
nexus start --node-id 36064546 --max-threads 7 --max-tasks 5000 --headless
```

## ⏰ Time-Based Usage

### Short Testing Sessions
```bash
# 5-minute test
timeout 300 nexus start --node-id 36064546 --max-threads 2 --headless

# 30-minute session
timeout 1800 nexus start --node-id 36064546 --max-tasks 100 --headless

# Quick benchmark
timeout 600 nexus start --node-id 36064546 --max-threads 4 --max-difficulty MEDIUM
```

### Long-Running Sessions
```bash
# 24-hour run with task limit
nexus start --node-id 36064546 --max-tasks 2000 --headless --max-threads 4

# Unlimited duration (until manually stopped)
nexus start --node-id 36064546 --headless --max-threads 6

# Weekend batch processing
nexus start --node-id 36064546 --max-tasks 10000 --max-difficulty LARGE --headless
```

## 🔧 Development & Testing

### Development Testing
```bash
# Quick functionality test
nexus start --node-id 36064546 --max-tasks 5 --max-difficulty SMALL

# Performance profiling
nexus start --node-id 36064546 --max-tasks 20 --max-threads 4 --max-difficulty MEDIUM

# Stress testing
nexus start --node-id 36064546 --max-tasks 50 --max-difficulty EXTRA_LARGE --max-threads 8
```

### Continuous Integration
```bash
# CI test run
nexus start --node-id 36064546 --max-tasks 10 --headless --max-difficulty SMALL

# Performance regression test
timeout 300 nexus start --node-id 36064546 --max-threads 2 --headless
```

## 📊 Monitoring & Logging Examples

### With Custom Logging
```bash
# Debug logging
RUST_LOG=debug nexus start --node-id 36064546 --max-tasks 10

# Error logging only
RUST_LOG=error nexus start --node-id 36064546 --headless

# Custom log file
nexus start --node-id 36064546 --headless 2>&1 | tee nexus.log
```

### Performance Monitoring
```bash
# Monitor system resources while running
htop &
nexus start --node-id 36064546 --max-threads 4 --headless

# Memory monitoring
watch -n 1 'free -h' &
nexus start --node-id 36064546 --max-threads 6

# CPU temperature monitoring (Linux)
watch -n 5 'sensors' &
nexus start --node-id 36064546 --max-threads 8 --max-difficulty LARGE
```

## 🚀 Production Deployment Scenarios

### 24/7 Production Server
```bash
# High-reliability configuration
nexus start --node-id 36064546 \
  --headless \
  --max-threads 6 \
  --max-difficulty LARGE \
  --max-tasks 5000
```

### Load-Balanced Multi-Instance
```bash
# Instance 1
nexus start --node-id 36064546 --headless --max-threads 3 &

# Instance 2  
nexus start --node-id 36064547 --headless --max-threads 3 &

# Instance 3
nexus start --node-id 36064548 --headless --max-threads 2 &
```

### Resource-Adaptive Configuration
```bash
# Peak hours (high performance)
if [ $(date +%H) -ge 9 ] && [ $(date +%H) -le 17 ]; then
  nexus start --node-id 36064546 --max-threads 8 --max-difficulty LARGE --headless
else
  # Off-peak (conservative)
  nexus start --node-id 36064546 --max-threads 4 --max-difficulty MEDIUM --headless
fi
```

## 🐳 Docker Examples

### Basic Docker Usage
```bash
# Run in Docker container
docker run -it nexus-cli nexus start --node-id 36064546 --headless

# With volume mounting
docker run -v ~/.nexus:/root/.nexus -it nexus-cli nexus start --node-id 36064546

# Background Docker container
docker run -d --name nexus-worker nexus-cli nexus start --node-id 36064546 --headless
```

### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'
services:
  nexus-worker:
    build: .
    command: nexus start --node-id 36064546 --headless --max-threads 4
    restart: unless-stopped
    volumes:
      - ./data:/root/.nexus
```

## 🔄 Automation Scripts

### Simple Automation
```bash
#!/bin/bash
# auto-nexus.sh - Restart on failure

while true; do
    echo "Starting Nexus CLI..."
    nexus start --node-id 36064546 --headless --max-threads 4 --max-tasks 1000
    
    echo "Nexus stopped. Restarting in 10 seconds..."
    sleep 10
done
```

### Advanced Automation
```bash
#!/bin/bash
# smart-nexus.sh - Adaptive configuration

CPU_CORES=$(nproc)
MEMORY_GB=$(free -g | awk '/^Mem:/ {print $2}')

# Calculate optimal threads (leave 2 cores for system)
THREADS=$((CPU_CORES - 2))
if [ $THREADS -lt 1 ]; then
    THREADS=1
fi

# Set difficulty based on memory
if [ $MEMORY_GB -ge 32 ]; then
    DIFFICULTY="EXTRA_LARGE"
elif [ $MEMORY_GB -ge 16 ]; then
    DIFFICULTY="LARGE"
elif [ $MEMORY_GB -ge 8 ]; then
    DIFFICULTY="MEDIUM"
else
    DIFFICULTY="SMALL_MEDIUM"
fi

echo "Starting with $THREADS threads, $DIFFICULTY difficulty"
nexus start --node-id 36064546 \
  --headless \
  --max-threads $THREADS \
  --max-difficulty $DIFFICULTY
```

## 📈 Performance Optimization Examples

### Memory-Optimized
```bash
# For systems with limited RAM
nexus start --node-id 36064546 \
  --max-threads 2 \
  --max-difficulty SMALL \
  --max-tasks 500 \
  --headless
```

### CPU-Optimized
```bash
# For high-core count systems
nexus start --node-id 36064546 \
  --max-threads 12 \
  --max-difficulty LARGE \
  --headless
```

### Network-Optimized
```bash
# For limited bandwidth
nexus start --node-id 36064546 \
  --max-threads 3 \
  --max-difficulty MEDIUM \
  --max-tasks 200 \
  --headless
```

## 🧪 Testing & Benchmarking

### Quick Performance Test
```bash
# 10-task benchmark
time nexus start --node-id 36064546 --max-tasks 10 --headless --max-difficulty MEDIUM
```

### Throughput Testing
```bash
# Measure tasks per hour
echo "Starting throughput test..."
START_TIME=$(date +%s)
nexus start --node-id 36064546 --max-tasks 100 --headless --max-threads 4
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))
TASKS_PER_HOUR=$((100 * 3600 / DURATION))
echo "Throughput: $TASKS_PER_HOUR tasks/hour"
```

### Resource Usage Test
```bash
# Monitor resource usage during test
(
  echo "timestamp,cpu_percent,memory_mb" > resource_usage.csv
  while true; do
    CPU=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
    MEM=$(free -m | awk '/^Mem:/ {print $3}')
    echo "$(date +%s),$CPU,$MEM" >> resource_usage.csv
    sleep 5
  done
) &

nexus start --node-id 36064546 --max-tasks 50 --headless
kill %1  # Stop monitoring
```

---

## 💡 Pro Tips

1. **Start Small**: Begin with conservative settings and gradually increase
2. **Monitor Resources**: Use `htop`, `free`, and `iostat` to monitor system health
3. **Test First**: Always test with `--max-tasks 10` before long runs
4. **Save Logs**: Redirect output to files for debugging: `nexus start ... 2>&1 | tee nexus.log`
5. **Use Screen/Tmux**: For long-running sessions: `screen -S nexus`

## 📞 Need Help?

- Check system resources: `htop`
- Monitor logs: `tail -f nexus.log`
- Test connectivity: `ping api.nexus.xyz`
- Reduce load: Lower `--max-threads` or `--max-difficulty`
