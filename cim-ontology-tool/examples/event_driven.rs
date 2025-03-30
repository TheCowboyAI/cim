use cim_ontology::events::{Event, EventError, EventHandler, EventType};
use cim_ontology::storage::neo4j::Neo4jConfig;

use async_trait::async_trait;
use serde_json::json;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

/// Custom event handler example
struct LoggingEventHandler {
    name: String,
}

impl LoggingEventHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl EventHandler for LoggingEventHandler {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn event_types(&self) -> Vec<EventType> {
        // Listen to all event types
        vec![
            EventType::OntologyCreated,
            EventType::OntologyUpdated,
            EventType::OntologyDeleted,
            EventType::OntologiesListed,
            EventType::TermAdded,
            EventType::TermUpdated,
            EventType::TermRemoved,
            EventType::TermsSearched,
            EventType::RelationshipAdded,
            EventType::RelationshipUpdated,
            EventType::RelationshipRemoved,
            EventType::SystemStarted,
            EventType::SystemShutdown,
            EventType::ErrorOccurred,
        ]
    }
    
    async fn handle_event(&self, event: Event) -> Result<(), EventError> {
        println!("[{}] Received event: {:?} (ID: {})", self.name, event.event_type, event.id);
        println!("  Source: {}", event.source);
        println!("  Timestamp: {}", event.timestamp);
        println!("  Payload: {}", serde_json::to_string_pretty(&event.payload).unwrap());
        println!("  Metadata: {:?}", event.metadata);
        println!();
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Event-Driven Architecture Example ===");
    
    // Initialize the event system
    let (event_bus, mut event_dispatcher, event_store) = cim_ontology::events::init_event_system(100);
    
    // Create a mock storage for testing
    println!("Creating mock storage...");
    let neo4j_config = Neo4jConfig {
        url: "bolt://localhost:7687".to_string(),
        username: "neo4j".to_string(),
        password: "password".to_string(),
        database: None,
        pool_size: 5,
    };
    
    // Note: In a real application, you would connect to a real Neo4j instance
    // For this example, we'll just demonstrate the event system
    // let storage = Neo4jStorage::new(neo4j_config).await?;
    // let storage = Arc::new(storage);
    
    // Register the custom logging handler (for demonstration)
    println!("Registering event handlers...");
    event_dispatcher.register_handler(LoggingEventHandler::new("logger")).await;
    
    // Start the event dispatcher
    println!("Starting event dispatcher...");
    event_dispatcher.start().await?;
    
    // Publish a system started event
    println!("Publishing system startup event...");
    let system_started_event = Event::new(
        EventType::SystemStarted,
        "example",
        json!({
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
    );
    
    event_bus.publish(system_started_event).await?;
    
    // Create an ontology event
    println!("Publishing ontology creation event...");
    let ontology_event = Event::new(
        EventType::OntologyCreated,
        "example",
        json!({
            "name": "Test Ontology",
            "description": "A test ontology created via events",
            "metadata": {
                "creator": "example",
                "created_at": chrono::Utc::now().to_rfc3339(),
            }
        }),
    );
    
    event_bus.publish(ontology_event).await?;
    
    // Add a term event
    println!("Publishing term addition event...");
    let ontology_id = Uuid::new_v4();
    let term_event = Event::new(
        EventType::TermAdded,
        "example",
        json!({
            "ontologyId": ontology_id.to_string(),
            "name": "TestTerm",
            "definition": "A test term created via events",
            "domain": "test",
        }),
    );
    
    event_bus.publish(term_event).await?;
    
    // Add a relationship event
    println!("Publishing relationship addition event...");
    let relationship_event = Event::new(
        EventType::RelationshipAdded,
        "example",
        json!({
            "ontologyId": ontology_id.to_string(),
            "sourceTermId": Uuid::new_v4().to_string(),
            "targetTermId": Uuid::new_v4().to_string(),
            "type": "IS_A",
            "strength": 0.95,
        }),
    );
    
    event_bus.publish(relationship_event).await?;
    
    // Wait for events to be processed
    sleep(Duration::from_secs(1)).await;
    
    // Shutdown event
    println!("Publishing system shutdown event...");
    let shutdown_event = Event::new(
        EventType::SystemShutdown,
        "example",
        json!({
            "reason": "Example completed",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
    );
    
    event_bus.publish(shutdown_event).await?;
    
    // Wait for final event to be processed
    sleep(Duration::from_secs(1)).await;
    
    // Stop the event dispatcher
    println!("Stopping event dispatcher...");
    event_dispatcher.stop().await?;
    
    println!("Example completed successfully!");
    
    Ok(())
} 