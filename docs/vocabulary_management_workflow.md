# Vocabulary Management Workflow

This document explains the vocabulary management workflow defined in `vocabulary.mdc`, which follows the path:

```
/notes -> /docs -> /vocabulary -> /code
```

![Generated-$(date +%Y-%m-%d)](https://img.shields.io/badge/Generated-$(date +%Y-%m-%d)-blue)

## Workflow Overview

The vocabulary management system follows a unidirectional flow of knowledge:

1. **Notes** (`/notes`) - Raw information, research findings, and initial observations
2. **Documentation** (`/docs`) - Structured documentation derived from notes
3. **Vocabulary** (`/vocabulary`) - Standardized terminology extracted from documentation
4. **Code** (`/cim/src`) - Implementation that reflects the vocabulary

This workflow ensures consistency across the entire system, from initial research to final implementation.

## Process Steps

### Step 1: Notes to Documentation

Notes in `/notes` directory are analyzed to create structured documentation in `/docs`:

1. Raw information is captured in notes
2. Key concepts and terms are identified
3. Notes are structured and organized into documentation
4. Documentation preserves and expands on the knowledge in notes

**Tools:**
- `scripts/analyze_notes_vocabulary_alignment.sh` - Analyze how well notes are reflected in documentation
- `scripts/vocabulary_manager.sh extract-terms` - Extract potential vocabulary terms from notes

### Step 2: Documentation to Vocabulary

Documentation in `/docs` is analyzed to create formal vocabulary in `/vocabulary/domains`:

1. Key terms are extracted from documentation
2. Terms are categorized according to vocabulary.mdc rules
3. Terms are organized into domains
4. Relationships between terms are identified
5. Taxonomies are created to organize related terms

**Tools:**
- `scripts/vocabulary_manager.sh create-vocabulary` - Generate vocabulary template from documentation
- `scripts/standardize_vocabulary_enhanced.sh` - Validate vocabulary against vocabulary.mdc rules

### Step 3: Vocabulary to Code

Vocabulary in `/vocabulary/domains` is used to create code in `/cim/src`:

1. Domain objects from vocabulary become classes/structs in code
2. Relationships guide the design of interactions between components
3. Code references in vocabulary link terms to specific implementations
4. Code maintains the same terminology as defined in vocabulary

**Tools:**
- `scripts/workflow_analyzer.sh` - Analyze how well code implements vocabulary
- `scripts/update_vocabulary_dashboard.sh` - Update metrics on vocabulary quality and implementation

## Term Categories

According to vocabulary.mdc, terms are categorized into:

### Domain Objects
- **Entity** - An object with a distinct identity
- **Aggregate** - A cluster of domain objects treated as a unit
- **ValueObject** - An object that describes characteristics but has no identity
- **Policy** - A rule or condition that guides behavior
- **Command** - An operation or action request
- **Query** - A request for information
- **Event** - Something that happened
- **Service** - An operation without state

### Technical Concepts
- **Pattern** - A reusable solution to a common problem
- **Architecture** - A system's high-level structure
- **Protocol** - Rules governing data exchange
- **Algorithm** - A step-by-step procedure for calculations

### Business Concepts
- **Process** - A series of actions to achieve a result
- **Rule** - A directive that guides decisions
- **Workflow** - A sequence of operations

### Cross-Cutting Terms
- **Security** - Protection against threats
- **Configuration** - Settings that control behavior
- **Monitoring** - Observation and tracking of system state

## Relationship Types

Relationships between terms follow these types:

### Hierarchical Relationships
- **IS_A** - Inheritance relationship
- **PART_OF** - Composition relationship
- **CONTAINS** - Container relationship
- **EXTENDS** - Extension relationship

### Functional Relationships
- **MANAGES** - Management relationship
- **PROCESSES** - Processing relationship
- **VALIDATES** - Validation relationship
- **CONFIGURES** - Configuration relationship

### Temporal Relationships
- **PRECEDES** - Sequential relationship (before)
- **FOLLOWS** - Sequential relationship (after)
- **TRIGGERS** - Causal relationship
- **DEPENDS_ON** - Dependency relationship

## Using the Vocabulary Manager

The `vocabulary_manager.sh` script provides a unified interface for managing the vocabulary workflow:

```bash
./scripts/vocabulary_manager.sh [command]
```

Available commands:
- `validate` - Check vocabulary compliance
- `analyze` - Analyze notes-vocabulary alignment
- `workflow-check` - Analyze workflow from notes to code
- `update-dashboard` - Update the vocabulary quality dashboard
- `extract-terms` - Extract potential terms from notes
- `create-vocabulary` - Generate a vocabulary template from notes/docs
- `help` - Display help message
- `all` - Run all checks and updates

## Best Practices

1. **Maintain Flow Direction** - Always follow the notes → docs → vocabulary → code flow
2. **Consistent Terminology** - Use the same terms across all stages
3. **Regular Analysis** - Run analysis tools frequently to ensure alignment
4. **Update Dashboard** - Keep the vocabulary quality dashboard updated
5. **Balanced Categories** - Maintain a balanced distribution of term categories
6. **Domain Alignment** - Ensure domain structure in notes aligns with vocabulary domains
7. **Code References** - Include code references for all vocabulary terms
8. **Primary Taxonomies** - Use primary taxonomies to organize terms consistently

## Metrics and Goals

The vocabulary management system tracks several metrics:

| Metric | Target | Description |
|--------|--------|-------------|
| Compliance | 100% | Vocabulary files should comply with vocabulary.mdc rules |
| Term Alignment | ≥85% | Terms in vocabulary should reflect content in notes |
| Structure Alignment | ≥95% | Domain structure should be consistent |
| Notes → Docs Flow | ≥75% | Terms from notes should appear in documentation |
| Docs → Vocabulary Flow | ≥80% | Terms from docs should appear in vocabulary |
| Vocabulary → Code Flow | ≥70% | Terms from vocabulary should be implemented in code |
| Category Balance | ≥80% | Term categories should be balanced |
| Relationship Type Balance | ≥75% | Relationship types should be balanced |
| Code Reference Coverage | ≥80% | Terms should have code references |

## Conclusion

Following this workflow ensures that knowledge flows consistently from research to implementation, maintains a ubiquitous language across all artifacts, and creates a traceable path from concepts to code. The various tools and scripts help automate the process and provide metrics to track progress and quality.

---

*This documentation was generated on $(date) using the vocabulary management system.* 