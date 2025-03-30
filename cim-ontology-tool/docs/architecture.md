# Architecture - CIM Ontology Tool

## Overview

The CIM Ontology Tool is designed as a modular Rust application that provides ontology extraction, analysis, and management capabilities through an MCP (Model Context Protocol) server interface. It uses Neo4j as its primary data store for graph-based ontology representations.

## System Architecture

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
│         │                  │   │            │  │
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

## Components in Detail

### 1. MCP Protocol Interface

The MCP Protocol Interface serves as the communication layer between clients (like Cursor) and the cim-ontology-tool.

#### Design Principles:
- Implement the Model Context Protocol specification
- Support both synchronous and asynchronous operations
- Provide structured error handling and response formatting
- Enable streaming of large result sets

#### Key Functionalities:
- Request parsing and validation
- Operation routing to appropriate handlers
- Response construction according to MCP standards
- Connection lifecycle management

#### Implementation:
```rust
// src/mcp/interface.rs
pub struct MCPInterface {
    server: MCPServer,
    handlers: HashMap<String, Box<dyn OperationHandler>>,
}

impl MCPInterface {
    // Initialize with registered operation handlers
    pub fn new(server: MCPServer) -> Self { ... }
    
    // Handle incoming MCP requests
    pub async fn handle_request(&self, request: MCPRequest) -> MCPResponse { ... }
    
    // Register operation handlers
    pub fn register_handler(&mut self, operation: String, handler: Box<dyn OperationHandler>) { ... }
}
```

### 2. Extractor Module

The Extractor Module is responsible for traversing file systems, parsing different file formats, and extracting ontological information.

#### Design Principles:
- Support multiple file formats and languages
- Use parallel processing for improved performance
- Apply NLP techniques for term extraction
- Provide extensible extraction pipeline architecture

#### Key Functionalities:
- File system traversal with filtering
- File parsing based on content type
- Term and relationship extraction
- Ontology normalization and standardization

#### Implementation:
```rust
// src/extractor/mod.rs
pub struct Extractor {
    config: ExtractorConfig,
    parsers: HashMap<String, Box<dyn FileParser>>,
    nlp_pipeline: NLPPipeline,
}

impl Extractor {
    // Create a new extractor with given configuration
    pub fn new(config: ExtractorConfig) -> Self { ... }
    
    // Process a directory recursively
    pub async fn process_directory(&self, path: &Path) -> Result<Ontology> { ... }
    
    // Process a single file
    pub async fn process_file(&self, path: &Path) -> Result<OntologyFragment> { ... }
    
    // Extract terms from content
    pub fn extract_terms(&self, content: &str, context: &FileContext) -> Vec<Term> { ... }
    
    // Extract relationships from content
    pub fn extract_relationships(&self, content: &str, terms: &[Term]) -> Vec<Relationship> { ... }
}
```

### 3. Analyzer Module

The Analyzer Module processes extracted ontologies, comparing them with seed ontologies and generating insights and metrics.

#### Design Principles:
- Support different analysis strategies
- Provide both statistical and semantic analysis
- Enable ontology comparison and merging
- Generate actionable insights

#### Key Functionalities:
- Comparison between ontologies
- Consistency checking
- Gap analysis
- Metrics calculation
- Visualization data preparation

#### Implementation:
```rust
// src/analyzer/mod.rs
pub struct Analyzer {
    config: AnalyzerConfig,
    strategies: Vec<Box<dyn AnalysisStrategy>>,
}

impl Analyzer {
    // Create a new analyzer with given configuration
    pub fn new(config: AnalyzerConfig) -> Self { ... }
    
    // Compare source ontology with target ontology
    pub fn compare(&self, source: &Ontology, target: &Ontology) -> ComparisonResult { ... }
    
    // Calculate metrics for an ontology
    pub fn calculate_metrics(&self, ontology: &Ontology) -> OntologyMetrics { ... }
    
    // Check for consistency within an ontology
    pub fn check_consistency(&self, ontology: &Ontology) -> ConsistencyReport { ... }
    
    // Generate visualization data
    pub fn generate_visualization(&self, ontology: &Ontology) -> VisualizationData { ... }
}
```

### 4. Domain Model

The Domain Model defines the core entities and relationships that represent ontological concepts within the system.

#### Design Principles:
- Follow Domain-Driven Design principles
- Ensure clear separation of concerns
- Support persistence to Neo4j
- Enable serialization and deserialization

#### Key Entities:
- Term: A concept or word in the ontology
- Relationship: Connection between terms
- Ontology: Collection of terms and relationships
- Domain: Group of related ontologies
- Source: Origin of ontological information

#### Implementation:
```rust
// src/domain/mod.rs
pub struct Term {
    id: Uuid,
    name: String,
    definition: Option<String>,
    domain: Option<String>,
    sources: Vec<Source>,
    metadata: HashMap<String, String>,
}

pub struct Relationship {
    id: Uuid,
    source_term_id: Uuid,
    target_term_id: Uuid,
    relationship_type: String,
    strength: f64,
    sources: Vec<Source>,
    metadata: HashMap<String, String>,
}

pub struct Ontology {
    id: Uuid,
    name: String,
    description: Option<String>,
    terms: Vec<Term>,
    relationships: Vec<Relationship>,
    metadata: HashMap<String, String>,
}

// Domain methods and implementations...
```

### 5. Neo4j Integration

The Neo4j Integration component handles all interactions with the Neo4j graph database.

#### Design Principles:
- Use connection pooling for efficiency
- Implement retry and circuit breaker patterns
- Optimize Cypher queries for performance
- Provide transaction management

#### Key Functionalities:
- Database connection management
- Domain object to graph data mapping
- Transaction handling
- Query execution and result mapping

#### Implementation:
```rust
// src/storage/neo4j/mod.rs
pub struct Neo4jClient {
    pool: Pool<Neo4jConnectionManager>,
    config: Neo4jConfig,
}

impl Neo4jClient {
    // Create a new Neo4j client with connection pool
    pub fn new(config: Neo4jConfig) -> Result<Self> { ... }
    
    // Execute a Cypher query with parameters
    pub async fn execute_query<T>(&self, query: &str, params: HashMap<String, Value>) -> Result<T> { ... }
    
    // Begin a new transaction
    pub async fn begin_transaction(&self) -> Result<Neo4jTransaction> { ... }
    
    // Save an ontology to Neo4j
    pub async fn save_ontology(&self, ontology: &Ontology) -> Result<()> { ... }
    
    // Load an ontology from Neo4j
    pub async fn load_ontology(&self, id: Uuid) -> Result<Ontology> { ... }
    
    // Other database operations...
}
```

### 6. MCP Server

The MCP Server implements the MCP protocol and exposes the system's capabilities through a well-defined API.

#### Design Principles:
- Adhere to MCP protocol specifications
- Support both HTTP and WebSocket transports
- Implement proper authentication and authorization
- Provide detailed logging and monitoring

#### Key Functionalities:
- Server lifecycle management
- Request routing to appropriate handlers
- Session management
- Authentication and authorization
- Error handling and reporting

#### Implementation:
```rust
// src/mcp/server.rs
pub struct MCPServer {
    config: MCPServerConfig,
    interface: Arc<MCPInterface>,
    http_server: Option<Server>,
}

impl MCPServer {
    // Create a new MCP server with given configuration
    pub fn new(config: MCPServerConfig) -> Self { ... }
    
    // Start the MCP server
    pub async fn start(&mut self) -> Result<()> { ... }
    
    // Stop the MCP server
    pub async fn stop(&mut self) -> Result<()> { ... }
    
    // Handle incoming HTTP request
    async fn handle_http_request(&self, req: Request<Body>) -> Response<Body> { ... }
    
    // Handle WebSocket connection
    async fn handle_websocket(&self, socket: WebSocket) { ... }
}
```

## Data Flow

### Extraction Process:
1. Client sends extraction request to MCP Server
2. MCP Server routes request to Extractor Module
3. Extractor traverses specified directory structure
4. Files are parsed and terms/relationships extracted
5. Extracted ontology is mapped to Domain Model
6. Domain objects are persisted to Neo4j
7. Results are returned to client via MCP Server

### Analysis Process:
1. Client sends analysis request to MCP Server
2. MCP Server routes request to Analyzer Module
3. Required ontologies are loaded from Neo4j
4. Analyzer performs comparison or metric calculation
5. Results are mapped to Domain Model
6. Insights and visualizations are generated
7. Results are returned to client via MCP Server

### Query Process:
1. Client sends query request to MCP Server
2. MCP Server validates query parameters
3. Query is translated to Cypher and executed against Neo4j
4. Results are mapped to Domain Model
5. Formatted response is returned to client via MCP Server

## Technology Decisions

### Language: Rust
- Memory safety without garbage collection
- Performance comparable to C/C++
- Strong type system and compile-time checks
- Excellent concurrency support
- Growing ecosystem for NLP and graph processing

### Database: Neo4j
- Native graph database
- Support for complex ontological relationships
- Powerful query language (Cypher)
- Scalable and performant
- Rich ecosystem and tooling

### Protocol: MCP (Model Context Protocol)
- Designed for AI/ML contexts
- Supports structured communication between clients and servers
- Flexible operation definitions
- Stream-oriented for large data sets

## Deployment Considerations

### Dependencies
- Neo4j database (4.4+)
- Rust toolchain (nightly)
- Docker for containerization (optional)
- Networking infrastructure

### Configuration
- Environment variables for sensitive information
- YAML/TOML for static configuration
- Runtime configuration via API

### Monitoring
- Prometheus metrics
- Log aggregation
- Health check endpoints
- Performance monitoring

### Scaling
- Horizontal scaling for MCP Server
- Connection pooling for Neo4j
- Caching layers for frequent queries
- Resource limits and throttling

## Current Implementation State

As of the latest implementation, several important architectural adaptations have been made to ensure maintainability, type safety, and proper error handling. These changes represent a practical evolution of the original architecture based on real-world implementation constraints.

### Key Architectural Adaptations

#### Storage Implementation

The system has temporarily moved away from Neo4j dependency to a `DummyStorage` implementation to focus on the core server functionality:

```rust
#[derive(Clone)]
struct DummyStorage;

#[async_trait::async_trait]
impl OntologyStorage for DummyStorage {
    async fn initialize(&self) -> StorageResult<()> {
        println!("Initializing dummy storage");
        Ok(())
    }
    // Other methods...
}
```

This allows development of the MCP server and operation handlers without depending on a Neo4j instance, providing a clean separation of concerns and easier testing.

#### Synchronous Operation Handlers

Operation handlers have been implemented as synchronous functions that return futures:

```rust
fn extract_ontology<S: OntologyStorage>(
    params: &HashMap<String, Value>,
    _storage: &Arc<S>,
) -> Result<Value, MCPError> {
    // Implementation...
}
```

This approach simplifies the implementation while still supporting the asynchronous nature of the server through appropriate wrapping mechanisms.

#### Handler Registration System

The system uses a type-safe function registration system:

```rust
pub fn register<F, Fut>(&mut self, operation: &str, handler: F) -> &mut Self
where
    F: Fn(&HashMap<String, Value>, &Arc<S>) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<Value, MCPError>> + Send,
{
    // Implementation...
}
```

This enables flexibility in handler implementation while maintaining strong type checking at compile time.

#### Error Handling Structure

A comprehensive error handling system has been implemented:

```rust
pub enum MCPError {
    InvalidParams(String),
    NotFound(String),
    OperationError(String),
    // Other variants...
}

impl From<MCPError> for MCPResponse {
    fn from(error: MCPError) -> Self {
        // Conversion logic...
    }
}
```

This provides structured error reporting and consistent error handling throughout the application.

### System Architecture: Current State

The current system architecture can be visualized as follows:

```
┌────────────────────┐     ┌───────────────────┐
│                    │     │                   │
│      MCP Clients   │◄────┤   MCP Protocol    │
│                    │     │    Interface      │
│                    │     │                   │
└────────────────────┘     └─────────┬─────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────┐
│                                                │
│               cim-ontology-tool                │
│                                                │
│  ┌─────────────┐       ┌──────────────────┐   │
│  │             │       │                  │   │
│  │ Operation   │       │  MCP Server &    │   │
│  │  Handlers   │◄─────►│  Request Router  │   │
│  │             │       │                  │   │
│  └──────┬──────┘       └───────┬──────────┘   │
│         │                      │              │
│         │                      │              │
│         ▼                      ▼              │
│  ┌─────────────┐       ┌──────────────────┐   │
│  │             │       │                  │   │
│  │  Domain     │       │  Dummy Storage   │   │
│  │   Model     │       │  Implementation  │   │
│  │             │       │                  │   │
│  └─────────────┘       └──────────────────┘   │
│                                                │
└────────────────────────────────────────────────┘
```

### Future Architectural Evolution

As the project continues to evolve, the following architectural changes are planned:

1. **Reintegration of Neo4j Storage**: The `DummyStorage` will be replaced with a full Neo4j implementation.

2. **Enhanced Concurrency**: Additional concurrency patterns will be implemented to handle higher loads.

3. **Extended Domain Model**: The domain model will be expanded to support more complex ontological relationships.

4. **Extraction and Analysis Modules**: Full implementation of the extraction and analysis capabilities outlined in the original architecture.

The current architectural state provides a solid foundation for these future enhancements while maintaining a clean, maintainable, and type-safe codebase. 