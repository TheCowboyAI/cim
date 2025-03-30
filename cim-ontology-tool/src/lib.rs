// cim-ontology-tool: A Rust-based MCP server for ontology extraction, analysis, and management.

// Silence warnings about unused code and variables throughout the entire crate
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod analyzer;
pub mod cli;
pub mod events;
pub mod extractor;
pub mod mcp;
pub mod ontology;
pub mod storage;
pub mod utils;

/// Version of the cim-ontology-tool
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Error type for the cim-ontology-tool crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error enum for the cim-ontology-tool crate
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Neo4j error: {0}")]
    Neo4j(String),

    #[error("MCP protocol error: {0}")]
    MCP(String),

    #[error("Extraction error: {0}")]
    Extraction(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Ontology error: {0}")]
    Ontology(String),

    #[error("Event error: {0}")]
    Event(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
} 