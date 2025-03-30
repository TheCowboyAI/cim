//! Event system for the CIM Ontology Tool
//!
//! This module implements an event-driven architecture for the application.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};
use uuid::Uuid;

pub mod handlers;

/// Event type enumeration representing all possible events in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    // Ontology events
    OntologyCreated,
    OntologyUpdated,
    OntologyDeleted,
    OntologyExtracted,
    OntologiesListed,
    
    // Term events
    TermAdded,
    TermUpdated,
    TermRemoved,
    TermsSearched,
    
    // Relationship events
    RelationshipAdded,
    RelationshipUpdated,
    RelationshipRemoved,
    
    // System events
    SystemStarted,
    SystemShutdown,
    
    // Error events
    ErrorOccurred,
}

// Implement Display for EventType so it can be used in format strings
impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::OntologyCreated => write!(f, "OntologyCreated"),
            EventType::OntologyUpdated => write!(f, "OntologyUpdated"),
            EventType::OntologyDeleted => write!(f, "OntologyDeleted"),
            EventType::OntologyExtracted => write!(f, "OntologyExtracted"),
            EventType::OntologiesListed => write!(f, "OntologiesListed"),
            EventType::TermAdded => write!(f, "TermAdded"),
            EventType::TermUpdated => write!(f, "TermUpdated"),
            EventType::TermRemoved => write!(f, "TermRemoved"),
            EventType::TermsSearched => write!(f, "TermsSearched"),
            EventType::RelationshipAdded => write!(f, "RelationshipAdded"),
            EventType::RelationshipUpdated => write!(f, "RelationshipUpdated"),
            EventType::RelationshipRemoved => write!(f, "RelationshipRemoved"),
            EventType::SystemStarted => write!(f, "SystemStarted"),
            EventType::SystemShutdown => write!(f, "SystemShutdown"),
            EventType::ErrorOccurred => write!(f, "ErrorOccurred"),
        }
    }
}

/// The event structure containing event metadata and payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event ID
    pub id: Uuid,
    
    /// Event type
    pub event_type: EventType,
    
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Event source (component that generated the event)
    pub source: String,
    
    /// Event payload
    pub payload: serde_json::Value,
    
    /// Event metadata
    pub metadata: HashMap<String, String>,
}

impl Event {
    /// Create a new event
    pub fn new(
        event_type: EventType, 
        source: &str, 
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            timestamp: chrono::Utc::now(),
            source: source.to_string(),
            payload,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the event
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Event handler trait for components that process events
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Returns the name of the handler
    fn name(&self) -> &str;
    
    /// Returns the event types this handler is interested in
    fn event_types(&self) -> Vec<EventType>;
    
    /// Process an event
    async fn handle_event(&self, event: Event) -> Result<(), EventError>;
}

/// Event bus for publishing and subscribing to events
#[derive(Debug, Clone)]
pub struct EventBus {
    /// Event sender
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }
    
    /// Publish an event to the bus
    pub async fn publish(&self, event: Event) -> Result<(), EventError> {
        match self.sender.send(event) {
            Ok(_) => Ok(()),
            Err(e) => Err(EventError::PublishError(format!("Failed to publish event: {}", e))),
        }
    }
    
    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

/// Event dispatcher that routes events to handlers
pub struct EventDispatcher {
    /// The event bus
    event_bus: EventBus,
    
    /// Event handlers
    handlers: Arc<RwLock<Vec<Arc<dyn EventHandler>>>>,
    
    /// Control channel for shutdown
    shutdown: Option<mpsc::Sender<()>>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            event_bus,
            handlers: Arc::new(RwLock::new(Vec::new())),
            shutdown: None,
        }
    }
    
    /// Register a handler with the dispatcher
    pub async fn register_handler<H>(&self, handler: H) 
    where 
        H: EventHandler + 'static
    {
        let mut handlers = self.handlers.write().await;
        handlers.push(Arc::new(handler));
    }
    
    /// Start the event dispatcher
    pub async fn start(&mut self) -> Result<(), EventError> {
        let handlers = Arc::clone(&self.handlers);
        let event_bus = self.event_bus.clone();
        let mut receiver = self.event_bus.subscribe();
        
        let (tx, mut rx) = mpsc::channel::<()>(1);
        self.shutdown = Some(tx);
        
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = rx.recv() => {
                        println!("Event dispatcher shutting down");
                        break;
                    }
                    result = receiver.recv() => {
                        match result {
                            Ok(event) => {
                                let handlers_lock = handlers.read().await;
                                for handler in handlers_lock.iter() {
                                    if handler.event_types().contains(&event.event_type) {
                                        let event_clone = event.clone();
                                        let handler_clone = Arc::clone(handler);
                                        let handler_name = handler.name().to_string();
                                        
                                        // Process event in a separate task
                                        tokio::spawn(async move {
                                            if let Err(e) = handler_clone.handle_event(event_clone).await {
                                                eprintln!("Error in handler {}: {}", handler_name, e);
                                            }
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error receiving event: {}", e);
                                // Resubscribe on error
                                receiver = event_bus.subscribe();
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Stop the event dispatcher
    pub async fn stop(&self) -> Result<(), EventError> {
        if let Some(tx) = &self.shutdown {
            tx.send(()).await.map_err(|_| EventError::DispatcherError("Failed to stop dispatcher".to_string()))?;
        }
        Ok(())
    }
}

/// Event store for persisting events
pub struct EventStore {
    /// Events storage
    events: Arc<RwLock<Vec<Event>>>,
}

impl EventStore {
    /// Create a new event store
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Store an event
    pub async fn store_event(&self, event: Event) -> Result<(), EventError> {
        let mut events = self.events.write().await;
        events.push(event);
        Ok(())
    }
    
    /// Get all events
    pub async fn get_events(&self) -> Result<Vec<Event>, EventError> {
        let events = self.events.read().await;
        Ok(events.clone())
    }
    
    /// Get events by type
    pub async fn get_events_by_type(&self, event_type: EventType) -> Result<Vec<Event>, EventError> {
        let events = self.events.read().await;
        Ok(events.iter().filter(|e| e.event_type == event_type).cloned().collect())
    }
}

/// Event errors
#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("Event publish error: {0}")]
    PublishError(String),
    
    #[error("Event dispatch error: {0}")]
    DispatcherError(String),
    
    #[error("Event handler error: {0}")]
    HandlerError(String),
    
    #[error("Event store error: {0}")]
    StoreError(String),
}

/// Initialize the event system
pub fn init_event_system(capacity: usize) -> (EventBus, EventDispatcher, EventStore) {
    let event_bus = EventBus::new(capacity);
    let event_dispatcher = EventDispatcher::new(event_bus.clone());
    let event_store = EventStore::new();
    
    (event_bus, event_dispatcher, event_store)
} 