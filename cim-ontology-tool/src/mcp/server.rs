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

/// Start the MCP server
pub async fn start_server<S: OntologyStorage + Clone + 'static>(
    config: ServerConfig,
    storage: Arc<S>,
) -> Result<(), String> {
    // Initialize the event system
    let (event_bus, mut event_dispatcher, event_store) = crate::events::init_event_system(100);
    
    // Register event handlers
    event_dispatcher.register_handler(
        OntologyEventHandler::new("ontology_handler", Arc::clone(&storage))
    ).await;
    
    event_dispatcher.register_handler(
        QueryEventHandler::new("query_handler", Arc::clone(&storage))
    ).await;
    
    // Start the event dispatcher
    event_dispatcher.start().await
        .map_err(|e| format!("Failed to start event dispatcher: {}", e))?;
    
    // Log system startup
    println!("Event system initialized");

    let state = ServerState {
        storage,
        config: config.clone(),
        event_bus,
    };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/mcp", post(handle_mcp_request::<S>))
        .with_state(state);

    let listener = TcpListener::bind(&config.address)
        .await
        .map_err(|e| format!("Failed to bind to address: {}", e))?;

    println!("MCP server listening on {}", config.address);

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server error: {}", e))?;

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