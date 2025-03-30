//! MCP server implementation
//!
//! This module provides the NATS-based server implementation for the MCP protocol.

use crate::events::EventBus;
use crate::mcp::{MCPError, MCPRequest, MCPResponse, MCPStatus, request_to_event, handle_event_and_create_response};
use crate::storage::OntologyStorage;
use serde_json::json;
use std::sync::Arc;
use anyhow::{Context, Result};
use futures::StreamExt;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// NATS server URL
    pub nats_url: String,
    /// Authentication token
    pub auth_token: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            nats_url: "nats://localhost:4222".to_string(),
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
    pub _event_bus: EventBus,
}

/// Start the MCP server using NATS
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
    let (event_bus, mut event_dispatcher, _event_store) = crate::events::init_event_system(100);
    
    // Start event dispatcher in a background task
    tokio::spawn(async move {
        if let Err(e) = event_dispatcher.start().await {
            eprintln!("Event dispatcher error: {}", e);
        }
    });
    
    println!("Event system initialized with capacity: {}", event_capacity);
    
    // Connect to NATS
    let client = match connect_to_nats(&config).await {
        Ok(client) => client,
        Err(e) => return Err(anyhow::anyhow!("Failed to connect to NATS: {}", e)),
    };
    
    println!("Connected to NATS server at {}", config.nats_url);
    
    // Create a health check subscription
    let health_client = client.clone();
    tokio::spawn(async move {
        if let Err(e) = setup_health_check(health_client).await {
            eprintln!("Failed to set up health check: {}", e);
        }
    });
    
    // Subscribe to MCP request subjects
    let mut subscription = client.subscribe("mcp.request.>").await
        .with_context(|| "Failed to subscribe to MCP request subjects")?;
    
    println!("Listening for MCP requests on 'mcp.request.>' subjects");
    
    // Process incoming messages
    while let Some(msg) = subscription.next().await {
        // Clone required data for the task
        let client_clone = client.clone();
        let storage_clone = Arc::clone(&storage);
        let event_bus_clone = event_bus.clone();
        let auth_token = config.auth_token.clone();
        
        // Process the message in a separate task
        tokio::spawn(async move {
            if let Err(e) = handle_nats_message(
                msg, 
                client_clone,
                storage_clone, 
                event_bus_clone,
                auth_token
            ).await {
                eprintln!("Error handling message: {}", e);
            }
        });
    }
    
    Ok(())
}

/// Connect to NATS server
async fn connect_to_nats(config: &ServerConfig) -> Result<async_nats::Client> {
    // Simple connection without advanced options for now
    let client = if let Some(token) = &config.auth_token {
        // Connect with authentication
        async_nats::connect_with_options(
            &config.nats_url,
            async_nats::ConnectOptions::new()
                .token(token.clone())
        ).await?
    } else {
        // Connect without authentication
        async_nats::connect(&config.nats_url).await?
    };
    
    Ok(client)
}

/// Setup health check handler
async fn setup_health_check(client: async_nats::Client) -> Result<()> {
    let mut subscription = client.subscribe("mcp.health").await?;
    
    println!("Health check service initialized on 'mcp.health' subject");
    
    while let Some(msg) = subscription.next().await {
        if let Some(reply) = msg.reply {
            let health_response = json!({
                "status": "ok",
                "version": env!("CARGO_PKG_VERSION"),
            });
            
            let response_data = serde_json::to_vec(&health_response)?;
            client.publish(reply, response_data.into()).await?;
        }
    }
    
    Ok(())
}

/// Handle a NATS message containing an MCP request
async fn handle_nats_message<S: OntologyStorage + 'static>(
    msg: async_nats::Message,
    client: async_nats::Client,
    storage: Arc<S>,
    event_bus: EventBus,
    auth_token: Option<String>,
) -> Result<()> {
    // Parse the subject to extract the operation
    let subject = msg.subject.to_string();
    
    // Parse the payload as an MCPRequest
    let request: MCPRequest = match serde_json::from_slice(&msg.payload) {
        Ok(req) => req,
        Err(e) => {
            // If parsing fails, reply with an error
            if let Some(reply) = msg.reply {
                let error_response = MCPResponse {
                    request_id: "unknown".to_string(),
                    status: MCPStatus::Error,
                    data: None,
                    error: Some(MCPError {
                        code: "INVALID_REQUEST".to_string(),
                        message: format!("Failed to parse request: {}", e),
                        details: None,
                    }),
                };
                let response_data = serde_json::to_vec(&error_response)?;
                client.publish(reply, response_data.into()).await?;
            }
            return Err(anyhow::anyhow!("Failed to parse request: {}", e));
        }
    };
    
    // Handle authentication if needed
    if let Some(auth_token) = &auth_token {
        // Check if the request contains a valid auth token in the context
        let is_authenticated = request.context
            .as_ref()
            .and_then(|ctx| ctx.get("auth_token"))
            .and_then(|token| token.as_str())
            .map(|token| token == auth_token)
            .unwrap_or(false);
            
        if !is_authenticated {
            let error_response = MCPResponse {
                request_id: request.id.clone(),
                status: MCPStatus::Error,
                data: None,
                error: Some(MCPError {
                    code: "UNAUTHORIZED".to_string(),
                    message: "Invalid authentication token".to_string(),
                    details: None,
                }),
            };
            
            if let Some(reply) = msg.reply {
                let response_data = serde_json::to_vec(&error_response)?;
                client.publish(reply, response_data.into()).await?;
            }
            
            return Err(anyhow::anyhow!("Unauthorized request"));
        }
    }
    
    // Use the existing event handling code
    if let Some(event) = request_to_event(&request) {
        let response = handle_event_and_create_response(&event_bus, &request, event).await;
        
        // Send the response if a reply subject was provided
        if let Some(reply) = msg.reply {
            let response_data = serde_json::to_vec(&response)?;
            client.publish(reply, response_data.into()).await?;
        }
    } else {
        // No matching event type, send an error response
        let error_response = MCPResponse {
            request_id: request.id.clone(),
            status: MCPStatus::Error,
            data: None,
            error: Some(MCPError {
                code: "INVALID_OPERATION".to_string(),
                message: format!("Unknown operation: {}", request.operation),
                details: None,
            }),
        };
        
        if let Some(reply) = msg.reply {
            let response_data = serde_json::to_vec(&error_response)?;
            client.publish(reply, response_data.into()).await?;
        }
    }
    
    Ok(())
} 