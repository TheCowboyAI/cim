#[cfg(test)]
mod event_store_tests {
    use super::*;
    use async_nats::jetstream::{self, stream::Config as StreamConfig};
    use cim_events::event_store::{EventStore, JetStreamEventStore, StoredEvent, EventMetadata};
    use cim_events::domain::{Event, EventHeader};
    use cid::Cid;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    #[tokio::test]
    async fn event_store_should_append_and_retrieve_events() {
        // Given
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let jetstream = jetstream::new(client);
        let store = JetStreamEventStore::new(jetstream, "test-events").await.unwrap();
        
        let aggregate_id = Uuid::new_v4().to_string();
        let event = TestEvent {
            id: aggregate_id.clone(),
            data: "test data".to_string(),
        };
        
        // When
        let metadata = store.append_event(&aggregate_id, event.clone(), None).await.unwrap();
        
        // Then
        assert_eq!(metadata.sequence, 1);
        assert!(metadata.cid.is_some());
        
        // Verify we can retrieve the event
        let events = store.get_events(&aggregate_id, 0, 10).await.unwrap();
        assert_eq!(events.len(), 1);
        
        let stored_event = &events[0];
        assert_eq!(stored_event.sequence, 1);
        assert_eq!(stored_event.aggregate_id, aggregate_id);
    }

    #[tokio::test]
    async fn event_store_should_maintain_cid_chain() {
        // Given
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let jetstream = jetstream::new(client);
        let store = JetStreamEventStore::new(jetstream, "test-events").await.unwrap();
        
        let aggregate_id = Uuid::new_v4().to_string();
        
        // When - append multiple events
        let event1 = TestEvent {
            id: aggregate_id.clone(),
            data: "event 1".to_string(),
        };
        let metadata1 = store.append_event(&aggregate_id, event1, None).await.unwrap();
        
        let event2 = TestEvent {
            id: aggregate_id.clone(),
            data: "event 2".to_string(),
        };
        let metadata2 = store.append_event(&aggregate_id, event2, Some(metadata1.cid.clone().unwrap())).await.unwrap();
        
        // Then - verify CID chain
        assert!(metadata1.cid.is_some());
        assert!(metadata2.cid.is_some());
        assert_ne!(metadata1.cid, metadata2.cid);
        
        // Verify the second event references the first via parent CID
        let events = store.get_events(&aggregate_id, 0, 10).await.unwrap();
        assert_eq!(events.len(), 2);
        
        assert!(events[0].parent_cid.is_none());
        assert_eq!(events[1].parent_cid, metadata1.cid);
    }

    #[tokio::test]
    async fn event_store_should_handle_correlation_and_causation() {
        // Given
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let jetstream = jetstream::new(client);
        let store = JetStreamEventStore::new(jetstream, "test-events").await.unwrap();
        
        let aggregate_id = Uuid::new_v4().to_string();
        let correlation_id = Uuid::new_v4().to_string();
        let causation_id = Uuid::new_v4().to_string();
        
        let event = TestEvent {
            id: aggregate_id.clone(),
            data: "correlated event".to_string(),
        };
        
        let header = EventHeader {
            message_id: Uuid::new_v4().to_string(),
            correlation_id: correlation_id.clone(),
            causation_id: Some(causation_id.clone()),
            timestamp: chrono::Utc::now(),
        };
        
        // When
        let metadata = store.append_event_with_header(&aggregate_id, event, header, None).await.unwrap();
        
        // Then
        let events = store.get_events(&aggregate_id, 0, 10).await.unwrap();
        assert_eq!(events.len(), 1);
        
        let stored_event = &events[0];
        assert_eq!(stored_event.header.correlation_id, correlation_id);
        assert_eq!(stored_event.header.causation_id, Some(causation_id));
    }

    #[tokio::test]
    async fn event_store_should_support_event_replay_from_sequence() {
        // Given
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let jetstream = jetstream::new(client);
        let store = JetStreamEventStore::new(jetstream, "test-events").await.unwrap();
        
        let aggregate_id = Uuid::new_v4().to_string();
        
        // Append 5 events
        for i in 1..=5 {
            let event = TestEvent {
                id: aggregate_id.clone(),
                data: format!("event {}", i),
            };
            store.append_event(&aggregate_id, event, None).await.unwrap();
        }
        
        // When - replay from sequence 3
        let events = store.get_events(&aggregate_id, 3, 10).await.unwrap();
        
        // Then
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].sequence, 3);
        assert_eq!(events[1].sequence, 4);
        assert_eq!(events[2].sequence, 5);
    }

    #[tokio::test]
    async fn event_store_should_validate_cid_chain_integrity() {
        // Given
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let jetstream = jetstream::new(client);
        let store = JetStreamEventStore::new(jetstream, "test-events").await.unwrap();
        
        let aggregate_id = Uuid::new_v4().to_string();
        
        // When - try to append with invalid parent CID
        let event = TestEvent {
            id: aggregate_id.clone(),
            data: "event with bad parent".to_string(),
        };
        
        let fake_cid = Cid::default(); // Invalid parent
        let result = store.append_event(&aggregate_id, event, Some(fake_cid)).await;
        
        // Then
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EventStoreError::InvalidCidChain(_)));
    }

    #[tokio::test]
    async fn event_store_should_support_stream_subscriptions() {
        // Given
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let jetstream = jetstream::new(client);
        let store = JetStreamEventStore::new(jetstream, "test-events").await.unwrap();
        
        let aggregate_id = Uuid::new_v4().to_string();
        
        // When - subscribe to events
        let mut subscription = store.subscribe_to_events(&aggregate_id).await.unwrap();
        
        // Append an event
        let event = TestEvent {
            id: aggregate_id.clone(),
            data: "streamed event".to_string(),
        };
        store.append_event(&aggregate_id, event.clone(), None).await.unwrap();
        
        // Then - should receive the event
        let received = subscription.next().await.unwrap();
        assert_eq!(received.aggregate_id, aggregate_id);
        assert_eq!(received.sequence, 1);
    }

    #[tokio::test]
    async fn event_store_should_handle_concurrent_appends() {
        // Given
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let jetstream = jetstream::new(client);
        let store = JetStreamEventStore::new(jetstream, "test-events").await.unwrap();
        
        let aggregate_id = Uuid::new_v4().to_string();
        
        // When - append events concurrently
        let mut handles = vec![];
        
        for i in 0..10 {
            let store_clone = store.clone();
            let aggregate_id_clone = aggregate_id.clone();
            
            let handle = tokio::spawn(async move {
                let event = TestEvent {
                    id: aggregate_id_clone,
                    data: format!("concurrent event {}", i),
                };
                store_clone.append_event(&aggregate_id_clone, event, None).await
            });
            
            handles.push(handle);
        }
        
        // Wait for all to complete
        for handle in handles {
            handle.await.unwrap().unwrap();
        }
        
        // Then - all events should be stored with unique sequences
        let events = store.get_events(&aggregate_id, 0, 20).await.unwrap();
        assert_eq!(events.len(), 10);
        
        // Verify sequences are unique and consecutive
        let mut sequences: Vec<u64> = events.iter().map(|e| e.sequence).collect();
        sequences.sort();
        assert_eq!(sequences, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}