# cim-projections

CQRS projection implementation for building read models from event streams in the Composable Information Machine (CIM).

## Features

- **Multiple Projection Handlers**: Transform events into specialized read models
- **Pluggable Storage Backends**: In-memory, PostgreSQL, Redis support
- **Position Tracking**: Resume projections from last processed event
- **Concurrent Processing**: Handle multiple projections efficiently
- **Query Support**: Rich querying capabilities for read models
- **Error Recovery**: Graceful handling of projection failures

## Architecture

```mermaid
graph LR
    A[Event Stream] --> B[Projection Manager]
    B --> C[Handler 1]
    B --> D[Handler 2]
    B --> E[Handler N]
    C --> F[Read Model Store]
    D --> F
    E --> F
    F --> G[Queries]
```

## Usage

### Basic Projection

```rust
use cim_projections::{ProjectionHandler, ProjectionStore, InMemoryProjectionStore};
use cim_events::StoredEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct UserProfile {
    user_id: String,
    name: String,
    email: String,
    total_orders: u32,
}

struct UserProfileProjection;

#[async_trait::async_trait]
impl ProjectionHandler for UserProfileProjection {
    type ReadModel = UserProfile;
    type Error = ProjectionError;
    
    fn projection_name(&self) -> &str {
        "user_profile"
    }
    
    async fn handle_event(
        &self,
        event: &StoredEvent,
        store: &dyn ProjectionStore<Self::ReadModel>,
    ) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "UserRegistered" => {
                let data: UserRegisteredEvent = serde_json::from_value(event.event_data.clone())?;
                let profile = UserProfile {
                    user_id: data.user_id,
                    name: data.name,
                    email: data.email,
                    total_orders: 0,
                };
                store.save(&profile.user_id, profile).await?;
            }
            "OrderPlaced" => {
                let data: OrderPlacedEvent = serde_json::from_value(event.event_data.clone())?;
                if let Some(mut profile) = store.get(&data.user_id).await? {
                    profile.total_orders += 1;
                    store.save(&data.user_id, profile).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
```

### Managing Multiple Projections

```rust
use cim_projections::{ProjectionManager, ProjectionRunner};

// Create projection manager with multiple handlers
let manager = ProjectionManager::new(vec![
    Box::new(UserProfileProjection),
    Box::new(OrderSummaryProjection),
    Box::new(InventoryProjection),
]);

// Process an event across all projections
let event = StoredEvent { /* ... */ };
manager.handle_event(&event, &store).await?;
```

### Query Read Models

```rust
// Get a specific user profile
let profile = store.get("user-123").await?;

// Query with predicates
let vip_users = store.query(|profile: &UserProfile| {
    profile.total_orders > 100
}).await?;

// List all profiles
let all_profiles = store.list().await?;
```

### Position Tracking

```rust
// Get current position
let position = store.get_position("user_profile").await?;

// Resume from last position
let events = event_store.get_events("users", position + 1, 100).await?;
for event in events {
    projection.process_event(&event, &store).await?;
}
```

### PostgreSQL Backend

```rust
#[cfg(feature = "postgres")]
use cim_projections::PostgresProjectionStore;

let store = PostgresProjectionStore::<UserProfile>::new(
    "postgres://user:pass@localhost/db",
    "user_profiles"
).await?;
```

## Projection Patterns

### 1. Summary Projections
Aggregate data across multiple events:

```rust
struct OrderSummary {
    total_orders: u64,
    total_revenue: f64,
    average_order_value: f64,
}
```

### 2. Denormalized Views
Join data from multiple aggregates:

```rust
struct OrderWithCustomer {
    order_id: String,
    customer_name: String,
    customer_email: String,
    items: Vec<OrderItem>,
}
```

### 3. Search Indexes
Build searchable projections:

```rust
struct ProductSearchEntry {
    product_id: String,
    name: String,
    description: String,
    tags: Vec<String>,
    search_text: String, // Concatenated searchable fields
}
```

## Testing

```bash
# Run all tests
cargo test

# Run with PostgreSQL tests
cargo test --features postgres

# Run specific projection tests
cargo test projection_should_handle
```

## Performance Considerations

- **Batch Processing**: Process events in batches for better throughput
- **Async Projections**: Non-critical projections can run asynchronously
- **Indexing**: Add database indexes for frequently queried fields
- **Caching**: Cache frequently accessed read models

## Best Practices

1. **Idempotency**: Projections should be idempotent - processing the same event twice should not corrupt the read model
2. **Error Handling**: Failed projections should not stop other projections
3. **Eventual Consistency**: Accept that read models lag behind the write model
4. **Rebuild Capability**: Always be able to rebuild projections from events

## License

Apache-2.0