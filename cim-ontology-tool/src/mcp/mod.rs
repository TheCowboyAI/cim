//! MCP (Model Context Protocol) implementation
//!
//! This module provides the MCP server implementation for the ontology tool.

use crate::events::{Event, EventBus, EventType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod server;
pub mod handler;
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

/// Helper function to convert an MCP request to an Event
pub fn request_to_event(request: &MCPRequest) -> Option<Event> {
    // Map MCP operations to event types
    let event_type = match request.operation.as_str() {
        "create_ontology" => EventType::OntologyCreated,
        "update_ontology" => EventType::OntologyUpdated,
        "delete_ontology" => EventType::OntologyDeleted,
        "list_ontologies" => EventType::OntologiesListed,
        "add_term" => EventType::TermAdded,
        "update_term" => EventType::TermUpdated,
        "remove_term" => EventType::TermRemoved,
        "search_terms" => EventType::TermsSearched,
        "add_relationship" => EventType::RelationshipAdded,
        "update_relationship" => EventType::RelationshipUpdated,
        "remove_relationship" => EventType::RelationshipRemoved,
        _ => return None,
    };
    
    // Create the event
    let event = Event::new(
        event_type,
        "mcp_server",
        serde_json::to_value(&request.params).unwrap_or_default(),
    ).with_metadata("request_id", &request.id);
    
    Some(event)
}

/// Helper function to handle events asynchronously and create responses
pub async fn handle_event_and_create_response(
    event_bus: &EventBus,
    request: &MCPRequest,
    event: Event,
) -> MCPResponse {
    // Publish the event
    match event_bus.publish(event.clone()).await {
        Ok(_) => {
            // Create a response for the client
            // For now, we'll just acknowledge that the event was published
            MCPResponse {
                request_id: request.id.clone(),
                status: MCPStatus::Success,
                data: Some(serde_json::json!({
                    "event_id": event.id.to_string(),
                    "message": format!("Event {} processed successfully", event.event_type),
                })),
                error: None,
            }
        },
        Err(e) => {
            // Create an error response
            MCPResponse {
                request_id: request.id.clone(),
                status: MCPStatus::Error,
                data: None,
                error: Some(MCPError {
                    code: "EVENT_PROCESSING_ERROR".to_string(),
                    message: format!("Failed to process event: {}", e),
                    details: None,
                }),
            }
        },
    }
} 