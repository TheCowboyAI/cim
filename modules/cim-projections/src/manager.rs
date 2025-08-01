use async_trait::async_trait;
use cim_events::{EventStore, StoredEvent};
use futures::stream::StreamExt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::{Projection, ProjectionError, ProjectionHandler, ProjectionStore, Result};

/// Manages multiple projections
#[derive(Clone)]
pub struct ProjectionManager {
    projections: Arc<Vec<Box<dyn DynamicProjectionHandler>>>,
}

impl ProjectionManager {
    /// Create a new projection manager
    pub fn new(projections: Vec<Box<dyn DynamicProjectionHandler>>) -> Self {
        Self {
            projections: Arc::new(projections),
        }
    }
    
    /// Handle a single event across all projections
    pub async fn handle_event<S>(&self, event: &StoredEvent, store: &S) -> Result<()>
    where
        S: DynamicProjectionStore,
    {
        for projection in self.projections.iter() {
            if projection.should_handle(event) {
                match projection.handle_event_dynamic(event, store).await {
                    Ok(()) => {
                        info!(
                            projection = projection.projection_name(),
                            event_type = event.event_type,
                            sequence = event.sequence,
                            "Projection updated"
                        );
                    }
                    Err(e) => {
                        error!(
                            projection = projection.projection_name(),
                            event_type = event.event_type,
                            sequence = event.sequence,
                            error = ?e,
                            "Failed to update projection"
                        );
                        // Continue with other projections
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Get projection names
    pub fn projection_names(&self) -> Vec<String> {
        self.projections
            .iter()
            .map(|p| p.projection_name().to_string())
            .collect()
    }
}

/// Dynamic projection handler that can work with type-erased stores
#[async_trait]
pub trait DynamicProjectionHandler: Send + Sync {
    fn projection_name(&self) -> &str;
    fn should_handle(&self, event: &StoredEvent) -> bool;
    async fn handle_event_dynamic(
        &self,
        event: &StoredEvent,
        store: &dyn DynamicProjectionStore,
    ) -> Result<()>;
}

/// Dynamic projection store that can work with multiple types
#[async_trait]
pub trait DynamicProjectionStore: Send + Sync {
    async fn get_position(&self, projection_name: &str) -> Result<u64>;
    async fn set_position(&self, projection_name: &str, position: u64) -> Result<()>;
}

/// Runs projections by subscribing to event streams
pub struct ProjectionRunner<E: EventStore> {
    event_store: Arc<E>,
    manager: ProjectionManager,
    store: Arc<dyn DynamicProjectionStore>,
}

impl<E: EventStore> ProjectionRunner<E> {
    pub fn new(
        event_store: E,
        manager: ProjectionManager,
        store: Arc<dyn DynamicProjectionStore>,
    ) -> Self {
        Self {
            event_store: Arc::new(event_store),
            manager,
            store,
        }
    }
    
    /// Run projections for a specific aggregate
    pub async fn run_for_aggregate(&self, aggregate_id: &str) -> Result<()> {
        // Get current positions for all projections
        let mut positions = Vec::new();
        for name in self.manager.projection_names() {
            let position = self.store.get_position(&name).await?;
            positions.push((name, position));
        }
        
        // Find the minimum position to start from
        let min_position = positions.iter().map(|(_, p)| *p).min().unwrap_or(0);
        
        // Subscribe to events from the minimum position
        let mut subscription = self
            .event_store
            .subscribe_to_events(aggregate_id)
            .await
            .map_err(|e| ProjectionError::EventProcessing(e.to_string()))?;
        
        info!(
            aggregate_id = aggregate_id,
            start_position = min_position,
            "Starting projection runner"
        );
        
        // Process events
        while let Some(event) = subscription.next().await {
            // Skip events we've already processed
            if event.sequence <= min_position {
                continue;
            }
            
            // Handle the event
            self.manager.handle_event(&event, self.store.as_ref()).await?;
        }
        
        Ok(())
    }
    
    /// Run projections for all aggregates (catch-up mode)
    pub async fn run_catch_up(&self, aggregate_ids: Vec<String>) -> Result<()> {
        for aggregate_id in aggregate_ids {
            info!(aggregate_id = %aggregate_id, "Running catch-up for aggregate");
            
            // Get the minimum position across all projections
            let mut min_position = u64::MAX;
            for name in self.manager.projection_names() {
                let position = self.store.get_position(&name).await?;
                min_position = min_position.min(position);
            }
            
            // Get all events from the minimum position
            let events = self
                .event_store
                .get_events(&aggregate_id, min_position, 1000)
                .await
                .map_err(|e| ProjectionError::EventProcessing(e.to_string()))?;
            
            info!(
                aggregate_id = %aggregate_id,
                event_count = events.len(),
                from_position = min_position,
                "Processing catch-up events"
            );
            
            // Process each event
            for event in events {
                self.manager.handle_event(&event, self.store.as_ref()).await?;
            }
        }
        
        Ok(())
    }
}

// Implementation helpers for concrete types
impl<H> DynamicProjectionHandler for H
where
    H: ProjectionHandler,
    H::Error: From<ProjectionError>,
{
    fn projection_name(&self) -> &str {
        ProjectionHandler::projection_name(self)
    }
    
    fn should_handle(&self, event: &StoredEvent) -> bool {
        ProjectionHandler::should_handle(self, event)
    }
    
    async fn handle_event_dynamic(
        &self,
        event: &StoredEvent,
        store: &dyn DynamicProjectionStore,
    ) -> Result<()> {
        // This is a simplified implementation
        // In a real system, you'd need type-safe store access
        warn!("Dynamic projection handler called - implement type-safe store access");
        Ok(())
    }
}

impl<T> DynamicProjectionStore for T
where
    T: ProjectionStore<serde_json::Value> + Send + Sync,
{
    async fn get_position(&self, projection_name: &str) -> Result<u64> {
        ProjectionStore::get_position(self, projection_name).await
    }
    
    async fn set_position(&self, projection_name: &str, position: u64) -> Result<()> {
        ProjectionStore::set_position(self, projection_name, position).await
    }
}