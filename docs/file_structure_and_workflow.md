# File Structure and Knowledge Flow

## Overview

This document describes the standardized file structure and knowledge flow process for the CIM project. The file structure supports the systematic management of information from initial research to code implementation, while the knowledge flow process defines how information moves between different components of the system.

## Directory Structure

The CIM project follows a structured organization with dedicated directories for different types of information:

```
/
├── notes/                  # Raw research and initial documentation
│   ├── incoming/           # Staging area for new documents
│   ├── claims/             # Extracted claims from documents
│   ├── research/           # Research findings and evaluations
│   ├── decisions/          # Technical and architectural decisions
│   ├── meetings/           # Meeting notes and outcomes
│   ├── reviews/            # Code and design review notes
│   └── explorations/       # Technical explorations and experiments
│
├── docs/                   # Formalized documentation
│   ├── domain/             # Domain-specific documentation
│   ├── projectBrief.md     # Core requirements and project scope
│   ├── projectContext.md   # Project existence and operation
│   ├── domainContext.md    # Current state and decisions
│   ├── systemPatterns.md   # Technical architecture
│   ├── techContext.md      # Technical environment
│   └── progress.md         # Project status
│
├── vocabulary/             # Vocabulary definitions
│   ├── domains/            # Domain-specific vocabularies
│   ├── taxonomies/         # Categorization hierarchies
│   ├── ontologies/         # Concept relationships
│   └── knowledge/          # Knowledge graph implementation
│
├── scripts/                # Utility scripts
│   └── vocabulary_manager.sh  # Vocabulary management tool
│
└── modules/                # Implementation modules
```

## Knowledge Flow Process

The knowledge flow process follows a structured path from initial research to code implementation:

```mermaid
graph TD
    A[Document Creation] -->|New Document| B[/notes/incoming/]
    B -->|Document Processing| C[Claim Extraction]
    C -->|Extracted Claims| D[/notes/claims/]
    D -->|Claim Organization| E[/notes/ Subdirectories]
    E -->|Knowledge Consolidation| F[/docs/]
    F -->|Vocabulary Extraction| G[/vocabulary/]
    G -->|Implementation| H[Code]
    
    subgraph "Knowledge Flow Process"
    A
    B
    C
    D
    E
    F
    G
    H
    end
```

### Process Steps

1. **Document Creation**
   - New research, decisions, meeting notes, etc. are created
   - Documents follow required format and naming conventions
   - Files are placed in `/notes/incoming/` directory

2. **Document Processing**
   - Documents in `/notes/incoming/` are analyzed
   - The vocabulary manager processes these documents
   - Terms and concepts are extracted
   - Claims are identified and extracted

3. **Claim Extraction**
   - Individual claims are extracted from documents
   - Claims are stored in `/notes/claims/` directory
   - Claim files maintain provenance to source documents
   - Claims serve as a foundation for decision-making

4. **Knowledge Organization**
   - Processed documents are organized into appropriate `/notes/` subdirectories
   - Organization is based on document type and content
   - Cross-references are maintained between related documents
   - Knowledge graph connections are established

5. **Documentation Consolidation**
   - Key information from notes is consolidated into formal docs
   - Core documentation files are updated
   - Documentation maintains consistency with notes
   - Knowledge is structured according to domain boundaries

6. **Vocabulary Extraction**
   - Domain vocabulary is extracted from documentation
   - Terms are categorized according to vocabulary rules
   - Relationships between terms are established
   - Vocabulary is structured into domains, taxonomies, and ontologies

7. **Code Implementation**
   - Implementation is guided by the established vocabulary
   - Code reflects the ubiquitous language from vocabulary
   - System architecture follows the domain model
   - Implementation decisions are documented

## File Naming Conventions

### Notes Files
- Format: `YYYY-MM-DD-brief-title.md`
- Example: `2024-03-30-vocabulary-workflow-standardization.md`

### Claims Files
- Format: `YYYY-MM-DD-source-document-claims.md`
- Example: `2024-03-30-vocabulary-workflow-standardization-claims.md`

### Documentation Files
- Core files: Standardized names (e.g., `projectBrief.md`)
- Supporting docs: Descriptive names (e.g., `vocabulary_management_workflow.md`)

### Vocabulary Files
- Domain files: `<domain-name>.md`
- Taxonomy files: `<taxonomy-name>.md`
- Ontology files: `<ontology-name>.md`

## Document Templates

### Notes Template
```markdown
# <Title>

## Context
<Why this note exists>

## Details
<Main content>

## Decisions
<Any decisions made>

## Next Steps
<Follow-up actions>

## References
<Links to related notes/issues>
```

### Claims Template
```markdown
# Claims from <Source Document>

## Source
- Document: <Document Name>
- Date: <Extraction Date>

## Claims
1. <Claim 1>
   - Context: <Context from source>
   - Importance: <High/Medium/Low>
   - Category: <Technical/Business/Process/etc.>

2. <Claim 2>
   ...
```

## Tooling Support

### Vocabulary Manager

The `vocabulary_manager.sh` script provides a unified interface for managing the knowledge flow:

```bash
# Usage
./scripts/vocabulary_manager.sh <command> [options]

# Commands
extract-claims         # Extract claims from documents
organize               # Organize documents into directories
update-docs            # Update documentation based on notes
extract-vocabulary     # Extract vocabulary from documentation
analyze-alignment      # Analyze vocabulary alignment
generate-model         # Generate code model from vocabulary
```

### CIM Ontology Tool

The CIM Ontology Tool enhances the knowledge flow by:
- Automatically extracting terms and relationships
- Storing ontologies in a Neo4j graph database
- Providing comparison against seed ontologies
- Enabling visualization of ontological relationships
- Exposing functionality through MCP protocol

## Quality Assurance

### Metrics

The knowledge flow process is monitored using several metrics:

1. **Document Processing**
   - Number of documents processed
   - Processing time per document
   - Claim extraction accuracy

2. **Claim Quality**
   - Number of claims extracted
   - Claim categorization accuracy
   - Claim integration rate

3. **Knowledge Organization**
   - Cross-reference completeness
   - Directory organization compliance
   - File naming convention adherence

4. **Documentation Quality**
   - Documentation completeness
   - Alignment with notes
   - Consistency across documents

5. **Vocabulary Quality**
   - Term extraction accuracy
   - Relationship identification accuracy
   - Ontology completeness

### Quality Checks

Regular quality checks are performed to ensure the integrity of the knowledge flow:

1. **File Structure Validation**
   - Verify directory structure compliance
   - Check file naming conventions
   - Validate file formats

2. **Process Compliance**
   - Verify adherence to knowledge flow process
   - Check document processing completeness
   - Validate claim extraction quality

3. **Content Quality**
   - Review documentation for completeness
   - Check vocabulary alignment with notes
   - Validate knowledge graph connections

## Best Practices

1. **Regular Processing**: Process incoming documents regularly
2. **Claim Review**: Review and validate extracted claims
3. **Cross-Referencing**: Maintain cross-references between related documents
4. **Consistency**: Ensure consistency across notes, docs, and vocabulary
5. **Metadata**: Include proper metadata in all files
6. **Organization**: Keep files organized in appropriate directories
7. **Automation**: Use available tools for repetitive tasks
8. **Version Control**: Use version control for all files
9. **Feedback Loop**: Continuously improve the process based on feedback
10. **Documentation**: Document process changes and improvements

## Future Enhancements

Planned enhancements to the file structure and knowledge flow process:

1. **Enhanced Automation**
   - Automated document processing
   - AI-assisted claim extraction
   - Automated cross-referencing

2. **Advanced Analysis**
   - Content similarity analysis
   - Gap identification
   - Duplicate detection

3. **Integration Improvements**
   - Tighter integration with CI/CD pipeline
   - Real-time knowledge graph updates
   - Enhanced visualization tools

4. **Process Refinement**
   - Streamlined claim review process
   - Improved document classification
   - Enhanced quality metrics

## Conclusion

The standardized file structure and knowledge flow process provide a solid foundation for managing information throughout the CIM project lifecycle. By following these standards, we ensure that knowledge flows consistently from initial research to code implementation, maintaining traceability and alignment across all project artifacts. 