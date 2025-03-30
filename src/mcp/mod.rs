//! MCP module for ontology operations
//!
//! This module provides the Model Context Protocol implementation for ontology operations.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value;

pub mod handler;
pub mod operations;
pub mod server;

/// Request type for Model Context Protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    /// Unique request identifier
    pub id: String,
    /// Operation to perform
    pub operation: String,
    /// Optional parameters
    #[serde(default)]
    pub params: Value,
}

/// Status code for MCP responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MCPStatus {
    /// Operation was successful
    Success,
    /// Operation failed
    Error,
}

/// Response type for Model Context Protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// Status of the response
    pub status: MCPStatus,
    /// Optional message (especially for errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional result data for successful operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error type for error responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    /// Detailed error information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Value>,
}

/// Error type for MCP operations
#[derive(Debug, thiserror::Error)]
pub enum MCPError {
    /// Invalid parameters
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Entity not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Operation error
    #[error("Operation error: {0}")]
    OperationError(String),

    /// Operation error with details
    #[error("Operation error: {0}")]
    OperationErrorWithDetails(String, Value),

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializeError(String),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IOError(String),

    /// Server error
    #[error("Server error: {0}")]
    ServerError(String),
}

/// Convert MCPError to MCPResponse
impl From<MCPError> for MCPResponse {
    fn from(error: MCPError) -> Self {
        match error {
            MCPError::InvalidParams(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("INVALID_PARAMS".to_string()),
                error_details: None,
            },
            MCPError::NotFound(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("NOT_FOUND".to_string()),
                error_details: None,
            },
            MCPError::OperationError(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("OPERATION_ERROR".to_string()),
                error_details: None,
            },
            MCPError::OperationErrorWithDetails(message, details) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message),
                result: None,
                error_type: Some("OPERATION_ERROR".to_string()),
                error_details: Some(details),
            },
            MCPError::StorageError(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("STORAGE_ERROR".to_string()),
                error_details: None,
            },
            MCPError::SerializeError(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("SERIALIZATION_ERROR".to_string()),
                error_details: None,
            },
            MCPError::ParseError(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("PARSE_ERROR".to_string()),
                error_details: None,
            },
            MCPError::IOError(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("IO_ERROR".to_string()),
                error_details: None,
            },
            MCPError::ServerError(message) => MCPResponse {
                status: MCPStatus::Error,
                message: Some(message.clone()),
                result: None,
                error_type: Some("SERVER_ERROR".to_string()),
                error_details: None,
            },
        }
    }
}

/// Session token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionToken {
    /// Token ID
    pub id: Uuid,
    /// Token value
    pub token: String,
    /// Expiration timestamp (seconds since epoch)
    pub expires_at: u64,
} 