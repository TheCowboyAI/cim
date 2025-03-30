# Composable Information Machine (CIM)

A continuously improving, distributed system that transforms scattered information into organized, actionable, reactive knowledge while keeping it secure and accessible across your digital world.

📊 **[View Project Dashboard](docs/dashboard.md)** - Track our progress, metrics, and next steps

## Project Overview

CIM Goals:

- THIS SYSTEM MUST BE REPRODUCIBLE
- Represent information in the information world, in both memory and persistence layers
- Create a system of information that is portable and can be used by any device
- Create a system of information that is secure and can be used by any authorized user
- Create a system that can start from nothing and build itself up over time
- Create a system of information that is scalable and can be used by any size of business
- Use AI Tools to make it easier to create, share, and use information

## Project Structure

The project follows a standardized file structure and knowledge flow process:

```
/
├── notes/                  # Raw research and initial documentation
│   ├── incoming/           # Staging area for new documents
│   ├── claims/             # Extracted claims from documents
│   └── [subdirectories]    # Organized by document type
│
├── docs/                   # Formalized documentation
│   ├── domain/             # Domain-specific documentation
│   └── [core files]        # Key project documentation
│
├── vocabulary/             # Vocabulary definitions
│   ├── domains/            # Domain-specific vocabularies
│   ├── taxonomies/         # Categorization hierarchies
│   ├── ontologies/         # Concept relationships
│   └── knowledge/          # Knowledge graph implementation
│
├── scripts/                # Utility scripts
│
├── modules/                # Implementation modules
│
└── cim-ontology-tool/      # Specialized tool for ontology management
```

## Knowledge Flow Process

We follow a structured knowledge flow process:

1. **Document Creation**: New documents are placed in `/notes/incoming/`
2. **Document Processing**: Documents are analyzed and claims are extracted
3. **Claim Extraction**: Individual claims are stored in `/notes/claims/`
4. **Knowledge Organization**: Processed information is organized into appropriate notes subdirectories
5. **Documentation**: Key information is consolidated into formal documentation in `/docs/`
6. **Vocabulary Management**: Domain vocabulary is extracted and managed in `/vocabulary/`
7. **Code Implementation**: Implementation based on the established vocabulary

The complete flow: `/notes` → `/docs` → `/vocabulary` → `/code`

## Current Projects

### CIM Ontology Tool

The CIM Ontology Tool is a Rust-based MCP server for ontology extraction, analysis, and management. It integrates with Neo4j for graph database storage and can be called from Cursor and other MCP clients.

Key features:
- Extract ontological terms and relationships from various file formats
- Build and manage taxonomies from source code and documentation
- Compare extracted ontologies against seed ontologies
- Store and query ontologies using Neo4j graph database
- Expose functionalities through MCP protocol endpoints
- Analyze and visualize ontological relationships

## Getting Started

1. Review the [Project Brief](docs/projectBrief.md) to understand core requirements
2. Explore the [Domain Context](docs/domainContext.md) for current focus areas
3. Check the [Progress](docs/progress.md) for current status and next steps
4. Follow the [File Structure and Workflow](docs/file_structure_and_workflow.md) guidelines
5. See the [Vocabulary Management Workflow](docs/vocabulary_management_workflow.md) for details on the knowledge flow process

## Out of Scope

- Make a new Operating System
- Make a new message transport system
- Make a new data storage system
- Make a new data query system
- Make a new data alerting system
- Make a new data reporting system

## Intent

THIS IS A RESEARCH PROJECT DETERMINING THE NATURE OF A CIM

- Use what is already available and make it better
- Contribute to Open Source projects
- Make sharing information better and more secure
- Have a permanent, immutable system of record
- Use MANY sources of truth and consolidate them by domain

## Documentation

- [Project Brief](docs/projectBrief.md) - Core requirements and scope
- [Project Context](docs/projectContext.md) - Project existence and operation
- [Domain Context](docs/domainContext.md) - Current state and decisions
- [System Patterns](docs/systemPatterns.md) - Technical architecture
- [Tech Context](docs/techContext.md) - Technical environment
- [Progress](docs/progress.md) - Project status
- [File Structure and Workflow](docs/file_structure_and_workflow.md) - Standardized file structure and knowledge flow
- [Vocabulary Management Workflow](docs/vocabulary_management_workflow.md) - Knowledge flow process
- [CIM Ontology Tool](docs/cim_ontology_tool.md) - Specialized ontology management tool
