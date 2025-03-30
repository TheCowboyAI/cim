//! Operation handler for MCP requests
//!
//! This module handles routing and execution of MCP operations.

use crate::mcp::{MCPError, MCPRequest, MCPResponse, MCPStatus};
use crate::storage::OntologyStorage;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Handler for MCP operations
pub struct OperationHandler<S: OntologyStorage> {
    /// Storage backend
    storage: Arc<S>,
    /// Registered operation handlers
    handlers: HashMap<String, Box<dyn OperationFn<S>>>,
}

/// Trait for operation handler functions
#[async_trait::async_trait]
pub trait OperationFn<S: OntologyStorage>: Send + Sync {
    /// Execute the operation
    async fn execute(
        &self,
        params: &HashMap<String, Value>,
        storage: &Arc<S>,
    ) -> Result<Value, MCPError>;
}

#[async_trait::async_trait]
impl<S, F, Fut> OperationFn<S> for F
where
    S: OntologyStorage,
    F: Send + Sync + Fn(&HashMap<String, Value>, &Arc<S>) -> Fut,
    Fut: std::future::Future<Output = Result<Value, MCPError>> + Send,
{
    async fn execute(
        &self,
        params: &HashMap<String, Value>,
        storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        self(params, storage).await
    }
}

impl<S: OntologyStorage + 'static> OperationHandler<S> {
    /// Create a new operation handler
    pub fn new(storage: Arc<S>) -> Self {
        Self {
            storage,
            handlers: HashMap::new(),
        }
    }

    /// Register an operation handler
    pub fn register<F, Fut>(&mut self, operation: &str, handler: F) -> &mut Self
    where
        F: Fn(&HashMap<String, Value>, &Arc<S>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Value, MCPError>> + Send,
    {
        self.handlers
            .insert(operation.to_string(), Box::new(handler));
        self
    }

    /// Handle an MCP request
    pub async fn handle(&self, request: MCPRequest) -> MCPResponse {
        let operation = request.operation.clone();
        let params = request.params.clone();

        match self.handlers.get(&operation) {
            Some(handler) => match handler.execute(&params, &self.storage).await {
                Ok(data) => MCPResponse {
                    request_id: request.id,
                    status: MCPStatus::Success,
                    data: Some(data),
                    error: None,
                },
                Err(error) => MCPResponse {
                    request_id: request.id,
                    status: MCPStatus::Error,
                    data: None,
                    error: Some(error),
                },
            },
            None => MCPResponse {
                request_id: request.id,
                status: MCPStatus::Error,
                data: None,
                error: Some(MCPError {
                    code: "OPERATION_NOT_FOUND".to_string(),
                    message: format!("Operation '{}' not found", operation),
                    details: None,
                }),
            },
        }
    }

    /// Get a reference to the storage
    pub fn storage(&self) -> &Arc<S> {
        &self.storage
    }
}

/// Create an MCP error
pub fn mcp_error(code: &str, message: &str) -> MCPError {
    MCPError {
        code: code.to_string(),
        message: message.to_string(),
        details: None,
    }
}

/// Create an MCP error with details
pub fn mcp_error_with_details(code: &str, message: &str, details: Value) -> MCPError {
    MCPError {
        code: code.to_string(),
        message: message.to_string(),
        details: Some(details),
    }
} 