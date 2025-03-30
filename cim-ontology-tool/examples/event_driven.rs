// examples/event_driven.rs - Example of event-driven architecture

use cim_ontology::events::{Event, EventBus, EventType};
use cim_ontology::storage::neo4j::Neo4jConfig;
use std::time::Duration;
use tokio::time::sleep;

/// Example event handler function
async fn handle_event(event: &Event) {
    println!(
        "Handling event: {} (id: {})",
        event.event_type,
        event.id
    );
    
    // Simulate processing time
    sleep(Duration::from_millis(100)).await;
    
    println!(
        "Event {} processed successfully",
        event.id
    );
}

/// Example function to generate events
async fn generate_events(event_bus: &EventBus, count: usize) {
    println!("Generating {} events", count);
    
    for i in 0..count {
        // Alternate between different event types
        let event_type = match i % 3 {
            0 => EventType::OntologyCreated,
            1 => EventType::TermAdded,
            _ => EventType::RelationshipAdded,
        };
        
        // Create the event with some payload
        let event = Event::new(
            event_type,
            "event_generator",
            serde_json::json!({
                "index": i,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }),
        );
        
        // Publish the event
        match event_bus.publish(event.clone()).await {
            Ok(_) => println!("Published event {}: {}", i, event.event_type),
            Err(e) => eprintln!("Failed to publish event {}: {}", i, e),
        }
        
        // Add a short delay between events
        sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Event-driven architecture example");
    
    // Initialize the event system
    println!("Initializing event system...");
    let (event_bus, mut event_dispatcher, _event_store) = cim_ontology::events::init_event_system(100);
    
    // Configure Neo4j (not actually connected in this example)
    let _neo4j_config = Neo4jConfig {
        url: "bolt://localhost:7687".to_string(),
        username: "neo4j".to_string(),
        password: "password".to_string(),
        database: None,
        pool_size: 5,
    };
    
    // Start the event dispatcher
    tokio::spawn(async move {
        println!("Starting event dispatcher...");
        if let Err(e) = event_dispatcher.start().await {
            eprintln!("Event dispatcher error: {}", e);
        }
    });
    
    // Wait for the dispatcher to start
    sleep(Duration::from_millis(500)).await;
    
    // Register an event handler (normally this would use the handler trait)
    println!("Setting up event handlers...");
    let event_bus_clone = event_bus.clone();
    tokio::spawn(async move {
        let mut receiver = event_bus_clone.subscribe().await.unwrap();
        println!("Event handler listening for events...");
        
        while let Some(event) = receiver.recv().await {
            handle_event(&event).await;
        }
    });
    
    // Wait for handlers to be set up
    sleep(Duration::from_millis(500)).await;
    
    // Generate and process events
    generate_events(&event_bus, 10).await;
    
    // Give time for events to be processed
    println!("Waiting for events to be processed...");
    sleep(Duration::from_secs(2)).await;
    
    println!("Example completed successfully");
    Ok(())
} 