# CIM Domain Context

## Current Focus
- Implementing core module prototypes
- Creating implementation patterns
- Developing case studies
- Building AI integration framework
- Standardizing vocabulary management workflow
- Maintaining structured knowledge flow

## Domain Specifics

### Knowledge Management Domain
- Status: Active Development
- Current Focus: Core module implementation and knowledge flow standardization
- Next Steps: Implement AI integration and enhance claim extraction

### Technical Domain
- Status: Active Development
- Current Focus: Core module prototype development
- Next Steps: Implement event-driven communication system

### Business Domain
- Status: Active Planning
- Current Focus: Case study development
- Next Steps: Implement domain-specific patterns

### Governance Domain
- Status: Active Planning
- Current Focus: Compliance documentation
- Next Steps: Implement security framework

### Organizational Domain
- Status: Active Development
- Current Focus: Implementation pattern documentation
- Next Steps: Develop testing framework

## Recent Changes
1. Formalized file structure with dedicated /notes/incoming and /notes/claims directories
2. Standardized knowledge workflow with defined processing stages
3. Updated scripts to support the standardized file structure
4. Implemented standardized vocabulary management workflow from notes to code
5. Created vocabulary_manager.sh as unified interface for vocabulary management
6. Enhanced vocabulary analysis scripts for notes-vocabulary alignment
7. Added workflow analysis to track knowledge flow across the system
8. Added comprehensive vocabulary structure with domains, ontologies, and taxonomies
9. Established Cowboy AI as primary implementation owner
10. Restructured documentation to follow vocabulary rules
11. Created project memory structure
12. Defined core domain categories
13. Developed strategic improvement plan
14. Transitioned from vocabulary to implementation phase
15. Replaced wasmCloud with generic WebAssembly runtime
16. Implemented knowledge graph visualization using Mermaid

## Active Decisions

### File Structure and Workflow Decisions
1. Created /notes/incoming directory for staging new documents
2. Created /notes/claims directory for storing extracted claims
3. Formalized knowledge flow process: /notes -> /docs -> /vocabulary -> /code
4. Developed comprehensive scripts to support the standardized workflow
5. Implemented automated claim extraction from incoming documents
6. Established metrics for measuring knowledge flow effectiveness

### Vocabulary Management Decisions
1. Implemented /notes -> /docs -> /vocabulary -> /code workflow for knowledge management
2. Created unified vocabulary_manager.sh script for vocabulary operations
3. Established term categories and relationship types according to vocabulary.mdc
4. Defined target metrics for vocabulary quality and alignment
5. Created vocabulary quality dashboard for tracking metrics
6. Implemented regular alignment checks between notes and vocabulary

### Architecture Decisions
1. Using NixOS module system for core architecture
2. Implementing event-driven communication
3. Using content-addressed storage
4. Implementing MCP for AI integration
5. Creating knowledge graph implementation for queryable vocabulary
6. Using Mermaid for knowledge graph visualization

### Implementation Decisions
1. Rust as primary implementation language
2. NATS for message transport
3. S3-compatible storage for objects
4. Local LLM integration through Ollama
5. Graph database for knowledge structure
6. Mermaid-based knowledge graphs for documentation

### Documentation Decisions
1. Following project memory structure
2. Implementing vocabulary management workflow
3. Using domain-driven documentation
4. Maintaining research trails
5. Documenting implementation patterns with code examples
6. Using Mermaid for visualization

## Next Steps

### Immediate Actions
1. Ensure team adoption of standardized file structure and workflow
2. Create training materials for vocabulary manager usage
3. Run initial vocabulary workflow analysis to establish baseline metrics
4. Develop core module prototype
5. Document implementation patterns
6. Begin case study development
7. Plan AI integration framework
8. Create research integration

### Short-term Goals
1. Integrate vocabulary workflow checking into CI pipeline
2. Develop minimal core module prototype
3. Document key implementation patterns
4. Create detailed case study
5. Implement basic AI integration
6. Define compliance framework

### Long-term Goals
1. Achieve target metrics for vocabulary quality and alignment
2. Enhance AI integration framework
3. Expand compliance documentation
4. Develop operational testing framework
5. Build research community

## Current Domain Focus

### Composable Information Machines (CIMs)

### Ontology Management

A new focus area has been added to support the development of a specialized ontology extraction and management tool. This tool will help in analyzing files and documents to extract domain-specific terms and relationships, building cohesive taxonomies and ontologies.

#### Key Domain Concepts

1. **Ontology Extraction**
   - Parsing different file formats to extract domain-specific terms and relationships
   - Using NLP techniques for identifying relevant concepts
   - Maintaining provenance of extracted information

2. **Ontology Comparison**
   - Comparing extracted ontologies against seed ontologies
   - Identifying gaps and inconsistencies
   - Mapping between different ontological representations

3. **Graph-Based Storage**
   - Using Neo4j as a graph database for ontology storage
   - Representing terms and relationships as graph structures
   - Enabling complex graph queries and traversals

4. **MCP Server Integration**
   - Exposing ontology management capabilities through an MCP server
   - Supporting both Cursor and other MCP clients
   - Enabling synchronous and asynchronous operations

#### Implementation

The implementation of these domain concepts is being realized through the cim-ontology-tool project, a Rust-based MCP server that integrates with Neo4j for graph database storage. 