use async_trait::async_trait;
use cim_events::StoredEvent;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{ProjectionError, ProjectionStore, Result};

/// Trait for projections that transform events into read models
#[async_trait]
pub trait ProjectionHandler: Send + Sync {
    /// The read model type this projection produces
    type ReadModel: Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>;
    
    /// Error type for this projection
    type Error: From<ProjectionError> + Send + Sync;
    
    /// Unique name for this projection
    fn projection_name(&self) -> &str;
    
    /// Handle a single event and update the read model
    async fn handle_event(
        &self,
        event: &StoredEvent,
        store: &dyn ProjectionStore<Self::ReadModel>,
    ) -> Result<(), Self::Error>;
    
    /// Rebuild the entire projection from a list of events
    async fn rebuild_from_events(
        &self,
        events: Vec<StoredEvent>,
        store: &dyn ProjectionStore<Self::ReadModel>,
    ) -> Result<(), Self::Error> {
        // Default implementation: clear and replay all events
        store.clear().await?;
        
        for event in events {
            self.handle_event(&event, store).await?;
        }
        
        Ok(())
    }
    
    /// Filter to determine if this projection should handle an event
    fn should_handle(&self, event: &StoredEvent) -> bool {
        // Default: handle all events
        true
    }
}

/// Metadata about a projection's state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionMetadata {
    /// Name of the projection
    pub name: String,
    
    /// Current position (last processed event sequence)
    pub position: u64,
    
    /// When the projection was last updated
    pub last_updated: chrono::DateTime<chrono::Utc>,
    
    /// Number of events processed
    pub events_processed: u64,
    
    /// Any errors encountered
    pub last_error: Option<String>,
}

/// A projection combines a handler with its metadata
pub struct Projection<H: ProjectionHandler> {
    pub handler: H,
    pub metadata: ProjectionMetadata,
}

impl<H: ProjectionHandler> Projection<H> {
    /// Create a new projection
    pub fn new(handler: H) -> Self {
        let metadata = ProjectionMetadata {
            name: handler.projection_name().to_string(),
            position: 0,
            last_updated: chrono::Utc::now(),
            events_processed: 0,
            last_error: None,
        };
        
        Self { handler, metadata }
    }
    
    /// Process an event and update metadata
    pub async fn process_event(
        &mut self,
        event: &StoredEvent,
        store: &dyn ProjectionStore<H::ReadModel>,
    ) -> Result<(), H::Error> {
        // Skip if we've already processed this event
        if event.sequence <= self.metadata.position {
            return Ok(());
        }
        
        // Skip if handler doesn't want this event
        if !self.handler.should_handle(event) {
            return Ok(());
        }
        
        // Process the event
        match self.handler.handle_event(event, store).await {
            Ok(()) => {
                self.metadata.position = event.sequence;
                self.metadata.events_processed += 1;
                self.metadata.last_updated = chrono::Utc::now();
                self.metadata.last_error = None;
                
                // Save position
                store.set_position(&self.metadata.name, event.sequence).await?;
                
                Ok(())
            }
            Err(e) => {
                self.metadata.last_error = Some(format!("{:?}", e));
                Err(e)
            }
        }
    }
    
    /// Load projection position from store
    pub async fn load_position<S>(&mut self, store: &S) -> Result<u64>
    where
        S: ProjectionStore<H::ReadModel>,
    {
        let position = store.get_position(&self.metadata.name).await?;
        self.metadata.position = position;
        Ok(position)
    }
}