//! CIM Ontology Tool
//!
//! A tool for extracting, managing, and manipulating ontologies from various sources.
//! Provides functionality for ontology extraction, storage, and manipulation via MCP.

pub mod ontology;
pub mod storage;
pub mod source;
pub mod mcp;

// Re-export key types for convenience
pub use ontology::{Ontology, Term, Relationship, RelationshipType};
pub use storage::{OntologyStorage, StorageError};
pub use source::{Source, SourceType};
pub use mcp::{MCPRequest, MCPResponse, MCPError, MCPStatus};

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Name of the library
pub const NAME: &str = env!("CARGO_PKG_NAME"); 