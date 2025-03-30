//! MCP server implementation
//!
//! This module provides the HTTP server implementation for the MCP protocol.

use crate::events::{EventBus, EventDispatcher, EventStore};
use crate::events::handlers::{OntologyEventHandler, QueryEventHandler};
use crate::mcp::{MCPError, MCPRequest, MCPResponse, MCPStatus, request_to_event, handle_event_and_create_response};
use crate::storage::OntologyStorage;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use anyhow::{Context, Result};
use tokio::sync::mpsc;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Address to bind to
    pub address: SocketAddr,
    /// Authentication token
    pub auth_token: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8080".parse().unwrap(),
            auth_token: None,
        }
    }
}

/// Server state
#[derive(Debug, Clone)]
pub struct ServerState<S: OntologyStorage + Clone + 'static> {
    /// Storage backend
    pub storage: Arc<S>,
    /// Configuration
    pub config: ServerConfig,
    /// Event bus
    pub event_bus: EventBus,
}

// Define the MCPRequestHandler struct
struct MCPRequestHandler<S: OntologyStorage> {
    storage: Arc<S>,
    auth_token: Option<String>,
}

impl<S: OntologyStorage> MCPRequestHandler<S> {
    /// Create a new MCPRequestHandler
    fn new(storage: Arc<S>, auth_token: Option<String>) -> Self {
        Self { storage, auth_token }
    }

    /// Handle a client connection
    async fn handle_connection(&self, _socket: tokio::net::TcpStream) -> Result<()> {
        // This is a placeholder for the actual connection handling logic
        println!("Handling connection (placeholder implementation)");
        Ok(())
    }
}

/// Start the MCP server
///
/// # Arguments
///
/// * `config` - Server configuration
/// * `storage` - Ontology storage implementation
///
/// # Returns
///
/// * `Result<()>` - Result indicating success or failure
pub async fn start_server<S: OntologyStorage + 'static>(
    config: ServerConfig,
    storage: Arc<S>,
) -> Result<()> {
    // Initialize event system
    let event_capacity = 100; // Default event capacity
    init_event_system(event_capacity).await?;

    // Bind TCP listener to the specified address
    let listener = TcpListener::bind(&config.address)
        .await
        .with_context(|| format!("Failed to bind to address: {}", config.address))?;

    println!("MCP server listening on {}", config.address);

    // Handle incoming connections
    loop {
        let (socket, addr) = match listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        println!("New client connected: {}", addr);

        // Clone storage for the new connection
        let storage_clone = Arc::clone(&storage);
        let auth_token = config.auth_token.clone();

        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            let handler = MCPRequestHandler::new(storage_clone, auth_token);
            if let Err(e) = handler.handle_connection(socket).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            }
            println!("Client disconnected: {}", addr);
        });
    }
}

/// Initialize the event system
///
/// # Arguments
///
/// * `capacity` - Capacity of the event queue
///
/// # Returns
///
/// * `Result<()>` - Result indicating success or failure
async fn init_event_system(capacity: usize) -> Result<()> {
    let (event_bus, mut event_dispatcher, _event_store) = crate::events::init_event_system(100);
    
    // Start event dispatcher in a background task
    tokio::spawn(async move {
        if let Err(e) = event_dispatcher.start().await {
            eprintln!("Event dispatcher error: {}", e);
        }
    });
    
    println!("Event system initialized with capacity: {}", capacity);
    
    Ok(())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Handle an MCP request
async fn handle_mcp_request<S: OntologyStorage + Clone + 'static>(
    State(state): State<ServerState<S>>,
    headers: HeaderMap,
    Json(request): Json<MCPRequest>,
) -> impl IntoResponse {
    // Authenticate request if auth token is set
    if let Some(auth_token) = &state.config.auth_token {
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if !auth_str.starts_with("Bearer ") || !auth_str[7..].eq(auth_token) {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(MCPResponse {
                            request_id: request.id.clone(),
                            status: MCPStatus::Error,
                            data: None,
                            error: Some(MCPError {
                                code: "UNAUTHORIZED".to_string(),
                                message: "Invalid authentication token".to_string(),
                                details: None,
                            }),
                        }),
                    );
                }
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(MCPResponse {
                        request_id: request.id.clone(),
                        status: MCPStatus::Error,
                        data: None,
                        error: Some(MCPError {
                            code: "UNAUTHORIZED".to_string(),
                            message: "Invalid authentication header".to_string(),
                            details: None,
                        }),
                    }),
                );
            }
        } else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(MCPResponse {
                    request_id: request.id.clone(),
                    status: MCPStatus::Error,
                    data: None,
                    error: Some(MCPError {
                        code: "UNAUTHORIZED".to_string(),
                        message: "Authentication required".to_string(),
                        details: None,
                    }),
                }),
            );
        }
    }

    // Convert the request to an event
    match request_to_event(&request) {
        Some(event) => {
            // Process the event
            let response = handle_event_and_create_response(&state.event_bus, &request, event).await;
            
            // Return the response with appropriate status code
            let status_code = match response.status {
                MCPStatus::Success => StatusCode::OK,
                MCPStatus::Pending => StatusCode::ACCEPTED,
                MCPStatus::Error => StatusCode::BAD_REQUEST,
            };
            
            (status_code, Json(response))
        },
        None => {
            // No matching event type, return an error
            (
                StatusCode::BAD_REQUEST,
                Json(MCPResponse {
                    request_id: request.id.clone(),
                    status: MCPStatus::Error,
                    data: None,
                    error: Some(MCPError {
                        code: "INVALID_OPERATION".to_string(),
                        message: format!("Unknown operation: {}", request.operation),
                        details: None,
                    }),
                }),
            )
        }
    }
} 