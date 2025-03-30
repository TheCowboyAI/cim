use thiserror::Error;

/// Custom error type for the MCP client
#[derive(Error, Debug)]
pub enum Error {
    #[error("NATS connection error: {0}")]
    NatsConnection(#[from] async_nats::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Request timed out after {0} seconds")]
    Timeout(u64),

    #[error("MCP error response: {code} - {message}")]
    MCPError { code: String, message: String },

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("No response received for request")]
    NoResponse,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for MCP client operations
pub type Result<T> = std::result::Result<T, Error>; 