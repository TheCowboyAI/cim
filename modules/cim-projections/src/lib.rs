//! # CIM Projections
//! 
//! CQRS projection implementation for building read models from event streams.
//! 
//! This module provides:
//! - Projection handlers for transforming events into read models
//! - Multiple projection store backends (in-memory, PostgreSQL, Redis)
//! - Automatic position tracking for resumable projections
//! - Query capabilities for read models
//! - Concurrent projection processing
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use cim_projections::{ProjectionHandler, ProjectionStore, InMemoryProjectionStore};
//! use cim_events::StoredEvent;
//! 
//! struct OrderSummaryProjection;
//! 
//! #[async_trait::async_trait]
//! impl ProjectionHandler for OrderSummaryProjection {
//!     type ReadModel = OrderSummary;
//!     type Error = ProjectionError;
//!     
//!     fn projection_name(&self) -> &str {
//!         "order_summary"
//!     }
//!     
//!     async fn handle_event(
//!         &self,
//!         event: &StoredEvent,
//!         store: &dyn ProjectionStore<Self::ReadModel>,
//!     ) -> Result<(), Self::Error> {
//!         // Transform event into read model update
//!         Ok(())
//!     }
//! }
//! ```

mod projection;
mod store;
mod manager;
mod error;

// Re-export main types
pub use projection::{Projection, ProjectionHandler};
pub use store::{ProjectionStore, InMemoryProjectionStore, PostgresProjectionStore};
pub use manager::{ProjectionManager, ProjectionRunner};
pub use error::{ProjectionError, Result};

// Re-export async trait for convenience
pub use async_trait::async_trait;