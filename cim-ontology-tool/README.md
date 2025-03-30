# CIM Ontology Tool

A Rust-based MCP server for ontology extraction, analysis, and management for Composable Information Machines (CIMs).

## Overview

The CIM Ontology Tool is designed to scan folders of files, extract taxonomies and ontologies from them, and compare them against seed ontologies. It functions as an MCP (Model Context Protocol) server that integrates with Neo4j for graph database storage and can be called from Cursor and other MCP clients.

## Features

- Extract ontological terms and relationships from various file formats
- Build and manage taxonomies from source code and documentation
- Compare extracted ontologies against seed ontologies
- Store and query ontologies using Neo4j graph database
- Expose functionalities through MCP protocol endpoints
- Analyze and visualize ontological relationships
- Support version control and change tracking

## Communication Architecture

The CIM Ontology Tool uses NATS for all communication. This approach aligns with the broader CIM architecture that uses NATS as its primary messaging system.

### Why NATS?

NATS provides a lightweight, high-performance messaging system that enables:

- Subject-based messaging (matching the Domain-Driven Design approach)
- Distributed, scalable architecture
- Built-in request-reply patterns
- Durable message delivery with JetStream
- Clustering and fault tolerance

### MCP Protocol over NATS

Rather than using HTTP endpoints, MCP requests are sent as NATS messages to subjects following the pattern:

```
mcp.request.<operation>
```

Responses are delivered via the NATS request-reply mechanism, maintaining the same MCP protocol message format.

## Architecture

### System Architecture Diagram
```
┌────────────────────┐     ┌───────────────────┐
│                    │     │                   │
│  Cursor and other  │◄────┤   MCP Protocol    │
│   MCP Clients      │     │ (over NATS)       │
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
│         │  Domain Model    │   │ NATS-based │  │
│         │                  │   │ MCP Server │  │
│         └────────┬─────────┘   └───────────┘  │
│                  │                             │
└──────────────────┼─────────────────────────────┘
                   │
                   ▼
       ┌─────────────────────────┐
       │                         │
       │      Neo4j Graph        │
       │       Database          │
       │                         │
       └─────────────────────────┘
```

### Components

1. **MCP Protocol Interface**: Handles communication with MCP clients
2. **Extractor Module**: Extracts terms and relationships from files
3. **Analyzer Module**: Compares, analyzes, and generates metrics
4. **Domain Model**: Core entities representing ontological data
5. **Neo4j Integration**: Manages graph database operations
6. **MCP Server**: Implements the MCP protocol specification

## Requirements

### Functional Requirements

1. **Ontology Extraction**
   - Parse various file formats to extract terms, concepts, and relationships
   - Compare extracted terms against seed ontologies
   - Support for multiple natural languages
   - Handle code files, markdown docs, and structured data formats

2. **Ontology Management**
   - Store ontologies in Neo4j graph database
   - Support CRUD operations on ontology elements
   - Version control for ontologies
   - Import/export capabilities (TTL, RDF, JSON-LD)

3. **MCP Protocol Integration**
   - Expose MCP-compliant endpoints
   - Support synchronous and asynchronous requests
   - Implement result streaming for large operations
   - Provide progress reporting for long-running tasks

4. **Analysis & Visualization**
   - Generate relationship graphs
   - Calculate ontology metrics and statistics
   - Provide consistency and completeness checks
   - Support interactive exploration of ontologies

### Non-Functional Requirements

1. **Performance**
   - Handle large repositories efficiently
   - Parallel processing of files and directories
   - Efficient Neo4j query patterns
   - Connection pooling and caching strategies

2. **Scalability**
   - Support for distributed processing
   - Horizontal scaling capabilities
   - Resource usage optimization

3. **Security**
   - Authentication and authorization
   - Secure communication channels
   - Data validation and sanitization

4. **Reliability**
   - Graceful error handling
   - State recovery mechanisms
   - Comprehensive logging

## Implementation Plan

### Phase 1: Core Infrastructure
1. Set up Neo4j connectivity
2. Implement MCP server foundation
3. Design domain model
4. Create basic API endpoints

### Phase 2: Extraction Capabilities
1. Implement file traversal
2. Build parsing engines for different file types
3. Develop NLP processing pipeline
4. Create ontology comparison logic

### Phase 3: Analysis Features
1. Implement ontology metrics
2. Develop visualization capabilities
3. Create advanced query patterns
4. Build recommendation system

### Phase 4: Integration & Deployment
1. Finalize MCP protocol compliance
2. Implement authentication and authorization
3. Create deployment configuration
4. Develop documentation and examples

## Usage

```rust
// Example usage will be provided as the API stabilizes
```

## License

MIT 