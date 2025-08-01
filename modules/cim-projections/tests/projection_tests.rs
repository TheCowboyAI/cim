#[cfg(test)]
mod projection_tests {
    use cim_projections::{
        Projection, ProjectionHandler, ProjectionStore, ProjectionError,
        InMemoryProjectionStore, ProjectionManager
    };
    use cim_events::{Event, EventHeader, StoredEvent};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use uuid::Uuid;

    // Test events
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ProductCreated {
        product_id: String,
        name: String,
        price: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ProductPriceChanged {
        product_id: String,
        old_price: f64,
        new_price: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct ProductDeleted {
        product_id: String,
    }

    impl Event for ProductCreated {
        fn event_type(&self) -> &str {
            "ProductCreated"
        }
        fn aggregate_id(&self) -> &str {
            &self.product_id
        }
    }

    impl Event for ProductPriceChanged {
        fn event_type(&self) -> &str {
            "ProductPriceChanged"
        }
        fn aggregate_id(&self) -> &str {
            &self.product_id
        }
    }

    impl Event for ProductDeleted {
        fn event_type(&self) -> &str {
            "ProductDeleted"
        }
        fn aggregate_id(&self) -> &str {
            &self.product_id
        }
    }

    // Test projection
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct ProductCatalogEntry {
        product_id: String,
        name: String,
        current_price: f64,
        price_history: Vec<f64>,
        last_updated: chrono::DateTime<chrono::Utc>,
    }

    struct ProductCatalogProjection;

    #[async_trait::async_trait]
    impl ProjectionHandler for ProductCatalogProjection {
        type ReadModel = ProductCatalogEntry;
        type Error = ProjectionError;

        fn projection_name(&self) -> &str {
            "product_catalog"
        }

        async fn handle_event(
            &self,
            event: &StoredEvent,
            store: &dyn ProjectionStore<Self::ReadModel>,
        ) -> Result<(), Self::Error> {
            match event.event_type.as_str() {
                "ProductCreated" => {
                    let data: ProductCreated = serde_json::from_value(event.event_data.clone())?;
                    let entry = ProductCatalogEntry {
                        product_id: data.product_id.clone(),
                        name: data.name,
                        current_price: data.price,
                        price_history: vec![data.price],
                        last_updated: event.timestamp,
                    };
                    store.save(&data.product_id, entry).await?;
                }
                "ProductPriceChanged" => {
                    if let Some(mut entry) = store.get(&event.aggregate_id).await? {
                        let data: ProductPriceChanged = serde_json::from_value(event.event_data.clone())?;
                        entry.current_price = data.new_price;
                        entry.price_history.push(data.new_price);
                        entry.last_updated = event.timestamp;
                        store.save(&event.aggregate_id, entry).await?;
                    }
                }
                "ProductDeleted" => {
                    store.delete(&event.aggregate_id).await?;
                }
                _ => {}
            }
            Ok(())
        }

        async fn rebuild_from_events(
            &self,
            events: Vec<StoredEvent>,
            store: &dyn ProjectionStore<Self::ReadModel>,
        ) -> Result<(), Self::Error> {
            // Clear existing data
            store.clear().await?;
            
            // Replay all events
            for event in events {
                self.handle_event(&event, store).await?;
            }
            
            Ok(())
        }
    }

    #[tokio::test]
    async fn projection_should_handle_product_created() {
        // Given
        let projection = ProductCatalogProjection;
        let store = InMemoryProjectionStore::new();
        
        let event = StoredEvent {
            sequence: 1,
            aggregate_id: "prod-123".to_string(),
            event_type: "ProductCreated".to_string(),
            event_data: serde_json::to_value(ProductCreated {
                product_id: "prod-123".to_string(),
                name: "Widget".to_string(),
                price: 19.99,
            }).unwrap(),
            header: EventHeader::new(),
            cid: None,
            parent_cid: None,
            timestamp: chrono::Utc::now(),
        };

        // When
        projection.handle_event(&event, &store).await.unwrap();

        // Then
        let entry = store.get("prod-123").await.unwrap().unwrap();
        assert_eq!(entry.product_id, "prod-123");
        assert_eq!(entry.name, "Widget");
        assert_eq!(entry.current_price, 19.99);
        assert_eq!(entry.price_history, vec![19.99]);
    }

    #[tokio::test]
    async fn projection_should_handle_price_changes() {
        // Given
        let projection = ProductCatalogProjection;
        let store = InMemoryProjectionStore::new();
        
        // Create product first
        let create_event = StoredEvent {
            sequence: 1,
            aggregate_id: "prod-123".to_string(),
            event_type: "ProductCreated".to_string(),
            event_data: serde_json::to_value(ProductCreated {
                product_id: "prod-123".to_string(),
                name: "Widget".to_string(),
                price: 19.99,
            }).unwrap(),
            header: EventHeader::new(),
            cid: None,
            parent_cid: None,
            timestamp: chrono::Utc::now(),
        };
        projection.handle_event(&create_event, &store).await.unwrap();

        // When - price changes
        let price_change_event = StoredEvent {
            sequence: 2,
            aggregate_id: "prod-123".to_string(),
            event_type: "ProductPriceChanged".to_string(),
            event_data: serde_json::to_value(ProductPriceChanged {
                product_id: "prod-123".to_string(),
                old_price: 19.99,
                new_price: 24.99,
            }).unwrap(),
            header: EventHeader::new(),
            cid: None,
            parent_cid: None,
            timestamp: chrono::Utc::now(),
        };
        projection.handle_event(&price_change_event, &store).await.unwrap();

        // Then
        let entry = store.get("prod-123").await.unwrap().unwrap();
        assert_eq!(entry.current_price, 24.99);
        assert_eq!(entry.price_history, vec![19.99, 24.99]);
    }

    #[tokio::test]
    async fn projection_should_handle_product_deletion() {
        // Given
        let projection = ProductCatalogProjection;
        let store = InMemoryProjectionStore::new();
        
        // Create product
        store.save("prod-123", ProductCatalogEntry {
            product_id: "prod-123".to_string(),
            name: "Widget".to_string(),
            current_price: 19.99,
            price_history: vec![19.99],
            last_updated: chrono::Utc::now(),
        }).await.unwrap();

        // When - delete product
        let delete_event = StoredEvent {
            sequence: 3,
            aggregate_id: "prod-123".to_string(),
            event_type: "ProductDeleted".to_string(),
            event_data: serde_json::to_value(ProductDeleted {
                product_id: "prod-123".to_string(),
            }).unwrap(),
            header: EventHeader::new(),
            cid: None,
            parent_cid: None,
            timestamp: chrono::Utc::now(),
        };
        projection.handle_event(&delete_event, &store).await.unwrap();

        // Then
        let entry = store.get("prod-123").await.unwrap();
        assert!(entry.is_none());
    }

    #[tokio::test]
    async fn projection_should_rebuild_from_events() {
        // Given
        let projection = ProductCatalogProjection;
        let store = InMemoryProjectionStore::new();
        
        let events = vec![
            StoredEvent {
                sequence: 1,
                aggregate_id: "prod-1".to_string(),
                event_type: "ProductCreated".to_string(),
                event_data: serde_json::to_value(ProductCreated {
                    product_id: "prod-1".to_string(),
                    name: "Widget A".to_string(),
                    price: 10.00,
                }).unwrap(),
                header: EventHeader::new(),
                cid: None,
                parent_cid: None,
                timestamp: chrono::Utc::now(),
            },
            StoredEvent {
                sequence: 2,
                aggregate_id: "prod-2".to_string(),
                event_type: "ProductCreated".to_string(),
                event_data: serde_json::to_value(ProductCreated {
                    product_id: "prod-2".to_string(),
                    name: "Widget B".to_string(),
                    price: 20.00,
                }).unwrap(),
                header: EventHeader::new(),
                cid: None,
                parent_cid: None,
                timestamp: chrono::Utc::now(),
            },
        ];

        // When
        projection.rebuild_from_events(events, &store).await.unwrap();

        // Then
        let all = store.list().await.unwrap();
        assert_eq!(all.len(), 2);
        assert!(all.iter().any(|e| e.product_id == "prod-1"));
        assert!(all.iter().any(|e| e.product_id == "prod-2"));
    }

    #[tokio::test]
    async fn projection_manager_should_handle_concurrent_updates() {
        // Given
        let store = InMemoryProjectionStore::new();
        let manager = ProjectionManager::new(vec![
            Box::new(ProductCatalogProjection),
        ]);

        // When - concurrent events
        let mut handles = vec![];
        
        for i in 0..10 {
            let manager_clone = manager.clone();
            let store_clone = store.clone();
            
            let handle = tokio::spawn(async move {
                let event = StoredEvent {
                    sequence: i,
                    aggregate_id: format!("prod-{}", i),
                    event_type: "ProductCreated".to_string(),
                    event_data: serde_json::to_value(ProductCreated {
                        product_id: format!("prod-{}", i),
                        name: format!("Widget {}", i),
                        price: 10.0 * i as f64,
                    }).unwrap(),
                    header: EventHeader::new(),
                    cid: None,
                    parent_cid: None,
                    timestamp: chrono::Utc::now(),
                };
                
                manager_clone.handle_event(&event, &store_clone).await
            });
            
            handles.push(handle);
        }

        // Wait for all
        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        // Then
        let all = store.list().await.unwrap();
        assert_eq!(all.len(), 10);
    }

    #[tokio::test]
    async fn projection_should_track_position() {
        // Given
        let projection = ProductCatalogProjection;
        let store = InMemoryProjectionStore::new();
        
        // When
        let position = store.get_position("product_catalog").await.unwrap();
        assert_eq!(position, 0);
        
        store.set_position("product_catalog", 42).await.unwrap();
        
        // Then
        let new_position = store.get_position("product_catalog").await.unwrap();
        assert_eq!(new_position, 42);
    }

    #[tokio::test]
    async fn projection_should_support_queries() {
        // Given
        let store = InMemoryProjectionStore::new();
        
        // Add test data
        for i in 1..=5 {
            store.save(&format!("prod-{}", i), ProductCatalogEntry {
                product_id: format!("prod-{}", i),
                name: format!("Widget {}", i),
                current_price: 10.0 * i as f64,
                price_history: vec![10.0 * i as f64],
                last_updated: chrono::Utc::now(),
            }).await.unwrap();
        }

        // When - query by price range
        let results: Vec<ProductCatalogEntry> = store
            .query(|entry: &ProductCatalogEntry| entry.current_price >= 30.0)
            .await
            .unwrap();

        // Then
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|e| e.current_price >= 30.0));
    }
}