# CIM Project Structure

This repository serves as the root for the Composable Information Machine (CIM) ecosystem.

## Repository Organization

```
cim/                           # This repository - CIM root and documentation
├── .rules/                    # Project rules and standards
│   ├── main.mdc              # Main project rules
│   ├── mermaid-styling.md    # Mermaid graph styling guidelines
│   └── cim-conversation-model.mdc  # Claude conversation standards
├── doc/                       # Documentation root
│   ├── design/               # Architecture and design documents
│   ├── plan/                 # Implementation plans and roadmaps
│   ├── research/             # Research and theoretical foundations
│   ├── progress/             # Progress tracking
│   ├── cim_comprehensive_manual.md      # Main CIM manual
│   ├── cim_domain_knowledge_graph.json  # Domain graph data
│   ├── cim_domain_knowledge_graph.md    # Domain graph visualization
│   ├── cim_modules_catalog.md           # Complete module listing
│   ├── readme_cim_overview.md           # Platform overview
│   └── commit_signing_policy.md         # Security policy
├── claude.md                  # Claude AI interaction notes
├── readme.md                  # Main project README
├── contributing.md            # Contribution guidelines
├── changelog.md               # Project changelog
├── LICENSE                    # MIT License
└── .gitignore                # Git ignore rules

## CIM Module Repositories

The CIM ecosystem consists of multiple specialized repositories:

### Core Infrastructure
- `cim-domain` - Domain-driven design framework
- `cim-infrastructure` - Core infrastructure components
- `cim-start` - CIM project starter template
- `cim-leaf` - Edge computing nodes

### Domain Modules
- `cim-domain-*` - Business domain implementations
- `cim-subject-*` - Subject-specific modules
- `cim-operator-*` - Operator modules

### Integration & Tools
- `cim-integration-*` - External system integrations
- `cim-cli` - Command-line interface
- `cim-portal` - Web portal interface
- `cim-keys` - Key management

### AI & Intelligence
- `cim-agent-alchemist` - AI reasoning agent
- `cim-ontology-tool` - Ontology management

### Security & Persistence
- `cim-security` - Security framework
- `cim-persistence` - Data persistence layer
- `cim-ipld` - InterPlanetary Linked Data

## Development Workflow

1. **Documentation First**: All features start with documentation in this repository
2. **Domain Modeling**: Define domains using the knowledge graph
3. **Module Development**: Implement features in appropriate module repositories
4. **Integration**: Connect modules through the event mesh
5. **Testing**: Comprehensive testing in module repositories
6. **Deployment**: Use cim-infrastructure for deployment

## Getting Help

- Start with `doc/cim_comprehensive_manual.md`
- Review domain descriptions in `doc/cim_domain_knowledge_graph.md`
- Check module capabilities in `doc/cim_modules_catalog.md`
- Follow contributing guidelines in `contributing.md`

---
Copyright 2025 Cowboy AI, LLC.