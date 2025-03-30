//! MCP (Model Context Protocol) implementation
//!
//! This module provides the MCP server implementation for the ontology tool.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use uuid::Uuid;

mod server;
mod handler;
mod operations;

pub use server::ServerConfig;
pub use handler::OperationHandler;
pub use operations::OntologyOperations;

/// MCP request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    /// Unique request ID
    pub id: String,
    /// Operation to perform
    pub operation: String,
    /// Parameters for the operation
    pub params: HashMap<String, serde_json::Value>,
    /// Request context
    pub context: Option<HashMap<String, serde_json::Value>>,
}

/// MCP response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// Request ID this response is for
    pub request_id: String,
    /// Status of the operation
    pub status: MCPStatus,
    /// Response data
    pub data: Option<serde_json::Value>,
    /// Error information, if status is error
    pub error: Option<MCPError>,
}

/// Status of an MCP operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MCPStatus {
    /// Operation completed successfully
    Success,
    /// Operation is still in progress
    Pending,
    /// Operation failed
    Error,
}

/// Error information for an MCP operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Additional error details
    pub details: Option<serde_json::Value>,
} 