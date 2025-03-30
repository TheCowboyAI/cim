# CIM Ontology Tool

## Overview

The CIM Ontology Tool is a Rust-based MCP (Model Context Protocol) server designed for ontology extraction, analysis, and management for Composable Information Machines (CIMs). It serves as a specialized component that enhances the project's vocabulary management capabilities by providing advanced semantic analysis and graph database integration.

## Purpose

The tool addresses several key challenges in vocabulary and ontology management:

1. **Automated Extraction**: Extracts ontological terms and relationships from various file formats
2. **Semantic Analysis**: Analyzes relationships between terms and concepts
3. **Comparison**: Compares extracted ontologies against seed ontologies
4. **Storage**: Stores ontological data in a Neo4j graph database for efficient querying
5. **Visualization**: Enables visualization of ontological relationships
6. **Integration**: Exposes functionality through MCP protocol for client integration

## Architecture

The CIM Ontology Tool follows a modular architecture with recent updates implementing NATS-based messaging:

```
┌────────────────────┐     ┌───────────────────┐
│                    │     │                   │
│  Cursor and other  │◄────┤   MCP Protocol    │
│   MCP Clients      │     │    Interface      │
│                    │     │                   │
└────────────────────┘     └─────────┬─────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────┐
│                                                │
│               cim-ontology-tool                │
│                                                │
│  ┌─────────────┐  ┌─────────────┐  ┌────────┐  │
│  │             │  │             │  │        │  │
│  │  Extractor  │  │  Analyzer   │  │  API   │  │
│  │   Module    │  │   Module    │  │ Module │  │
│  │             │  │             │  │        │  │
│  └──────┬──────┘  └──────┬──────┘  └───┬────┘  │
│         │                │             │       │
│         └────────┬───────┘             │       │
│                  │                     │       │
│         ┌────────▼─────────┐   ┌───────▼────┐  │
│         │                  │   │            │  │
│         │  Domain Model    │   │ MCP Server │  │
│         │                  │   │   (NATS)   │  │
│         └────────┬─────────┘   └───────────┘  │
│                  │                             │
└──────────────────┼─────────────────────────────┘
                   │                  ▲
                   ▼                  │
       ┌─────────────────────────┐    │
       │                         │    │
       │      Neo4j Graph        │    │
       │       Database          │    │
       │                         │    │
       └─────────────────────────┘    │
                                      │
                                      │
┌──────────────────────────────────────────────┐
│                                              │
│               cim-client                     │
│                                              │
│  ┌─────────────┐      ┌─────────────────┐    │
│  │             │      │                 │    │
│  │  MCP Client │      │ User Interfaces │    │
│  │  (NATS)     │      │ (CLI, GUI)      │    │
│  │             │      │                 │    │
│  └─────────────┘      └─────────────────┘    │
│                                              │
└──────────────────────────────────────────────┘
```

### Key Components

1. **MCP Protocol Interface**: 
   - Handles communication with MCP clients like Cursor
   - Supports synchronous and asynchronous operations
   - Implements the MCP protocol specification

2. **Extractor Module**:
   - Parses various file formats (code, markdown, etc.)
   - Extracts terms, concepts, and relationships
   - Supports different natural languages
   - Handles code files, documentation, structured data

3. **Analyzer Module**:
   - Compares extracted ontologies against seed ontologies
   - Generates ontology metrics and statistics
   - Provides consistency and completeness checks
   - Identifies gaps and inconsistencies

4. **Domain Model**:
   - Represents ontological entities and relationships
   - Maintains ontology structure
   - Supports version control for ontologies
   - Maps between different ontological representations

5. **Neo4j Integration**:
   - Stores ontologies in Neo4j graph database
   - Implements efficient query patterns
   - Supports CRUD operations on ontology elements
   - Enables complex graph queries and traversals

6. **MCP Server (NATS-Based)**:
   - Implements the MCP protocol over NATS messaging
   - Provides enhanced scalability and reliability
   - Enables asynchronous communication patterns
   - Supports request-response and publish-subscribe patterns
   - Handles connection management and messaging

7. **CIM Client (New Component)**:
   - Provides a standalone client for connecting to the MCP server
   - Implements the MCP protocol over NATS
   - Features both CLI and GUI interfaces
   - Supports ontology browsing, creation, and management
   - Enables easier integration with other systems

## Integration with Vocabulary Workflow

The CIM Ontology Tool enhances the existing vocabulary management workflow:

1. **Document Processing**:
   - Automatically scans documents in `/notes/incoming/`
   - Extracts ontological terms and relationships
   - Provides more advanced semantic analysis

2. **Knowledge Organization**:
   - Enhances claim extraction with semantic understanding
   - Identifies relationships between claims and terms
   - Helps organize information into appropriate categories

3. **Vocabulary Management**:
   - Provides advanced analysis of vocabulary terms
   - Identifies relationships between terms
   - Compares vocabulary against seed ontologies
   - Suggests improvements and identifies gaps

4. **Knowledge Graph**:
   - Stores vocabulary in a Neo4j graph database
   - Enables complex queries across the knowledge graph
   - Provides visualization capabilities
   - Maintains relationships between terms

5. **Integration Points**:
   - Works alongside the vocabulary manager
   - Exposes MCP endpoints for client integration
   - Provides API for vocabulary analysis tools
   - Stores extracted ontologies in the graph database

## Implementation Plan

The implementation follows a phased approach:

### Phase 1: Core Infrastructure (Completed)
- Set up Neo4j connectivity
- Implement MCP server foundation
- Design domain model
- Create basic API endpoints

### Phase 2: Extraction Capabilities (Completed)
- Implement file traversal
- Build parsing engines for different file types
- Develop NLP processing pipeline
- Create ontology comparison logic

### Phase 3: Analysis Features (In Progress)
- Implement ontology metrics
- Develop visualization capabilities
- Create advanced query patterns
- Build recommendation system

### Phase 4: Client Implementation (New - In Progress)
- Develop standalone NATS-based MCP client
- Implement CLI for command-line operations
- Create GUI for visual ontology management
- Support both core ontology operations and advanced features

### Phase 5: Integration & Deployment (Planned)
- Finalize MCP protocol compliance
- Implement authentication and authorization
- Create deployment configuration
- Develop documentation and examples

## Use Cases

The CIM Ontology Tool supports several key use cases:

1. **Ontology Extraction**:
   - Extract terms and relationships from code and documentation
   - Build comprehensive ontologies from existing materials
   - Identify domain-specific vocabularies

2. **Ontology Comparison**:
   - Compare extracted ontologies against seed ontologies
   - Identify gaps and inconsistencies
   - Track ontology evolution over time

3. **Knowledge Graph Querying**:
   - Query the knowledge graph to find relationships
   - Explore connections between terms
   - Identify clusters of related concepts

4. **Ontology Visualization**:
   - Visualize ontological relationships
   - Generate diagrams of term relationships
   - Create interactive visualizations for exploration

5. **Integration with Development Workflow**:
   - Analyze code for terminology consistency
   - Check documentation against established vocabulary
   - Ensure ubiquitous language usage

6. **Collaborative Ontology Management (New)**:
   - Use the dedicated client to manage ontologies collaboratively
   - Access ontology tools from command line or GUI interface
   - Browse, create, edit, and delete ontologies and terms

## Getting Started

### Setting up the Server

To use the CIM Ontology Tool:

1. Ensure Neo4j is installed and running
2. Build the tool using `cargo build`
3. Configure the connection to Neo4j
4. Start the MCP server
5. Connect through Cursor or other MCP clients

### Using the CIM Client (New)

The dedicated client provides an easy way to interact with the CIM Ontology Tool:

1. Build the client: `cd cim-client && cargo build`
2. Use the CLI: `./target/debug/cim-client-cli <command> <arguments>`
3. Or launch the GUI: `./target/debug/cim-client-gui`
4. Configure connection to NATS server in the settings

## Future Work

Planned enhancements include:

1. **Advanced NLP Capabilities**:
   - Enhanced semantic understanding
   - Domain-specific language processing
   - Multiple language support

2. **Integration with AI Models**:
   - LLM-based ontology extraction
   - AI-assisted relationship identification
   - Automated ontology merging

3. **Enhanced Visualization**:
   - Interactive exploration tools
   - Custom visualization templates
   - Real-time collaborative visualization

4. **Workflow Automation**:
   - Automated ontology extraction on commit
   - Scheduled ontology analysis
   - Continuous vocabulary improvement

5. **Advanced Client Features (New)**:
   - Real-time collaboration capabilities
   - Custom visualization templates
   - Integration with other development tools
   - Enhanced search and filtering

## Conclusion

The CIM Ontology Tool represents a significant enhancement to the vocabulary management capabilities of the CIM project. By providing advanced ontology extraction, analysis, and storage capabilities, it enables more sophisticated vocabulary management and knowledge organization, ultimately contributing to better alignment between documentation, vocabulary, and code implementation. 

The newly added NATS-based messaging architecture and dedicated client implementation further enhance these capabilities by providing improved scalability, reliability, and user experience through dedicated interfaces for interacting with the ontology management system. 