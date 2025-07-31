# CIM Module Graph System

## Overview

The CIM repository maintains a real-time graph of all modules, tracking:
- Git commits and versions
- Module dependencies
- Production readiness status
- Update timestamps

## Components

### 1. Module Graph (`modules-graph.json`)
The authoritative graph structure containing:
- **Nodes**: Each CIM module with metadata
- **Edges**: Dependency relationships
- **Git tracking**: Latest commits, tags, versions
- **Status**: Production, development, or template

### 2. Update Mechanisms

#### Automatic Updates via Git Commits
When any `cim-*` module commits to main:
1. Module's GitHub Action sends notification
2. CIM registry workflow triggered
3. Graph updated with commit hash and timestamp
4. Catalog synchronized

#### Manual Updates
```bash
# Trigger update for specific module
gh workflow run update-module-graph.yml \
  -f module_name=cim-domain \
  -f commit_hash=abc123d \
  -f version=1.0.1
```

### 3. Query Tools

#### Graph Query Script
```bash
# Show module details
./scripts/query-graph.sh --module cim-domain

# Show dependencies
./scripts/query-graph.sh --dependencies cim-security

# Show what depends on a module
./scripts/query-graph.sh --dependents cim-domain

# Show git information
./scripts/query-graph.sh --git cim-ipld

# List production modules
./scripts/query-graph.sh --production

# Find outdated modules
./scripts/query-graph.sh --outdated
```

#### Direct JSON Queries
```bash
# Get all production modules
jq '.graph.nodes | to_entries[] | select(.value.status == "production") | .key' \
  registry/modules-graph.json

# Get module version
jq '.graph.nodes["cim-domain"].version.current' \
  registry/modules-graph.json

# Get latest commit
jq '.graph.nodes["cim-domain"].git.latest_commit' \
  registry/modules-graph.json
```

## Graph Structure

### Node Schema
```json
{
  "id": "module-name",
  "type": "core|domain|template",
  "status": "production|development|template",
  "repository": "https://github.com/thecowboyai/module-name",
  "git": {
    "latest_commit": "abc123d",
    "latest_tag": "v1.0.0",
    "last_updated": "2025-01-30T10:00:00Z"
  },
  "version": {
    "current": "1.0.0",
    "releases": ["1.0.0", "0.9.0"]
  },
  "dependencies": ["dep1", "dep2"],
  "dependents": ["mod1", "mod2"]
}
```

### Edge Schema
```json
{
  "from": "source-module",
  "to": "target-module",
  "type": "dependency"
}
```

## Module Notification Setup

Each CIM module must include the notification workflow:

1. Copy template to module repository:
```bash
cp templates/module-notify-action.yml \
  ../cim-your-module/.github/workflows/notify-cim-registry.yml
```

2. Configure repository secrets:
- `CIM_REGISTRY_TOKEN`: PAT with repo dispatch permission
- `NATS_SERVER`: Optional NATS server URL
- `NATS_CREDS`: Optional NATS credentials

3. Ensure version tracking in:
- `Cargo.toml` for Rust modules
- `package.json` for JavaScript
- `cim.yaml` for metadata

## Visualization

### Generate Dependency Graph
```bash
# Generate Mermaid visualization
./scripts/generate-graph-viz.js > registry/modules-graph.mmd

# View in GitHub or Mermaid editor
```

### Graph Statistics
```bash
# Total modules
jq '.graph.metadata.total_modules' registry/modules-graph.json

# Production ready count
jq '.graph.metadata.production_ready' registry/modules-graph.json

# Most dependencies
jq '.graph.nodes | to_entries | 
  map({key: .key, deps: (.value.dependencies | length)}) | 
  sort_by(.deps) | reverse | .[0:5]' registry/modules-graph.json
```

## Integration with Catalog

The human-readable catalog (`doc/cim_modules_catalog.md`) is kept in sync:
- Status indicators added to module names
- Production count updated
- Git information available via graph queries

## Benefits

1. **Real-time Tracking**: Always know latest commits
2. **Dependency Analysis**: Understand module relationships
3. **Version Management**: Track all module versions
4. **Production Readiness**: Clear status visibility
5. **Automated Updates**: No manual maintenance
6. **Query Flexibility**: Multiple ways to access data

## Future Enhancements

- [ ] GraphQL API for queries
- [ ] Web dashboard visualization
- [ ] Breaking change detection
- [ ] Compatibility matrix generation
- [ ] Automated dependency updates