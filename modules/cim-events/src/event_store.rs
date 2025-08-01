use async_nats::jetstream::{self, consumer, stream};
use async_trait::async_trait;
use cid::Cid;
use ipfs_api::{IpfsApi, IpfsClient};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

// Import cim-subject for proper NATS subject handling
use cim_subject::{Subject, SubjectBuilder, MessageIdentity};

use crate::domain::{Event, EventHeader};

#[derive(Error, Debug)]
pub enum EventStoreError {
    #[error("NATS error: {0}")]
    Nats(#[from] async_nats::Error),
    
    #[error("JetStream error: {0}")]
    JetStream(#[from] jetstream::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IPFS error: {0}")]
    Ipfs(String),
    
    #[error("Invalid CID chain: {0}")]
    InvalidCidChain(String),
    
    #[error("Event not found: {0}")]
    EventNotFound(String),
    
    #[error("Concurrent modification detected")]
    ConcurrentModification,
}

pub type Result<T> = std::result::Result<T, EventStoreError>;

/// Metadata returned after storing an event
#[derive(Debug, Clone)]
pub struct EventMetadata {
    pub sequence: u64,
    pub cid: Option<Cid>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// A stored event with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub sequence: u64,
    pub aggregate_id: String,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub header: EventHeader,
    pub cid: Option<String>,
    pub parent_cid: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Event store trait for appending and retrieving events
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Append an event to the store
    async fn append_event<E: Event + Serialize + Send>(
        &self,
        aggregate_id: &str,
        event: E,
        parent_cid: Option<Cid>,
    ) -> Result<EventMetadata>;
    
    /// Append an event with custom header
    async fn append_event_with_header<E: Event + Serialize + Send>(
        &self,
        aggregate_id: &str,
        event: E,
        header: EventHeader,
        parent_cid: Option<Cid>,
    ) -> Result<EventMetadata>;
    
    /// Get events for an aggregate
    async fn get_events(
        &self,
        aggregate_id: &str,
        from_sequence: u64,
        limit: usize,
    ) -> Result<Vec<StoredEvent>>;
    
    /// Subscribe to events for an aggregate
    async fn subscribe_to_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Box<dyn Stream<Item = StoredEvent> + Send + Unpin>>;
    
    /// Validate CID chain integrity
    async fn validate_cid_chain(
        &self,
        aggregate_id: &str,
    ) -> Result<bool>;
}

/// JetStream-based event store implementation
#[derive(Clone)]
pub struct JetStreamEventStore {
    jetstream: jetstream::Context,
    stream_name: String,
    ipfs_client: Option<Arc<IpfsClient>>,
    subject_builder: SubjectBuilder,
}

impl JetStreamEventStore {
    /// Create a new JetStream event store
    pub async fn new(
        jetstream: jetstream::Context,
        stream_name: &str,
    ) -> Result<Self> {
        // Create or update the stream configuration
        let stream_config = stream::Config {
            name: stream_name.to_string(),
            subjects: vec![
                format!("events.>"),  // All events
            ],
            retention: stream::RetentionPolicy::Limits,
            storage: stream::StorageType::File,
            max_age: std::time::Duration::from_days(365),
            duplicate_window: std::time::Duration::from_secs(120),
            ..Default::default()
        };
        
        // Create or update stream
        match jetstream.create_stream(stream_config.clone()).await {
            Ok(_) => {},
            Err(jetstream::Error::StreamNameExists) => {
                jetstream.update_stream(stream_config).await?;
            },
            Err(e) => return Err(e.into()),
        }
        
        // Initialize subject builder for event routing
        let subject_builder = SubjectBuilder::new("events");
        
        Ok(Self {
            jetstream,
            stream_name: stream_name.to_string(),
            ipfs_client: None, // Can be added later for CID storage
            subject_builder,
        })
    }
    
    /// Enable IPFS for CID storage
    pub fn with_ipfs(mut self, ipfs_client: IpfsClient) -> Self {
        self.ipfs_client = Some(Arc::new(ipfs_client));
        self
    }
    
    /// Generate subject for an event
    fn event_subject(&self, aggregate_id: &str, event_type: &str) -> Subject {
        self.subject_builder
            .with_entity(aggregate_id)
            .with_action(event_type)
            .build()
    }
    
    /// Store event data in IPFS and return CID
    async fn store_in_ipfs(&self, data: &[u8]) -> Result<Cid> {
        if let Some(ipfs) = &self.ipfs_client {
            match ipfs.add(data).await {
                Ok(res) => {
                    let cid = Cid::try_from(res.hash.as_str())
                        .map_err(|e| EventStoreError::Ipfs(e.to_string()))?;
                    Ok(cid)
                }
                Err(e) => Err(EventStoreError::Ipfs(e.to_string()))
            }
        } else {
            // Generate a deterministic CID without IPFS
            let cid = self.generate_local_cid(data);
            Ok(cid)
        }
    }
    
    /// Generate a local CID for testing without IPFS
    fn generate_local_cid(&self, data: &[u8]) -> Cid {
        use sha2::{Sha256, Digest};
        let hash = Sha256::digest(data);
        
        // Create a CID v1 with raw codec
        cid::Cid::new_v1(0x55, multihash::Multihash::wrap(0x12, &hash).unwrap())
    }
    
    /// Get the latest event for CID chain validation
    async fn get_latest_event(&self, aggregate_id: &str) -> Result<Option<StoredEvent>> {
        let events = self.get_events(aggregate_id, 0, 1).await?;
        Ok(events.into_iter().next())
    }
}

#[async_trait]
impl EventStore for JetStreamEventStore {
    async fn append_event<E: Event + Serialize + Send>(
        &self,
        aggregate_id: &str,
        event: E,
        parent_cid: Option<Cid>,
    ) -> Result<EventMetadata> {
        let header = EventHeader {
            message_id: Uuid::new_v4().to_string(),
            correlation_id: Uuid::new_v4().to_string(),
            causation_id: None,
            timestamp: chrono::Utc::now(),
        };
        
        self.append_event_with_header(aggregate_id, event, header, parent_cid).await
    }
    
    async fn append_event_with_header<E: Event + Serialize + Send>(
        &self,
        aggregate_id: &str,
        event: E,
        header: EventHeader,
        parent_cid: Option<Cid>,
    ) -> Result<EventMetadata> {
        // Validate parent CID if provided
        if let Some(ref parent) = parent_cid {
            let latest = self.get_latest_event(aggregate_id).await?;
            if let Some(latest_event) = latest {
                if let Some(latest_cid_str) = &latest_event.cid {
                    let latest_cid = Cid::try_from(latest_cid_str.as_str())
                        .map_err(|e| EventStoreError::InvalidCidChain(e.to_string()))?;
                    
                    if &latest_cid != parent {
                        return Err(EventStoreError::InvalidCidChain(
                            "Parent CID does not match latest event".to_string()
                        ));
                    }
                }
            }
        }
        
        // Create stored event
        let event_data = serde_json::to_value(&event)?;
        let mut stored_event = StoredEvent {
            sequence: 0, // Will be set by JetStream
            aggregate_id: aggregate_id.to_string(),
            event_type: event.event_type().to_string(),
            event_data,
            header: header.clone(),
            cid: None,
            parent_cid: parent_cid.map(|c| c.to_string()),
            timestamp: chrono::Utc::now(),
        };
        
        // Serialize for storage
        let event_bytes = serde_json::to_vec(&stored_event)?;
        
        // Generate CID
        let cid = self.store_in_ipfs(&event_bytes).await?;
        stored_event.cid = Some(cid.to_string());
        
        // Re-serialize with CID
        let final_bytes = serde_json::to_vec(&stored_event)?;
        
        // Create NATS headers with message identity
        let mut headers = async_nats::HeaderMap::new();
        headers.insert("X-Message-ID", header.message_id.as_str());
        headers.insert("X-Correlation-ID", header.correlation_id.as_str());
        if let Some(ref causation) = header.causation_id {
            headers.insert("X-Causation-ID", causation.as_str());
        }
        headers.insert("X-CID", cid.to_string().as_str());
        if let Some(ref parent) = parent_cid {
            headers.insert("X-Parent-CID", parent.to_string().as_str());
        }
        
        // Publish to JetStream
        let subject = self.event_subject(aggregate_id, event.event_type());
        let ack = self.jetstream
            .publish_with_headers(subject.to_string(), headers, final_bytes.into())
            .await?
            .await?;
        
        Ok(EventMetadata {
            sequence: ack.sequence,
            cid: Some(cid),
            timestamp: stored_event.timestamp,
        })
    }
    
    async fn get_events(
        &self,
        aggregate_id: &str,
        from_sequence: u64,
        limit: usize,
    ) -> Result<Vec<StoredEvent>> {
        // Create consumer for reading events
        let consumer_config = consumer::pull::Config {
            filter_subject: format!("events.{}.>", aggregate_id),
            deliver_policy: consumer::DeliverPolicy::ByStartSequence {
                start_sequence: from_sequence.max(1),
            },
            ..Default::default()
        };
        
        let consumer = self.jetstream
            .create_consumer(&self.stream_name, consumer_config)
            .await?;
        
        let mut events = Vec::new();
        let mut messages = consumer.messages().await?;
        
        for _ in 0..limit {
            match messages.try_next().await {
                Ok(Some(msg)) => {
                    let mut event: StoredEvent = serde_json::from_slice(&msg.payload)?;
                    event.sequence = msg.info().stream_sequence;
                    events.push(event);
                    msg.ack().await?;
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }
        
        Ok(events)
    }
    
    async fn subscribe_to_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Box<dyn Stream<Item = StoredEvent> + Send + Unpin>> {
        // Create push consumer for real-time subscription
        let consumer_config = consumer::push::Config {
            durable_name: Some(format!("sub-{}", Uuid::new_v4())),
            filter_subject: format!("events.{}.>", aggregate_id),
            deliver_policy: consumer::DeliverPolicy::New,
            ..Default::default()
        };
        
        let consumer = self.jetstream
            .create_consumer(&self.stream_name, consumer_config)
            .await?;
        
        let messages = consumer.messages().await?;
        
        // Create a stream that converts messages to StoredEvents
        let event_stream = EventStream {
            messages: Box::pin(messages),
        };
        
        Ok(Box::new(event_stream))
    }
    
    async fn validate_cid_chain(
        &self,
        aggregate_id: &str,
    ) -> Result<bool> {
        let events = self.get_events(aggregate_id, 0, 1000).await?;
        
        if events.is_empty() {
            return Ok(true);
        }
        
        // First event should have no parent
        if events[0].parent_cid.is_some() {
            return Ok(false);
        }
        
        // Validate chain
        for i in 1..events.len() {
            let expected_parent = &events[i - 1].cid;
            let actual_parent = &events[i].parent_cid;
            
            if expected_parent != actual_parent {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}

// Stream implementation for event subscriptions
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

struct EventStream {
    messages: Pin<Box<dyn Stream<Item = std::result::Result<async_nats::Message, async_nats::Error>> + Send>>,
}

impl Stream for EventStream {
    type Item = StoredEvent;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.messages.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(msg))) => {
                match serde_json::from_slice::<StoredEvent>(&msg.payload) {
                    Ok(mut event) => {
                        if let Some(info) = msg.info() {
                            event.sequence = info.stream_sequence;
                        }
                        Poll::Ready(Some(event))
                    }
                    Err(_) => Poll::Ready(None),
                }
            }
            Poll::Ready(Some(Err(_))) => Poll::Ready(None),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}