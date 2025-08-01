use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Core trait for all events in the system
pub trait Event: Debug + Clone + Send + Sync {
    /// Get the type of this event
    fn event_type(&self) -> &str;
    
    /// Get the aggregate ID this event belongs to
    fn aggregate_id(&self) -> &str;
}

/// Standard event header with correlation and causation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHeader {
    /// Unique identifier for this message
    pub message_id: String,
    
    /// Correlation ID to track related messages
    pub correlation_id: String,
    
    /// Causation ID to track what caused this event
    pub causation_id: Option<String>,
    
    /// When the event was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl EventHeader {
    /// Create a new event header with generated IDs
    pub fn new() -> Self {
        use uuid::Uuid;
        
        Self {
            message_id: Uuid::new_v4().to_string(),
            correlation_id: Uuid::new_v4().to_string(),
            causation_id: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Create a header with specific correlation
    pub fn with_correlation(correlation_id: String) -> Self {
        use uuid::Uuid;
        
        Self {
            message_id: Uuid::new_v4().to_string(),
            correlation_id,
            causation_id: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Create a header with correlation and causation
    pub fn with_causation(correlation_id: String, causation_id: String) -> Self {
        use uuid::Uuid;
        
        Self {
            message_id: Uuid::new_v4().to_string(),
            correlation_id,
            causation_id: Some(causation_id),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Event envelope that wraps any event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<E> {
    pub header: EventHeader,
    pub event: E,
}

impl<E: Event> EventEnvelope<E> {
    /// Create a new event envelope
    pub fn new(event: E) -> Self {
        Self {
            header: EventHeader::new(),
            event,
        }
    }
    
    /// Create with specific header
    pub fn with_header(event: E, header: EventHeader) -> Self {
        Self { header, event }
    }
}

/// Trait for aggregates that produce events
pub trait EventSourced {
    type Event: Event;
    type Error;
    
    /// Apply an event to update the aggregate state
    fn apply(&mut self, event: &Self::Event) -> Result<(), Self::Error>;
    
    /// Get the aggregate ID
    fn aggregate_id(&self) -> &str;
    
    /// Get the current version
    fn version(&self) -> u64;
    
    /// Increment the version
    fn increment_version(&mut self);
}

/// Commands that can be handled by aggregates
pub trait Command {
    type Aggregate: EventSourced;
    type Event;
    type Error;
    
    /// Execute the command on the aggregate
    fn execute(self, aggregate: &Self::Aggregate) -> Result<Vec<Self::Event>, Self::Error>;
}

/// Event metadata for tracking versions and sequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// The aggregate this event belongs to
    pub aggregate_id: String,
    
    /// The sequence number in the aggregate's event stream
    pub sequence: u64,
    
    /// The global sequence in the event store
    pub global_sequence: u64,
    
    /// When this event was stored
    pub stored_at: chrono::DateTime<chrono::Utc>,
}

/// Expected version for optimistic concurrency control
#[derive(Debug, Clone, Copy)]
pub enum ExpectedVersion {
    /// Any version is acceptable
    Any,
    
    /// The aggregate should not exist
    NoStream,
    
    /// The aggregate should be at this exact version
    Exact(u64),
}

/// Result of a command execution
#[derive(Debug)]
pub struct CommandResult<E> {
    /// Events produced by the command
    pub events: Vec<E>,
    
    /// New version after applying events
    pub version: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        id: String,
        data: String,
    }
    
    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            "TestEvent"
        }
        
        fn aggregate_id(&self) -> &str {
            &self.id
        }
    }
    
    #[test]
    fn event_header_generates_unique_ids() {
        let header1 = EventHeader::new();
        let header2 = EventHeader::new();
        
        assert_ne!(header1.message_id, header2.message_id);
        assert_ne!(header1.correlation_id, header2.correlation_id);
    }
    
    #[test]
    fn event_header_preserves_correlation() {
        let correlation_id = "test-correlation";
        let header = EventHeader::with_correlation(correlation_id.to_string());
        
        assert_eq!(header.correlation_id, correlation_id);
        assert!(header.causation_id.is_none());
    }
    
    #[test]
    fn event_envelope_wraps_event_correctly() {
        let event = TestEvent {
            id: "test-123".to_string(),
            data: "test data".to_string(),
        };
        
        let envelope = EventEnvelope::new(event.clone());
        
        assert_eq!(envelope.event.id, event.id);
        assert_eq!(envelope.event.data, event.data);
        assert!(!envelope.header.message_id.is_empty());
    }
}