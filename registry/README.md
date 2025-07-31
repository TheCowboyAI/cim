# CIM Module Registry

This directory contains the official registry of all CIM modules and domain implementations.

## Files

- `modules.yaml` - Complete listing of all CIM modules
- `modules-graph.json` - **Real-time dependency graph with git tracking**
- `private-domains.yaml` - Registry of private domain implementations
- `status.json` - Current health and status of modules
- `changelog.md` - Automatically generated changelog
- `graph-system.md` - Documentation for the graph system

## How It Works

1. All `cim-*` repositories in the `thecowboyai` organization are automatically tracked
2. When a module is updated, it sends an event via NATS
3. This triggers an update to the registry
4. The registry queries GitHub for the latest information
5. Files in this directory are updated automatically

## Querying the Registry

### Via Graph Query Tool (Recommended)
```bash
# Query module details
./scripts/query-graph.sh --module cim-domain

# Show dependencies
./scripts/query-graph.sh --dependencies cim-security

# Show git information
./scripts/query-graph.sh --git cim-ipld

# List production modules
./scripts/query-graph.sh --production
```

### Via GitHub API
```bash
# List all CIM modules
gh api graphql -f query='
  query {
    organization(login: "thecowboyai") {
      repositories(first: 100, query: "cim") {
        nodes {
          name
          description
          updatedAt
        }
      }
    }
  }
'
```

### Via Registry Files
```bash
# Get module graph with git info
curl https://raw.githubusercontent.com/thecowboyai/cim/main/registry/modules-graph.json

# Get all modules (YAML format)
curl https://raw.githubusercontent.com/thecowboyai/cim/main/registry/modules.yaml

# Get private domains
curl https://raw.githubusercontent.com/thecowboyai/cim/main/registry/private-domains.yaml
```

## Module Types

- **template** - Starting templates (cim-start)
- **core** - Core infrastructure modules
- **domain** - Business domain modules
- **integration** - External system integrations
- **utility** - Helper modules

## Module Status

- **production** - Production-ready, stable API (currently: cim-domain, cim-ipld, cim-component, cim-subject)
- **development** - Under active development, API may change
- **template** - Template for creating new modules
- **deprecated** - No longer maintained

### Production-Ready Modules
Only 4 modules currently meet production standards:
- `cim-ipld` - Content-addressed storage
- `cim-component` - ECS component system
- `cim-subject` - Event routing algebra
- `cim-domain` - Domain patterns and traits

All other modules are being updated to meet production readiness standards defined in `.claude/standards/production-readiness.md`.

## Adding a New Module

1. Create repository with `cim-*` naming convention
2. Add `cim` and appropriate type topics
3. Include `cim.yaml` metadata file
4. Push to main branch
5. Registry will auto-update within minutes