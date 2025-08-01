//! # CIM Events
//! 
//! Event store implementation with CID chains using NATS JetStream.
//! 
//! This module provides:
//! - Event sourcing with NATS JetStream persistence
//! - CID chain validation for event integrity
//! - Correlation and causation ID tracking
//! - Real-time event subscriptions
//! - Optimistic concurrency control
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use cim_events::event_store::{EventStore, JetStreamEventStore};
//! use cim_events::domain::{Event, EventHeader};
//! 
//! #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
//! struct OrderCreated {
//!     order_id: String,
//!     customer: String,
//! }
//! 
//! impl Event for OrderCreated {
//!     fn event_type(&self) -> &str {
//!         "OrderCreated"
//!     }
//!     
//!     fn aggregate_id(&self) -> &str {
//!         &self.order_id
//!     }
//! }
//! 
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to NATS
//!     let client = async_nats::connect("nats://localhost:4222").await?;
//!     let jetstream = async_nats::jetstream::new(client);
//!     
//!     // Create event store
//!     let store = JetStreamEventStore::new(jetstream, "orders").await?;
//!     
//!     // Create and store event
//!     let event = OrderCreated {
//!         order_id: "order-123".to_string(),
//!         customer: "Alice".to_string(),
//!     };
//!     
//!     let metadata = store.append_event("order-123", event, None).await?;
//!     println!("Event stored with CID: {:?}", metadata.cid);
//!     
//!     Ok(())
//! }
//! ```

pub mod domain;
pub mod event_store;

// Re-export commonly used types
pub use domain::{Event, EventHeader, EventEnvelope, EventSourced, Command};
pub use event_store::{EventStore, JetStreamEventStore, StoredEvent, EventMetadata};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn module_exports_work() {
        // Verify that our re-exports are accessible
        use crate::{Event, EventHeader, EventStore};
        
        // This test just ensures the module structure is correct
        assert_eq!(std::mem::size_of::<EventHeader>(), std::mem::size_of::<EventHeader>());
    }
}