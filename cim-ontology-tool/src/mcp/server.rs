//! MCP server implementation
//!
//! This module provides the HTTP server implementation for the MCP protocol.

use crate::mcp::{MCPError, MCPRequest, MCPResponse, MCPStatus, OperationHandler, OntologyOperations};
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
    /// Operation handler
    pub handler: Arc<OperationHandler<S>>,
}

/// Start the MCP server
pub async fn start_server<S: OntologyStorage + Clone + 'static>(
    config: ServerConfig,
    storage: Arc<S>,
) -> Result<(), String> {
    // Create and configure the operation handler
    let mut handler = OperationHandler::new(Arc::clone(&storage));
    
    // Register all operations
    OntologyOperations::register_all(&mut handler);

    let state = ServerState {
        storage,
        config: config.clone(),
        handler: Arc::new(handler),
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

    // Process the request using the operation handler
    let response = state.handler.handle(request).await;
    
    // Return the response with appropriate status code
    let status_code = match response.status {
        MCPStatus::Success => StatusCode::OK,
        MCPStatus::Pending => StatusCode::ACCEPTED,
        MCPStatus::Error => StatusCode::BAD_REQUEST,
    };
    
    (status_code, Json(response))
} 