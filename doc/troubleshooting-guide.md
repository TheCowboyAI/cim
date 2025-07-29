# CIM Troubleshooting Guide

Common issues and solutions for the Composable Information Machine.

## Table of Contents

1. [Diagnostic Tools](#diagnostic-tools)
2. [Common Issues](#common-issues)
3. [NATS Issues](#nats-issues)
4. [Event Processing Issues](#event-processing-issues)
5. [Domain Issues](#domain-issues)
6. [Performance Issues](#performance-issues)
7. [Network Issues](#network-issues)
8. [Storage Issues](#storage-issues)
9. [Security Issues](#security-issues)
10. [Debug Techniques](#debug-techniques)

## Diagnostic Tools

### CIM Health Check

```bash
# Check overall system health
cim health check

# Check specific component
cim health check --component=nats
cim health check --component=events
cim health check --component=domains

# Verbose output
cim health check -v
```

### Status Dashboard

```bash
# Start diagnostic dashboard
cim dashboard --port=8888

# Access at http://localhost:8888
# Shows:
# - Event throughput
# - Error rates
# - Domain status
# - Resource usage
```

### Log Analysis

```bash
# View logs
cim logs --tail=100
cim logs --follow
cim logs --level=error
cim logs --domain=inventory

# Search logs
cim logs --search="timeout"
cim logs --correlation-id="req-123"

# Export logs
cim logs --from="2024-01-29T00:00:00Z" --export=logs.json
```

## Common Issues

### 1. CIM Won't Start

**Symptoms:**
- Service fails to start
- Immediate crash after startup
- No logs generated

**Diagnosis:**
```bash
# Check if port is in use
lsof -i :9090
netstat -tulpn | grep 9090

# Check configuration
cim config validate

# Try verbose mode
cim serve -vvv
```

**Solutions:**

1. **Port conflict:**
   ```bash
   # Change port
   export CIM_PORT=9091
   cim serve
   ```

2. **Missing configuration:**
   ```bash
   # Generate default config
   cim init --config > cim.yaml
   ```

3. **Permission issues:**
   ```bash
   # Check file permissions
   ls -la /var/lib/cim
   sudo chown -R cim:cim /var/lib/cim
   ```

### 2. Cannot Connect to CIM

**Symptoms:**
- Connection refused errors
- Timeouts
- 503 Service Unavailable

**Diagnosis:**
```bash
# Test connectivity
curl -v http://localhost:9090/health
telnet localhost 9090

# Check firewall
sudo iptables -L -n | grep 9090
```

**Solutions:**

1. **Service not running:**
   ```bash
   systemctl status cim
   systemctl start cim
   ```

2. **Firewall blocking:**
   ```bash
   # Allow CIM port
   sudo ufw allow 9090/tcp
   # or
   sudo iptables -A INPUT -p tcp --dport 9090 -j ACCEPT
   ```

3. **Wrong endpoint:**
   ```bash
   # Verify configuration
   echo $CIM_API_URL
   # Should match your deployment
   ```

## NATS Issues

### 1. NATS Connection Failed

**Symptoms:**
- "no servers available for connection"
- "connection refused"
- Event publishing fails

**Diagnosis:**
```bash
# Check NATS status
nats-server -v
nats server ping

# Test connection
nats pub test "hello" --server=nats://localhost:4222

# Check JetStream
nats stream ls
```

**Solutions:**

1. **NATS not running:**
   ```bash
   # Start NATS with JetStream
   nats-server -js
   
   # Or with Docker
   docker run -d -p 4222:4222 -p 8222:8222 nats:latest -js
   ```

2. **Wrong NATS URL:**
   ```yaml
   # cim.yaml
   nats:
     url: nats://localhost:4222  # Correct format
   ```

3. **JetStream not enabled:**
   ```bash
   # Enable JetStream
   nats-server -js -sd /var/lib/nats
   ```

### 2. Event Stream Issues

**Symptoms:**
- Events not being consumed
- Duplicate events
- Missing events

**Diagnosis:**
```bash
# Check stream health
nats stream info CIM_EVENTS

# Check consumer status
nats consumer info CIM_EVENTS CIM_CONSUMER

# Monitor stream
nats stream view CIM_EVENTS
```

**Solutions:**

1. **Reset consumer position:**
   ```bash
   # Delete and recreate consumer
   nats consumer rm CIM_EVENTS CIM_CONSUMER
   nats consumer add CIM_EVENTS CIM_CONSUMER \
     --deliver=all \
     --ack-policy=explicit \
     --replay-policy=instant
   ```

2. **Increase stream limits:**
   ```bash
   nats stream edit CIM_EVENTS \
     --max-msgs=-1 \
     --max-bytes=-1 \
     --max-age=30d
   ```

## Event Processing Issues

### 1. Events Not Processing

**Symptoms:**
- Commands accepted but nothing happens
- Events published but not consumed
- Projections not updating

**Diagnosis:**
```bash
# Check event processor status
cim events status

# List pending events
cim events pending

# Check specific event
cim events get evt_123
```

**Solutions:**

1. **Restart event processors:**
   ```bash
   cim events restart-processors
   ```

2. **Clear stuck events:**
   ```bash
   # Reprocess failed events
   cim events reprocess --status=failed
   
   # Skip poison events
   cim events skip evt_problematic_123
   ```

3. **Increase processor capacity:**
   ```yaml
   # cim.yaml
   events:
     processors:
       count: 10  # Increase from default
       batch_size: 100
   ```

### 2. Event Ordering Issues

**Symptoms:**
- Events processed out of order
- Aggregate version conflicts
- Inconsistent projections

**Diagnosis:**
```bash
# Check event order
cim events list --aggregate=WIDGET-001 --order=version

# Verify causation chain
cim events trace --correlation-id=req-123
```

**Solutions:**

1. **Enable strict ordering:**
   ```yaml
   events:
     ordering:
       strict: true
       partition_by: aggregate_id
   ```

2. **Fix version conflicts:**
   ```bash
   # Rebuild aggregate from events
   cim aggregates rebuild --id=WIDGET-001
   ```

## Domain Issues

### 1. Domain Not Found

**Symptoms:**
- "domain not found" errors
- Commands rejected
- Queries return empty

**Diagnosis:**
```bash
# List registered domains
cim domains list

# Check domain status
cim domains status inventory
```

**Solutions:**

1. **Register domain:**
   ```bash
   # Manual registration
   cim domains register inventory \
     --module=cim-domain-inventory
   ```

2. **Reload domains:**
   ```bash
   cim domains reload
   ```

### 2. Domain Version Conflicts

**Symptoms:**
- "incompatible domain version"
- Migration errors
- Schema mismatches

**Solutions:**

1. **Run migrations:**
   ```bash
   cim domains migrate inventory --to-version=2.0.0
   ```

2. **Force compatibility mode:**
   ```yaml
   domains:
     inventory:
       compatibility_mode: true
       supported_versions: ["1.0", "2.0"]
   ```

## Performance Issues

### 1. Slow Response Times

**Symptoms:**
- High latency
- Timeouts
- Poor throughput

**Diagnosis:**
```bash
# Performance metrics
cim perf report

# Trace slow requests
cim trace --duration=">1s"

# Profile CPU usage
cim profile cpu --duration=30s
```

**Solutions:**

1. **Enable caching:**
   ```yaml
   cache:
     enabled: true
     size: 1GB
     ttl: 300s
   ```

2. **Optimize projections:**
   ```bash
   # Rebuild projections
   cim projections rebuild --parallel=4
   
   # Add indexes
   cim projections index add --field=sku
   ```

3. **Scale horizontally:**
   ```bash
   # Add more nodes
   cim cluster add-node --replicas=3
   ```

### 2. Memory Issues

**Symptoms:**
- Out of memory errors
- Garbage collection pauses
- Memory leaks

**Diagnosis:**
```bash
# Memory usage
cim stats memory

# Heap dump
cim debug heap-dump

# Find leaks
cim debug find-leaks
```

**Solutions:**

1. **Increase memory limits:**
   ```bash
   # Set memory limit
   export CIM_MEMORY_LIMIT=4G
   ```

2. **Enable memory profiling:**
   ```yaml
   debug:
     memory_profiling: true
     gc_verbose: true
   ```

## Network Issues

### 1. Connection Timeouts

**Symptoms:**
- Intermittent failures
- "context deadline exceeded"
- Connection pool exhausted

**Solutions:**

1. **Increase timeouts:**
   ```yaml
   network:
     connect_timeout: 10s
     read_timeout: 60s
     write_timeout: 60s
   ```

2. **Adjust connection pool:**
   ```yaml
   network:
     pool:
       max_idle: 100
       max_open: 1000
       idle_timeout: 90s
   ```

### 2. DNS Resolution Issues

**Symptoms:**
- "no such host" errors
- Service discovery failures

**Solutions:**

1. **Use IP addresses:**
   ```yaml
   nats:
     url: nats://10.0.1.5:4222
   ```

2. **Configure DNS cache:**
   ```yaml
   network:
     dns:
       cache_ttl: 60s
       negative_ttl: 5s
   ```

## Storage Issues

### 1. Disk Space

**Symptoms:**
- "no space left on device"
- Write failures
- Corruption errors

**Diagnosis:**
```bash
# Check disk usage
df -h /var/lib/cim

# Find large files
du -sh /var/lib/cim/* | sort -h

# Check event store size
cim storage stats
```

**Solutions:**

1. **Clean up old data:**
   ```bash
   # Archive old events
   cim events archive --before="30d ago"
   
   # Compact storage
   cim storage compact
   ```

2. **Enable compression:**
   ```yaml
   storage:
     compression:
       enabled: true
       algorithm: zstd
       level: 3
   ```

### 2. Corruption Issues

**Symptoms:**
- Checksum failures
- Cannot read events
- Inconsistent state

**Solutions:**

1. **Verify and repair:**
   ```bash
   # Check integrity
   cim storage verify
   
   # Repair if needed
   cim storage repair --confirm
   ```

2. **Restore from backup:**
   ```bash
   # List backups
   cim backup list
   
   # Restore
   cim backup restore --id=backup_20240129
   ```

## Security Issues

### 1. Authentication Failures

**Symptoms:**
- 401 Unauthorized
- "invalid token"
- "expired credentials"

**Solutions:**

1. **Refresh tokens:**
   ```bash
   cim auth refresh
   ```

2. **Check token validity:**
   ```bash
   cim auth verify --token=$TOKEN
   ```

### 2. Permission Denied

**Symptoms:**
- 403 Forbidden
- "insufficient permissions"
- Operation not allowed

**Solutions:**

1. **Check user permissions:**
   ```bash
   cim auth permissions --user=user-123
   ```

2. **Grant permissions:**
   ```bash
   cim auth grant --user=user-123 --permission="domains:inventory:write"
   ```

## Debug Techniques

### 1. Enable Debug Logging

```yaml
# cim.yaml
logging:
  level: debug
  outputs:
    - stdout
    - file: /var/log/cim/debug.log
  
  # Component-specific
  components:
    nats: trace
    events: debug
    domains: info
```

### 2. Distributed Tracing

```bash
# Enable tracing
export CIM_TRACING_ENABLED=true
export CIM_JAEGER_ENDPOINT=http://localhost:14268/api/traces

# View traces in Jaeger UI
open http://localhost:16686
```

### 3. Event Replay

```bash
# Replay events for debugging
cim debug replay \
  --from="2024-01-29T10:00:00Z" \
  --to="2024-01-29T11:00:00Z" \
  --domain=inventory \
  --dry-run
```

### 4. Interactive Debugging

```bash
# Start REPL
cim repl

> domains.list()
> events.get("evt_123")
> aggregates.load("WIDGET-001")
> debug.trace_event("evt_123")
```

## Emergency Procedures

### System Recovery

```bash
#!/bin/bash
# emergency-recovery.sh

# 1. Stop all services
systemctl stop cim

# 2. Backup current state
tar -czf /backup/emergency-$(date +%Y%m%d-%H%M%S).tar.gz /var/lib/cim

# 3. Clear problematic data
cim storage clear-cache
cim events clear-failed

# 4. Verify configuration
cim config validate

# 5. Start in safe mode
cim serve --safe-mode --single-domain=inventory

# 6. Gradually enable features
cim feature enable events
cim feature enable projections
cim feature enable all-domains

# 7. Verify health
cim health check --detailed
```

### Data Recovery

```bash
# Recover from event store
cim recovery from-events \
  --output=/recovery/state.json \
  --verify

# Rebuild all projections
cim projections rebuild-all \
  --parallel=4 \
  --progress

# Verify data integrity
cim data verify --deep
```

## Getting Help

### Collect Diagnostic Information

```bash
# Generate support bundle
cim support bundle \
  --include-logs \
  --include-config \
  --include-metrics \
  --output=support-bundle.tar.gz
```

### Resources

- **Documentation**: https://docs.cim.io
- **GitHub Issues**: https://github.com/thecowboyai/cim/issues
- **Community Forum**: https://discuss.cim.io
- **Emergency Support**: support@cim.io

### Useful Commands Summary

```bash
# Quick diagnostics
cim doctor                    # Run all diagnostic checks
cim events stats             # Event processing statistics
cim domains health           # Domain health status
cim cluster status          # Cluster node status
cim storage analyze         # Storage usage analysis
cim performance report      # Performance metrics report
```

---

Remember: Most issues can be resolved by:
1. Checking logs for specific errors
2. Verifying configuration
3. Ensuring all dependencies are running
4. Restarting affected components

When in doubt, run `cim doctor` for automated diagnostics.