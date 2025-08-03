# CIM Nix Integration

This directory contains the Nix integration for the Composable Information Machine (CIM) platform.

## Overview

The CIM Nix integration provides:
- Reproducible builds for all CIM modules
- Declarative system configuration
- Development environments
- Container image generation
- Deployment templates

## Structure

```
nix/
├── lib/              # Common Nix functions and utilities
├── packages/         # Package definitions for CIM modules
├── modules/          # NixOS module definitions
├── containers/       # Container image definitions
├── environments/     # Environment configurations
└── README.md         # This file
```

## Usage

### Building Packages

```bash
# Build all CIM packages
nix build .#cim-all

# Build specific package
nix build .#cim-events

# Build container image
nix build .#containers.cim-events
```

### Development

```bash
# Enter development shell
nix develop

# Run specific module
nix run .#cim-events

# Enter integration test shell
nix develop .#integration
```

### NixOS Deployment

```nix
# In your configuration.nix
{ config, pkgs, ... }:

{
  imports = [
    (builtins.getFlake "github:thecowboyai/cim").nixosModules.default
  ];
  
  services.cim-events = {
    enable = true;
    natsUrl = "nats://localhost:4222";
  };
  
  services.cim-projections = {
    enable = true;
    eventSource = "http://localhost:8080";
  };
}
```

### Container Deployment

```bash
# Build container
nix build .#containers.cim-events

# Load into Docker
docker load < result

# Run container
docker run -p 8080:8080 cim-events:latest
```

## Configuration

### Environment Variables

All CIM services support configuration via environment variables:

- `RUST_LOG`: Log level (trace, debug, info, warn, error)
- `CONFIG_FILE`: Path to configuration file

### Service-Specific Configuration

#### cim-events
- `NATS_URL`: NATS server URL
- `JETSTREAM_BUCKET`: JetStream bucket name
- `IPFS_API_URL`: IPFS API endpoint (optional)

#### cim-projections
- `EVENT_SOURCE`: Event source URL
- `POSTGRES_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string

## Integration with cim-domain-nix

The CIM platform uses `cim-domain-nix` for:
- Parsing Nix expressions
- Building Nix configurations
- Managing Nix-based deployments

Example usage:
```rust
use cim_domain_nix::{NixExpression, NixBuilder};

let expr = NixExpression::parse_file("config.nix")?;
let result = NixBuilder::new()
    .expression(expr)
    .build()?;
```

## Testing

### Unit Tests
```bash
nix develop -c cargo test
```

### Integration Tests
```bash
nix build .#testVm
./result/bin/run-cim-integration-test-vm
```

### NixOS Tests
```nix
# tests/integration.nix
{ pkgs, ... }:

pkgs.nixosTest {
  name = "cim-integration";
  
  nodes = {
    server = { ... }: {
      imports = [ ../modules ];
      services.cim-events.enable = true;
    };
  };
  
  testScript = ''
    server.wait_for_unit("cim-events")
    server.succeed("curl http://localhost:8080/health")
  '';
}
```

## Troubleshooting

### Common Issues

1. **Build fails with "No such file or directory"**
   - Ensure all source files are tracked in git
   - Run `git add -A` before building

2. **Service fails to start**
   - Check logs: `journalctl -u cim-events`
   - Verify dependencies are running (NATS, PostgreSQL, etc.)

3. **Container missing dependencies**
   - Ensure all runtime dependencies are included in container contents

### Debug Mode

Enable debug logging:
```nix
services.cim-events = {
  enable = true;
  logLevel = "debug";
  logFormat = "pretty";
};
```

## Contributing

When adding new modules:

1. Create package definition in `packages/`
2. Create NixOS module in `modules/`
3. Add container definition in `containers/`
4. Update `default.nix` files to include new module
5. Add tests and documentation

## License

Apache-2.0