//! Operation handler for MCP requests
//!
//! This module handles routing and execution of MCP operations.

use crate::mcp::{MCPError, MCPRequest};
use crate::storage::OntologyStorage;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for an async function that handles an MCP operation
type HandlerFn<S> = Box<dyn Fn(&HashMap<String, Value>, &Arc<S>) -> OperationResult + Send + Sync>;

/// Result type for operation handlers
type OperationResult = Pin<Box<dyn Future<Output = Result<Value, MCPError>> + Send>>;

/// A trait for operation handler functions
pub trait OperationFn<S: OntologyStorage> {
    /// Execute the operation with the given parameters and storage
    fn execute<'a>(
        &'a self,
        params: &'a HashMap<String, Value>,
        storage: &'a Arc<S>,
    ) -> OperationResult;
}

// Implement the trait for any function that matches our signature
impl<S, F, Fut> OperationFn<S> for F
where
    S: OntologyStorage,
    F: Fn(&HashMap<String, Value>, &Arc<S>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Value, MCPError>> + Send + 'static,
{
    fn execute<'a>(
        &'a self,
        params: &'a HashMap<String, Value>,
        storage: &'a Arc<S>,
    ) -> OperationResult {
        Box::pin(self(params, storage))
    }
}

/// Handler for MCP operations
pub struct OperationHandler<S: OntologyStorage> {
    /// Storage backend
    storage: Arc<S>,
    /// Map of operation name to handler function
    handlers: HashMap<String, HandlerFn<S>>,
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
    pub fn register<F>(&mut self, operation: &str, handler: F) -> &mut Self
    where
        F: Fn(&HashMap<String, Value>, &Arc<S>) -> Result<Value, MCPError> + Send + Sync + Clone + 'static,
    {
        let handler_clone = handler.clone();
        
        let handler_fn = Box::new(move |params: &HashMap<String, Value>, storage: &Arc<S>| {
            // Clone references to avoid lifetime issues
            let params_owned: HashMap<String, Value> = params.clone();
            let storage_clone = storage.clone();
            let handler_copy = handler_clone.clone();
            
            // Create a future that executes the handler with owned data
            let fut = async move { 
                handler_copy(&params_owned, &storage_clone)
            };
            
            Box::pin(fut) as OperationResult
        });

        self.handlers.insert(operation.to_string(), handler_fn);
        self
    }

    /// Handle an MCP request
    pub async fn handle(&self, request: &MCPRequest) -> Result<Value, MCPError> {
        // Convert request parameters to HashMap
        let params = match request.params.as_object() {
            Some(obj) => obj.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<String, Value>>(),
            None => HashMap::new(),
        };

        // Look up the handler for this operation
        let handler = self.handlers.get(&request.operation).ok_or_else(|| {
            mcp_error_with_message("UNKNOWN_OPERATION", &format!("Unknown operation: {}", request.operation))
        })?;

        // Execute the handler
        handler(&params, &self.storage).await
    }

    /// Get the storage reference
    pub fn storage(&self) -> &Arc<S> {
        &self.storage
    }
}

/// Create an MCP error with the given message
pub fn mcp_error_with_message(_error_type: &str, message: &str) -> MCPError {
    MCPError::OperationError(message.to_string())
}

/// Create an MCP error with details
pub fn mcp_error_with_details(_error_type: &str, message: &str, details: Value) -> MCPError {
    MCPError::OperationErrorWithDetails(message.to_string(), details)
}

/// Alias for mcp_error_with_message for backward compatibility
pub fn mcp_error(error_type: &str, message: &str) -> MCPError {
    mcp_error_with_message(error_type, message)
} 