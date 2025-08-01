use cim_events::{Event, EventHeader, EventStore, JetStreamEventStore};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

// Domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderCreated {
    order_id: String,
    customer_id: String,
    total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderShipped {
    order_id: String,
    tracking_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderDelivered {
    order_id: String,
    delivered_at: chrono::DateTime<chrono::Utc>,
}

// Implement Event trait
impl Event for OrderCreated {
    fn event_type(&self) -> &str {
        "OrderCreated"
    }
    
    fn aggregate_id(&self) -> &str {
        &self.order_id
    }
}

impl Event for OrderShipped {
    fn event_type(&self) -> &str {
        "OrderShipped"
    }
    
    fn aggregate_id(&self) -> &str {
        &self.order_id
    }
}

impl Event for OrderDelivered {
    fn event_type(&self) -> &str {
        "OrderDelivered"
    }
    
    fn aggregate_id(&self) -> &str {
        &self.order_id
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    println!("🚀 CIM Event Store Demo\n");
    
    // Connect to NATS
    println!("📡 Connecting to NATS...");
    let client = async_nats::connect("nats://localhost:4222").await?;
    let jetstream = async_nats::jetstream::new(client);
    
    // Create event store
    println!("📦 Creating event store...");
    let store = JetStreamEventStore::new(jetstream, "orders-demo").await?;
    
    // Create an order saga
    let order_id = format!("order-{}", Uuid::new_v4());
    let correlation_id = Uuid::new_v4().to_string();
    
    println!("\n🛍️  Starting order saga for: {}", order_id);
    println!("🔗 Correlation ID: {}", correlation_id);
    
    // Event 1: Order Created
    println!("\n1️⃣  Creating order...");
    let order_created = OrderCreated {
        order_id: order_id.clone(),
        customer_id: "customer-123".to_string(),
        total: 99.99,
    };
    
    let header1 = EventHeader::with_correlation(correlation_id.clone());
    let metadata1 = store
        .append_event_with_header(&order_id, order_created, header1.clone(), None)
        .await?;
    
    println!("   ✅ Order created with CID: {:?}", metadata1.cid);
    println!("   📍 Sequence: {}", metadata1.sequence);
    
    // Event 2: Order Shipped (caused by order creation)
    println!("\n2️⃣  Shipping order...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    let order_shipped = OrderShipped {
        order_id: order_id.clone(),
        tracking_number: "TRACK-12345".to_string(),
    };
    
    let header2 = EventHeader::with_causation(
        correlation_id.clone(),
        header1.message_id.clone(),
    );
    
    let metadata2 = store
        .append_event_with_header(
            &order_id,
            order_shipped,
            header2.clone(),
            metadata1.cid, // Link to previous event
        )
        .await?;
    
    println!("   ✅ Order shipped with CID: {:?}", metadata2.cid);
    println!("   📍 Sequence: {}", metadata2.sequence);
    println!("   ⛓️  Parent CID: {:?}", metadata1.cid);
    
    // Event 3: Order Delivered
    println!("\n3️⃣  Delivering order...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    let order_delivered = OrderDelivered {
        order_id: order_id.clone(),
        delivered_at: chrono::Utc::now(),
    };
    
    let header3 = EventHeader::with_causation(
        correlation_id.clone(),
        header2.message_id.clone(),
    );
    
    let metadata3 = store
        .append_event_with_header(
            &order_id,
            order_delivered,
            header3,
            metadata2.cid, // Link to previous event
        )
        .await?;
    
    println!("   ✅ Order delivered with CID: {:?}", metadata3.cid);
    println!("   📍 Sequence: {}", metadata3.sequence);
    
    // Validate CID chain
    println!("\n🔍 Validating CID chain integrity...");
    let is_valid = store.validate_cid_chain(&order_id).await?;
    println!("   Chain integrity: {}", if is_valid { "✅ VALID" } else { "❌ INVALID" });
    
    // Replay events
    println!("\n📼 Replaying order events:");
    let events = store.get_events(&order_id, 0, 10).await?;
    
    for event in &events {
        println!("\n   Event #{}", event.sequence);
        println!("   Type: {}", event.event_type);
        println!("   CID: {:?}", event.cid);
        println!("   Parent: {:?}", event.parent_cid);
        println!("   Correlation: {}", event.header.correlation_id);
        println!("   Causation: {:?}", event.header.causation_id);
        println!("   Data: {}", serde_json::to_string_pretty(&event.event_data)?);
    }
    
    // Demonstrate real-time subscription
    println!("\n📡 Setting up real-time subscription...");
    
    // Create a new order to watch
    let watch_order_id = format!("order-{}", Uuid::new_v4());
    
    // Subscribe before creating events
    let mut subscription = store.subscribe_to_events(&watch_order_id).await?;
    
    // Spawn a task to create events
    let store_clone = store.clone();
    let watch_order_clone = watch_order_id.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        let event = OrderCreated {
            order_id: watch_order_clone.clone(),
            customer_id: "customer-456".to_string(),
            total: 199.99,
        };
        
        store_clone.append_event(&watch_order_clone, event, None).await.unwrap();
    });
    
    // Wait for the event
    println!("   ⏳ Waiting for real-time events...");
    
    use futures::StreamExt;
    if let Some(event) = subscription.next().await {
        println!("   🎉 Received real-time event!");
        println!("      Type: {}", event.event_type);
        println!("      Sequence: {}", event.sequence);
    }
    
    println!("\n✨ Demo completed successfully!");
    
    Ok(())
}