//! MCP Server implementation
//!
//! This module provides the HTTP server implementation for the MCP protocol.

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::mcp::{MCPRequest, MCPResponse, MCPError, MCPStatus};
use crate::storage::OntologyStorage;
use crate::mcp::handler::OperationHandler;

/// Configuration for the MCP server
#[derive(Clone)]
pub struct ServerConfig {
    /// Address to bind the server to
    pub address: String,
    /// Maximum request size in bytes
    pub max_request_size: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:3000".to_string(),
            max_request_size: 1024 * 1024, // 1 MB
        }
    }
}

/// Server state for axum
#[derive(Clone)]
pub struct ServerState<S: OntologyStorage + Clone> {
    /// Operation handler
    pub handler: Arc<OperationHandler<S>>,
    /// Server configuration
    pub config: ServerConfig,
}

/// HTTP server for the MCP API
pub struct Server<S: OntologyStorage + Clone + 'static> {
    /// Server state
    state: ServerState<S>,
    /// Shutdown channel
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: mpsc::Receiver<()>,
}

/// Start the MCP server
pub async fn start_server<S>(config: ServerConfig, storage: S) -> Result<(), String>
where
    S: OntologyStorage + Send + Sync + Clone + 'static,
{
    // Create the handler
    let mut handler = OperationHandler::new(Arc::new(storage));
    
    // Register operations
    crate::mcp::operations::OntologyOperations::register_all(&mut handler);
    
    // Create server state
    let state = ServerState {
        handler: Arc::new(handler),
        config: config.clone(),
    };
    
    let addr = &config.address;
    
    println!("Starting MCP server on {}", addr);
    
    // Create the router
    let app = Router::new()
        .route("/health", get(health_check::<S>))
        .route("/mcp", post(handle_mcp_request::<S>))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start the server
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|e| format!("Server error: {}", e))?;

    Ok(())
}

/// Health check endpoint
async fn health_check<S>(State(_state): State<ServerState<S>>) -> impl IntoResponse
where
    S: OntologyStorage + Send + Sync + Clone + 'static,
{
    (StatusCode::OK, "OK")
}

/// MCP request handler
async fn handle_mcp_request<S>(
    State(state): State<ServerState<S>>,
    Json(request): Json<MCPRequest>,
) -> impl IntoResponse
where
    S: OntologyStorage + Send + Sync + Clone + 'static,
{
    // Authenticate the request
    if request.operation != "hello" && !is_authenticated::<S>(&request, &state.handler) {
        return Json(MCPResponse {
            status: MCPStatus::Error,
            message: Some("Unauthorized".to_string()),
            result: None,
            error_type: Some("AUTH_ERROR".to_string()),
            error_details: None,
        })
        .into_response();
    }

    // Handle the request
    match state.handler.handle(&request).await {
        Ok(response) => Json(response).into_response(),
        Err(error) => {
            let (error_type, message) = match &error {
                MCPError::InvalidParams(msg) => ("INVALID_PARAMS", msg.clone()),
                MCPError::NotFound(msg) => ("NOT_FOUND", msg.clone()),
                MCPError::OperationError(msg) => ("OPERATION_ERROR", msg.clone()),
                MCPError::StorageError(msg) => ("STORAGE_ERROR", msg.clone()),
                MCPError::SerializeError(msg) => ("SERIALIZATION_ERROR", msg.clone()),
                MCPError::ParseError(msg) => ("PARSE_ERROR", msg.clone()),
                MCPError::IOError(msg) => ("IO_ERROR", msg.clone()),
                MCPError::ServerError(msg) => ("SERVER_ERROR", msg.clone()),
                MCPError::OperationErrorWithDetails(msg, _) => ("OPERATION_ERROR", msg.clone()),
            };

            Json(MCPResponse {
                status: MCPStatus::Error,
                message: Some(message),
                result: None,
                error_type: Some(error_type.to_string()),
                error_details: None,
            })
            .into_response()
        }
    }
}

/// Check if a request is authenticated
fn is_authenticated<S>(_request: &MCPRequest, _handler: &Arc<OperationHandler<S>>) -> bool
where
    S: OntologyStorage + Send + Sync + Clone + 'static,
{
    // Here we should check the authentication token in a header or auth field
    // For now, always return true
    true
}

impl<S: OntologyStorage + Clone + 'static> Server<S> {
    /// Create a new server
    pub fn new(storage: Arc<S>, config: ServerConfig) -> Self {
        // Create operation handler
        let mut handler = OperationHandler::new(storage);
        
        // Register operations
        crate::mcp::operations::OntologyOperations::register_all(&mut handler);
        
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        
        Self {
            state: ServerState {
                handler: Arc::new(handler),
                config,
            },
            shutdown_tx,
            shutdown_rx,
        }
    }
    
    /// Start the server
    pub async fn start(&mut self) -> Result<(), MCPError> {
        let listener = TcpListener::bind(&self.state.config.address).await
            .map_err(|e| MCPError::ServerError(format!("Failed to bind to address: {}", e)))?;
        
        println!("MCP Server listening on {}", self.state.config.address);
        
        loop {
            tokio::select! {
                // Accept new connection
                result = listener.accept() => {
                    match result {
                        Ok((socket, addr)) => {
                            println!("New connection from {}", addr);
                            
                            // Clone state for the connection handler
                            let state = self.state.clone();
                            
                            // Spawn new task to handle the connection
                            tokio::spawn(async move {
                                if let Err(e) = Self::handle_connection(socket, state).await {
                                    eprintln!("Error handling connection: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Error accepting connection: {}", e);
                        }
                    }
                }
                
                // Shutdown signal
                _ = self.shutdown_rx.recv() => {
                    println!("Shutting down server...");
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Shutdown the server
    pub async fn shutdown(&self) -> Result<(), MCPError> {
        self.shutdown_tx.send(()).await
            .map_err(|_| MCPError::ServerError("Failed to send shutdown signal".to_string()))?;
        
        Ok(())
    }
    
    /// Handle a client connection
    async fn handle_connection(mut socket: tokio::net::TcpStream, state: ServerState<S>) -> Result<(), MCPError> {
        let max_size = state.config.max_request_size;
        let mut buffer = vec![0; max_size];
        
        // Read request
        let n = socket.read(&mut buffer).await
            .map_err(|e| MCPError::IOError(format!("Failed to read from socket: {}", e)))?;
        
        if n == 0 {
            // Connection closed
            return Ok(());
        }
        
        // Parse request
        let request_str = String::from_utf8_lossy(&buffer[..n]);
        let request: MCPRequest = serde_json::from_str(&request_str)
            .map_err(|e| MCPError::ParseError(format!("Failed to parse request: {}", e)))?;
        
        // Handle request
        let response = match state.handler.handle(&request).await {
            Ok(result) => MCPResponse {
                status: MCPStatus::Success,
                message: Some("Operation completed successfully".to_string()),
                result: Some(result),
                error_type: None,
                error_details: None,
            },
            Err(err) => MCPResponse::from(err),
        };
        
        // Serialize and send response
        let response_json = serde_json::to_string(&response)
            .map_err(|e| MCPError::SerializeError(format!("Failed to serialize response: {}", e)))?;
        
        socket.write_all(response_json.as_bytes()).await
            .map_err(|e| MCPError::IOError(format!("Failed to write to socket: {}", e)))?;
        
        Ok(())
    }
} 