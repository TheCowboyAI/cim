# Contributing to CIM

Thank you for your interest in contributing to the Composable Information Machine (CIM)! We welcome contributions from the community.

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct: be respectful, constructive, and professional in all interactions.

## How to Contribute

### Reporting Issues

- Check if the issue already exists in the [issue tracker](https://github.com/thecowboyai/cim/issues)
- Provide a clear description of the problem
- Include steps to reproduce the issue
- Include your environment details (OS, language versions, etc.)

### Submitting Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Run formatting and linting for your code language
7. Commit your changes with clear messages (see commit message format below)
8. Sign your commits (see commit signing policy)
9. Push to your fork
10. Open a pull request with a clear description

### Development Setup

#### Prerequisites

- Git with SSH commit signing configured (see `doc/commit_signing_policy.md`)
- Development environment for your chosen module language (Rust, Python, Node.js, etc.)
- NATS server for testing messaging functionality

#### Getting Started

```bash
# Clone the repository
git clone https://github.com/thecowboyai/cim.git
cd cim

# Review the documentation
cat doc/cim_comprehensive_manual.md
cat doc/cim_domain_knowledge_graph.md

# Choose a module to work on
ls -la ~/git/thecowboyai/cim-*
```

### Coding Standards

- Follow language-specific conventions for the module you're working on
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions focused and small
- Write comprehensive tests
- Document public APIs
- Follow the project's architectural principles (see `.rules/`)

### Testing

- Write unit tests for new functionality
- Add integration tests for cross-module features
- Ensure existing tests continue to pass
- Test edge cases and error conditions
- Test with NATS messaging when applicable

### Documentation

- Update documentation for any API changes
- Add examples for new features
- Keep module README files up to date
- Document breaking changes in CHANGELOG.md
- Follow documentation standards in `.rules/`

### Commit Messages

Follow conventional commit format:

```
type(scope): subject

body (optional)

footer (optional)

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions or modifications
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

Example:
```
feat(domain): add conceptual spaces integration

Implements conceptual spaces theory for semantic understanding,
including quality dimensions and similarity metrics.

Closes #123
```

### Commit Signing

All commits must be signed. See `doc/commit_signing_policy.md` for setup instructions.

## Areas for Contribution

### Core Infrastructure
- **Event System**: Enhance event correlation and causation handling
- **Domain Models**: Create new domain aggregates and entities
- **Persistence**: Implement new storage backend adapters
- **Security**: Improve authentication and authorization

### Modules
- **New Domains**: Add domain modules for different business contexts
- **AI Agents**: Enhance Alchemist and create new AI capabilities
- **Integrations**: Build adapters for external systems
- **Edge Computing**: Improve CIM Leaf node functionality

### Documentation
- **Tutorials**: Create step-by-step guides for common tasks
- **Examples**: Add real-world implementation examples
- **API Documentation**: Improve module API documentation
- **Architecture**: Document design decisions and patterns

### Tooling
- **CLI Tools**: Enhance cim-cli functionality
- **Development Tools**: Create debugging and monitoring utilities
- **Deployment**: Improve deployment automation
- **Testing**: Add testing frameworks and utilities

## Module Development

When creating or modifying CIM modules:

1. Follow the established module naming convention: `cim-<category>-<name>`
2. Include a comprehensive README.md
3. Implement standard CIM interfaces
4. Provide examples and tests
5. Document all public APIs
6. Follow domain-driven design principles

## Questions?

- Open an issue for questions
- Review existing documentation in `/doc`
- Check module-specific documentation
- Join community discussions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---
Copyright 2025 Cowboy AI, LLC.