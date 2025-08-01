use cim_events::{Event, EventStore, JetStreamEventStore, StoredEvent};
use cim_projections::{
    InMemoryProjectionStore, ProjectionHandler, ProjectionManager, ProjectionStore,
    ProjectionError,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

// Domain Events
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomerRegistered {
    customer_id: String,
    name: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderPlaced {
    order_id: String,
    customer_id: String,
    items: Vec<OrderItem>,
    total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderItem {
    product_id: String,
    quantity: u32,
    price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderShipped {
    order_id: String,
    shipped_at: chrono::DateTime<chrono::Utc>,
}

// Read Models
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomerSummary {
    customer_id: String,
    name: String,
    email: String,
    total_orders: u32,
    total_spent: f64,
    last_order_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderDetails {
    order_id: String,
    customer_id: String,
    customer_name: String,
    items: Vec<OrderItem>,
    total: f64,
    status: OrderStatus,
    created_at: chrono::DateTime<chrono::Utc>,
    shipped_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OrderStatus {
    Pending,
    Shipped,
}

// Customer Summary Projection
struct CustomerSummaryProjection {
    customer_names: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>,
}

impl CustomerSummaryProjection {
    fn new() -> Self {
        Self {
            customer_names: std::sync::Arc::new(tokio::sync::RwLock::new(
                std::collections::HashMap::new(),
            )),
        }
    }
}

#[async_trait::async_trait]
impl ProjectionHandler for CustomerSummaryProjection {
    type ReadModel = CustomerSummary;
    type Error = ProjectionError;

    fn projection_name(&self) -> &str {
        "customer_summary"
    }

    async fn handle_event(
        &self,
        event: &StoredEvent,
        store: &dyn ProjectionStore<Self::ReadModel>,
    ) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "CustomerRegistered" => {
                let data: CustomerRegistered = serde_json::from_value(event.event_data.clone())?;
                
                // Cache customer name
                self.customer_names
                    .write()
                    .await
                    .insert(data.customer_id.clone(), data.name.clone());
                
                let summary = CustomerSummary {
                    customer_id: data.customer_id.clone(),
                    name: data.name,
                    email: data.email,
                    total_orders: 0,
                    total_spent: 0.0,
                    last_order_date: None,
                };
                store.save(&data.customer_id, summary).await?;
            }
            "OrderPlaced" => {
                let data: OrderPlaced = serde_json::from_value(event.event_data.clone())?;
                if let Some(mut summary) = store.get(&data.customer_id).await? {
                    summary.total_orders += 1;
                    summary.total_spent += data.total;
                    summary.last_order_date = Some(event.timestamp);
                    store.save(&data.customer_id, summary).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// Order Details Projection
struct OrderDetailsProjection {
    customer_names: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>,
}

impl OrderDetailsProjection {
    fn new(
        customer_names: std::sync::Arc<
            tokio::sync::RwLock<std::collections::HashMap<String, String>>,
        >,
    ) -> Self {
        Self { customer_names }
    }
}

#[async_trait::async_trait]
impl ProjectionHandler for OrderDetailsProjection {
    type ReadModel = OrderDetails;
    type Error = ProjectionError;

    fn projection_name(&self) -> &str {
        "order_details"
    }

    async fn handle_event(
        &self,
        event: &StoredEvent,
        store: &dyn ProjectionStore<Self::ReadModel>,
    ) -> Result<(), Self::Error> {
        match event.event_type.as_str() {
            "OrderPlaced" => {
                let data: OrderPlaced = serde_json::from_value(event.event_data.clone())?;
                
                // Get customer name from cache
                let customer_name = self
                    .customer_names
                    .read()
                    .await
                    .get(&data.customer_id)
                    .cloned()
                    .unwrap_or_else(|| "Unknown".to_string());
                
                let details = OrderDetails {
                    order_id: data.order_id.clone(),
                    customer_id: data.customer_id,
                    customer_name,
                    items: data.items,
                    total: data.total,
                    status: OrderStatus::Pending,
                    created_at: event.timestamp,
                    shipped_at: None,
                };
                store.save(&data.order_id, details).await?;
            }
            "OrderShipped" => {
                let data: OrderShipped = serde_json::from_value(event.event_data.clone())?;
                if let Some(mut details) = store.get(&data.order_id).await? {
                    details.status = OrderStatus::Shipped;
                    details.shipped_at = Some(data.shipped_at);
                    store.save(&data.order_id, details).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// Implement Event trait for our events
impl Event for CustomerRegistered {
    fn event_type(&self) -> &str {
        "CustomerRegistered"
    }
    fn aggregate_id(&self) -> &str {
        &self.customer_id
    }
}

impl Event for OrderPlaced {
    fn event_type(&self) -> &str {
        "OrderPlaced"
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 CIM Projections Demo\n");

    // Connect to NATS and create event store
    println!("📡 Connecting to NATS...");
    let client = async_nats::connect("nats://localhost:4222").await?;
    let jetstream = async_nats::jetstream::new(client);
    let event_store = JetStreamEventStore::new(jetstream, "ecommerce-demo").await?;

    // Create projection stores
    let customer_store = InMemoryProjectionStore::<CustomerSummary>::new();
    let order_store = InMemoryProjectionStore::<OrderDetails>::new();

    // Create projections with shared state
    let customer_names = std::sync::Arc::new(tokio::sync::RwLock::new(
        std::collections::HashMap::new(),
    ));
    let customer_projection = CustomerSummaryProjection::new();
    let order_projection = OrderDetailsProjection::new(customer_names.clone());

    // Simulate event flow
    println!("\n📊 Simulating e-commerce event flow...\n");

    // 1. Register customers
    let customer1_id = Uuid::new_v4().to_string();
    let customer2_id = Uuid::new_v4().to_string();

    println!("1️⃣  Registering customers...");
    let event1 = CustomerRegistered {
        customer_id: customer1_id.clone(),
        name: "Alice Smith".to_string(),
        email: "alice@example.com".to_string(),
    };
    let stored1 = create_stored_event(event1.clone());
    customer_projection
        .handle_event(&stored1, &customer_store)
        .await?;

    let event2 = CustomerRegistered {
        customer_id: customer2_id.clone(),
        name: "Bob Jones".to_string(),
        email: "bob@example.com".to_string(),
    };
    let stored2 = create_stored_event(event2.clone());
    customer_projection
        .handle_event(&stored2, &customer_store)
        .await?;

    // 2. Place orders
    println!("\n2️⃣  Placing orders...");
    let order1_id = Uuid::new_v4().to_string();
    let order_event1 = OrderPlaced {
        order_id: order1_id.clone(),
        customer_id: customer1_id.clone(),
        items: vec![
            OrderItem {
                product_id: "PROD-1".to_string(),
                quantity: 2,
                price: 29.99,
            },
            OrderItem {
                product_id: "PROD-2".to_string(),
                quantity: 1,
                price: 49.99,
            },
        ],
        total: 109.97,
    };
    let stored_order1 = create_stored_event(order_event1.clone());
    customer_projection
        .handle_event(&stored_order1, &customer_store)
        .await?;
    order_projection
        .handle_event(&stored_order1, &order_store)
        .await?;

    let order2_id = Uuid::new_v4().to_string();
    let order_event2 = OrderPlaced {
        order_id: order2_id.clone(),
        customer_id: customer1_id.clone(),
        items: vec![OrderItem {
            product_id: "PROD-3".to_string(),
            quantity: 3,
            price: 19.99,
        }],
        total: 59.97,
    };
    let stored_order2 = create_stored_event(order_event2.clone());
    customer_projection
        .handle_event(&stored_order2, &customer_store)
        .await?;
    order_projection
        .handle_event(&stored_order2, &order_store)
        .await?;

    // 3. Ship an order
    println!("\n3️⃣  Shipping order...");
    let ship_event = OrderShipped {
        order_id: order1_id.clone(),
        shipped_at: chrono::Utc::now(),
    };
    let stored_ship = create_stored_event(ship_event.clone());
    order_projection
        .handle_event(&stored_ship, &order_store)
        .await?;

    // Query projections
    println!("\n📈 Querying projections...\n");

    // Customer summaries
    println!("Customer Summaries:");
    println!("==================");
    let customers = customer_store.list().await?;
    for customer in customers {
        println!("\n  Customer: {}", customer.name);
        println!("  Email: {}", customer.email);
        println!("  Total Orders: {}", customer.total_orders);
        println!("  Total Spent: ${:.2}", customer.total_spent);
        if let Some(last_order) = customer.last_order_date {
            println!("  Last Order: {}", last_order.format("%Y-%m-%d %H:%M:%S"));
        }
    }

    // Order details
    println!("\n\nOrder Details:");
    println!("==============");
    let orders = order_store.list().await?;
    for order in orders {
        println!("\n  Order ID: {}", order.order_id);
        println!("  Customer: {}", order.customer_name);
        println!("  Status: {:?}", order.status);
        println!("  Total: ${:.2}", order.total);
        println!("  Items:");
        for item in &order.items {
            println!(
                "    - {} x{} @ ${:.2}",
                item.product_id, item.quantity, item.price
            );
        }
        if let Some(shipped) = order.shipped_at {
            println!("  Shipped: {}", shipped.format("%Y-%m-%d %H:%M:%S"));
        }
    }

    // Query examples
    println!("\n\n🔍 Advanced Queries:");
    println!("===================");

    // High-value customers
    let vip_customers = customer_store
        .query(|c: &CustomerSummary| c.total_spent > 100.0)
        .await?;
    println!("\n  VIP Customers (spent > $100):");
    for customer in vip_customers {
        println!("    - {} (${:.2})", customer.name, customer.total_spent);
    }

    // Shipped orders
    let shipped_orders = order_store
        .query(|o: &OrderDetails| matches!(o.status, OrderStatus::Shipped))
        .await?;
    println!("\n  Shipped Orders:");
    for order in shipped_orders {
        println!("    - {} to {}", order.order_id, order.customer_name);
    }

    println!("\n✨ Demo completed successfully!");

    Ok(())
}

// Helper to create stored events
fn create_stored_event<E: Event + Serialize>(event: E) -> StoredEvent {
    use cim_events::EventHeader;
    
    StoredEvent {
        sequence: 0,
        aggregate_id: event.aggregate_id().to_string(),
        event_type: event.event_type().to_string(),
        event_data: serde_json::to_value(event).unwrap(),
        header: EventHeader::new(),
        cid: None,
        parent_cid: None,
        timestamp: chrono::Utc::now(),
    }
}