# Decision: File Structure and Workflow Standardization

- **Date**: 2025-03-30
- **Status**: Implemented
- **Decision Type**: Process
- **Domain**: All
- **Context**: Project organization
- **Decision Makers**: Development Team

## Context and Problem Statement

As the CIM project grows in complexity and team size, we need a standardized file structure and workflow to ensure:
1. Consistent code organization across modules
2. Clear documentation and decision tracking
3. Efficient onboarding for new team members
4. Reduced friction in code reviews and collaboration

## Decision

We've established standardized file structures and workflows for the CIM project:

### File Structure

```
/cim
├── cim-ontology-tool/       # Main project directory
│   ├── src/                 # Source code
│   │   ├── analyzer/        # Ontology analysis
│   │   ├── cli/             # Command-line interface
│   │   ├── events/          # Event-driven architecture
│   │   ├── extractor/       # Ontology extraction
│   │   ├── mcp/             # Model Context Protocol
│   │   ├── ontology/        # Ontology models
│   │   ├── storage/         # Storage backends
│   │   └── utils/           # Utility functions
│   ├── examples/            # Usage examples
│   ├── tests/               # Integration tests
│   └── Cargo.toml           # Project manifest
├── flake.nix               # Nix flake configuration
├── modules/                # Modular components
│   ├── cursor-rules/       # Cursor rules as a NixOS-style module
│   │   ├── default.nix     # Module definition
│   │   ├── config.nix      # Module configuration
│   │   └── update-rules.sh # Rule management script
│   └── default.nix         # Module entry point
├── .cursor/                # Cursor-specific configuration
│   └── rules/              # Cursor rules
│       ├── code-policy.mdc # Coding policies
│       ├── design-patterns.mdc # Design patterns
│       └── ... other rules
├── notes/                  # Documentation and notes
│   ├── decisions/          # Architecture decision records
│   ├── architecture.md     # Architecture overview
│   ├── vocabulary.md       # Domain vocabulary and terms
│   └── knowledge_graph.md  # Knowledge graph documentation
└── README.md              # Project overview
```

### Workflow Standards

1. **Code Structure**:
   - Each module has a clear single responsibility
   - Public API is exposed through the module's `mod.rs` file
   - Private implementations in separate files within the module
   - Tests co-located with the code they test

2. **Documentation**:
   - All public functions and types must be documented
   - Architecture decisions are recorded in the `notes/decisions/` directory
   - Domain vocabulary maintained in `notes/vocabulary.md`
   - System architecture documented in `notes/architecture.md`

3. **Version Control**:
   - Descriptive commit messages using conventional commits format
   - Feature branches for new functionality
   - Pull requests for all changes
   - Code review required before merging

4. **CI/CD Pipeline**:
   - Automated tests for all PRs
   - Documentation generation
   - Linting and formatting checks
   - Reproducible builds via Nix

5. **Cursor Rules Management**:
   - Cursor rules are treated as a NixOS-style module
   - Rules are defined declaratively in `modules/cursor-rules/config.nix`
   - Rules are managed using the `modules/cursor-rules/update-rules.sh` script
   - Changes to rules should be made through the module configuration

## Benefits

1. **Consistency**: Standardized structure makes it easier to navigate the codebase
2. **Documentation**: Decision records provide context for architectural choices
3. **Maintainability**: Clear module boundaries and responsibilities
4. **Onboarding**: New team members can quickly understand the project structure
5. **Quality**: Automated CI/CD ensures code meets standards
6. **Modularity**: NixOS-style modules improve encapsulation and reusability

## Implementation

The standardized structure and workflow have been implemented:

1. Code has been reorganized according to the new structure
2. Documentation templates have been created
3. CI/CD pipeline has been set up with GitHub Actions
4. Decision records are now being maintained for significant architectural changes
5. Cursor rules have been modularized using a NixOS-style module

## References

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Architecture Decision Records](https://adr.github.io/)
- [Cargo Project Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)
- [NixOS Modules](https://nixos.wiki/wiki/NixOS_modules) 