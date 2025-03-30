# Knowledge Graph Implementation

This document describes the implementation of the CIM Knowledge Graph, which visualizes the relationships between terms in our vocabulary.

## Overview

The knowledge graph is a visual representation of terms and their relationships across all domains. It helps understand:

- Term hierarchies
- Cross-domain relationships
- Usage patterns
- Implementation dependencies

We've implemented two types of knowledge graphs:

1. **Main Knowledge Graph**: Shows all terms and relationships across domains
2. **Domain-Specific Graphs**: Show terms and relationships within each domain

## Implementation

The knowledge graphs are generated using Mermaid, a JavaScript-based diagramming tool that renders Markdown-inspired text into diagrams. Mermaid diagrams can be viewed directly in many Markdown viewers, including GitHub.

### Generator Scripts

We've created three shell scripts to generate the knowledge graphs:

1. `scripts/generate_knowledge_graph.sh`: The main script that runs both generators
2. `scripts/generate_mermaid_graph.sh`: Generates the main knowledge graph
3. `scripts/generate_domain_graphs.sh`: Generates domain-specific knowledge graphs

These scripts parse our vocabulary files in `vocabulary/domains/` and extract:
- Terms and their attributes (type, taxonomy, etc.)
- Relationships between terms

### Graph Structure

The knowledge graphs use the following structure:

- **Nodes**: Represent terms from our vocabulary
  - Colored by type (Entity, Value Object, etc.)
  - Labeled with term name and type
  - Tooltips show definitions

- **Edges**: Represent relationships between terms
  - Labeled with relationship type (Contains, Uses, etc.)
  - Directed from source to target

- **Clusters**: Group terms by domain

### Viewing the Graphs

The knowledge graphs can be viewed in any Markdown viewer that supports Mermaid diagrams, including:

- GitHub
- VS Code with Markdown Preview
- Obsidian
- JetBrains IDEs

## Usage

### Generating the Knowledge Graphs

To generate the knowledge graphs:

```bash
./scripts/generate_knowledge_graph.sh
```

This will create:
- `docs/knowledge_graph.md`: The main knowledge graph
- `docs/knowledge_graphs/`: Directory containing domain-specific knowledge graphs

### Interpreting the Graphs

- **Node color** indicates term type:
  - Blue: Entity
  - Green: Value Object
  - Pink: Aggregate
  - Yellow: Service
  - Cyan: Platform
  - Purple: Tool
  - Orange: Protocol
  - Light Blue: Framework

- **Edge labels** indicate relationship types:
  - Contains: Containment relationship
  - Uses: Usage dependency
  - Implements: Implementation relationship
  - Part-Of: Composition relationship
  - Provides: Capability provision
  - Defines: Definition relationship
  - Manages: Management relationship
  - Identifies: Identification relationship

## Benefits

The knowledge graph implementation provides several benefits:

1. **Visual Understanding**: Easy visualization of complex relationships
2. **Cross-Domain Analysis**: Identify relationships between different domains
3. **Gap Identification**: Identify missing relationships or inconsistencies
4. **Onboarding**: Help new team members understand the vocabulary
5. **Communication**: Facilitate discussions about the vocabulary
6. **Documentation**: Serve as living documentation of the vocabulary

## Future Enhancements

Potential future enhancements to the knowledge graph implementation:

1. **Interactive Visualization**: Implement more interactive visualization options
2. **Filtering**: Add more advanced filtering options
3. **Queries**: Implement a query language for the knowledge graph
4. **Validation**: Automatically validate the vocabulary for consistency
5. **Integration**: Integrate with other documentation tools 