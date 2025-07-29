# Event Storming Guide for CIM

Event Storming is the **mandatory first step** in creating any CIM domain. It's a workshop technique that helps you discover the true nature of your business domain by focusing on what happens (events) rather than data structures.

## Why Event Storming First?

Without Event Storming, you risk:
- Building the wrong system
- Missing critical business rules
- Creating domains that don't reflect reality
- Wasting time on features nobody needs

With Event Storming, you gain:
- Deep understanding of the business domain
- Shared language between technical and business people
- Discovery of hidden complexity and rules
- Clear boundaries between domains

## Tools for Visual Design

CIM uses visual graphs as the primary design artifact. AI agents then generate code from these graphs.

### Primary Tools

#### Traditional Event Storming with Physical Sticky Notes (Alberto Brandolini Method)

Alberto Brandolini created Event Storming as a workshop-based approach to quickly explore complex business domains. The method uses physical sticky notes on a large wall to discover domain events through collaborative exploration.

##### The Brandolini Method

**Core Principles:**
1. **Big Picture First** - Start with a high-level view of the entire business process
2. **Timeline-Based** - Events are arranged chronologically from left to right
3. **No Chairs** - Standing keeps energy high and people engaged
4. **Unlimited Modeling Space** - Use an entire wall or multiple walls
5. **Domain Experts Drive** - Business people lead, developers learn

**Materials Needed:**
- **Orange sticky notes** - Domain Events (past tense)
- **Blue sticky notes** - Commands (imperative mood)
- **Yellow sticky notes** - Aggregates/Entities
- **Pink sticky notes** - Hot spots/Problems/Questions
- **Purple sticky notes** - Policies/Business Rules
- **Green sticky notes** - External Systems
- **Small yellow stickies** - Actors/Users
- **Red tape or string** - Boundaries between contexts
- **Markers** - Thick black markers for readability
- **Unlimited wall space** - Minimum 6-8 meters of wall

##### The Event Storming Process

**Step 1: Chaotic Exploration (45-60 minutes)**
```
Instructions:
1. "Write down business events that happen in this domain"
2. "Use past tense: 'Order Placed', 'Payment Received'"
3. "One event per orange sticky note"
4. "Don't worry about order or duplicates yet"
5. "If you're not sure, write it anyway"

Result: Wall covered in orange sticky notes
```

**Step 2: Enforce Timeline (20-30 minutes)**
```
Instructions:
1. "Arrange events from left to right in time order"
2. "Parallel events stack vertically"
3. "Remove obvious duplicates"
4. "Mark unclear areas with pink stickies"

Result: Rough timeline of domain events
```

**Step 3: Find Pivotal Events (15-20 minutes)**
```
Instructions:
1. "Identify events that change the business flow"
2. "Mark events that trigger many other events"
3. "Look for events that represent phase transitions"
4. "These often indicate aggregate boundaries"

Result: Key business transitions identified
```

**Step 4: Commands and Actors (30-40 minutes)**
```
Instructions:
1. "What command causes each event?"
2. "Write commands on blue stickies (imperative: 'Place Order')"
3. "Who issues this command? Add yellow actor stickies"
4. "Place commands before their events"

Result: Commands and actors connected to events
```

**Step 5: Aggregates and Boundaries (30-40 minutes)**
```
Instructions:
1. "Group related events around yellow aggregate stickies"
2. "Name the thing that changes state (Order, Inventory, Customer)"
3. "Draw boundaries with red tape between different aggregates"
4. "These boundaries often become your bounded contexts"

Result: Domain model emerges from events
```

**Step 6: Policies and Read Models (20-30 minutes)**
```
Instructions:
1. "When this event happens, what policies trigger?"
2. "Write policies on purple stickies between events"
3. "Identify what information is needed for decisions"
4. "Mark external systems with green stickies"

Result: Complete behavioral model of the domain
```

##### Converting Physical Event Storm to CIM

**Photographing the Wall:**
```bash
# Best practices for capturing:
1. Take one high-resolution overview photo
2. Take detailed photos of each section (3-4 meter segments)
3. Ensure all text is readable
4. Use consistent lighting
5. Keep camera perpendicular to wall
```

**Import to Digital Format:**
```bash
# Option 1: Direct to ExcaliDraw
1. Upload photos as background
2. Recreate sticky notes as digital elements
3. Preserve spatial relationships
4. Add digital-only metadata

# Option 2: AI-Assisted Extraction
cim ai extract-event-storm \
  --photos ./event-storm-photos/*.jpg \
  --output ./design/event-storm.excalidraw

# AI will:
# - OCR all sticky notes
# - Identify types by color
# - Preserve spatial relationships
# - Generate ExcaliDraw file
```

##### Why Start with Physical Event Storming?

**Advantages:**
1. **No Technical Barriers** - Anyone can write on a sticky note
2. **Tactile Engagement** - Physical interaction improves memory
3. **Spatial Memory** - People remember where things were placed
4. **Equal Participation** - No one person controls the mouse
5. **Energy and Movement** - Standing and moving keeps energy high
6. **Instant Visualization** - No UI to learn

**When to Go Digital:**
1. After initial discovery is complete
2. For remote team members to review
3. To version control the design
4. To generate code from the model
5. For ongoing refinement and evolution

##### Common Patterns in Event Storming

**Pattern 1: Command-Event Pairs**
```
[Place Order] → (Order Placed)
[Pay Invoice] → (Invoice Paid)
[Ship Package] → (Package Shipped)
```

**Pattern 2: Event Cascades**
```
(Order Placed) → [Reserve Inventory] → (Inventory Reserved)
                → [Calculate Tax] → (Tax Calculated)
                → [Send Confirmation] → (Confirmation Sent)
```

**Pattern 3: Policy Triggers**
```
(Inventory Low) → Policy: "When inventory < 10" → [Reorder Stock]
```

**Pattern 4: Aggregate Boundaries**
```
ORDER AGGREGATE          INVENTORY AGGREGATE
(Order Placed)          (Stock Reserved)
(Order Paid)            (Stock Depleted)
(Order Shipped)         (Stock Received)
```

##### Tips for Successful Event Storming

**Do:**
- Invite actual domain experts who do the work daily
- Keep sessions under 4 hours (split large domains)
- Have plenty of supplies (hundreds of stickies)
- Take breaks every hour
- Document hot spots and questions
- Let the domain experts drive

**Don't:**
- Start designing solutions
- Discuss implementation details
- Let developers dominate
- Worry about perfection
- Skip the chaotic exploration phase
- Sit down

**Export**: Photo (preserves all data), PNG (for documentation), Extract to CIM Graph using AI

#### ExcaliDraw (Recommended)
- **Why**: Open source, local-first, excellent export formats
- **Use for**: Event Storming, Context Maps, Concept Diagrams
- **Export**: JSON (preserves all data), SVG (for documentation)
- **Library**: https://libraries.excalidraw.com - CIM component library

#### Arrows.app (For Graph Databases)
- **Why**: Designed for property graphs, exports Cypher
- **Use for**: Domain relationships, Knowledge graphs
- **Export**: Cypher queries, JSON, SVG
- **Integration**: Direct to Neo4j or CIM Knowledge Graph

### Tool Setup

```bash
# Unlimited roll of Paper
1. Use multi-colored notes with large, hand written text
2. Arrange as required
3. Take a picture
4. Import into CIM - Alchemist

# ExcaliDraw with CIM Libraries
1. Visit https://excalidraw.com
2. Menu → Libraries → Browse Libraries
3. Search "CIM Event Storm"
4. Add: Events, Commands, Aggregates, Policies

# Arrows.app for Relationships
1. Visit https://arrows.app
2. Create New Diagram
3. Use CIM naming conventions:
   - Nodes: PascalCase (Order, Customer)
   - Relationships: UPPER_SNAKE (PLACED_BY, CONTAINS)
   - Properties: camelCase (orderId, totalAmount)
```

### Why Visual First?

1. **Graphs are executable specifications** - AI agents read these directly
2. **No ambiguity** - Visual relationships are explicit
3. **Collaborative** - Business and tech speak the same language
4. **Version controlled** - JSON exports go in git

## Event Storming with ExcaliDraw

### Phase 1: Chaotic Exploration (45-60 minutes)

#### Setting Up ExcaliDraw

```json
// ExcaliDraw Event Storm Template
{
  "elements": {
    "event": {
      "type": "rectangle",
      "backgroundColor": "#ff9f43",  // Orange for events
      "strokeWidth": 2,
      "roughness": 1,
      "fontSize": 16,
      "fontFamily": "Cascadia",
      "textAlign": "center",
      "verticalAlign": "middle"
    }
  }
}
```

#### Creating Events in ExcaliDraw

1. **Create Event Cards**:
   - Rectangle tool (R)
   - Orange background (#ff9f43)
   - Past tense text: "Order Placed", "Payment Received"
   - Group (Ctrl+G) text with rectangle

2. **Organize Spatially**:
   - Left to right = timeline
   - Top to bottom = different flows
   - Clusters = related events

3. **Export for AI Processing**:
   ```bash
   # Export as JSON to preserve all metadata
   ExcaliDraw → Menu → Export → JSON
   
   # File: event-storm-2024-01-29.excalidraw
   git add event-storm-2024-01-29.excalidraw
   git commit -m "Event storm session: Order fulfillment domain"
   ```

**Example ExcaliDraw Event Storm**:
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    Customer     │    │     Order       │    │    Payment      │
│   Registered    │───▶│    Placed       │───▶│   Processed     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │                        │
                              ▼                        ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │   Inventory     │    │    Payment      │
                       │    Checked      │    │     Failed      │
                       └─────────────────┘    └─────────────────┘
```

### Phase 2: Enforce Timeline (30-45 minutes)

Arrange events in chronological order.

```
[Customer Registered] → [Order Placed] → [Payment Processed] → [Order Confirmed] → [Items Picked] → [Order Shipped]
                              ↓
                    [Payment Failed] → [Order Cancelled]
```

**Key Questions:**
- What happens first?
- What triggers what?
- Are there parallel flows?
- Where are the decision points?

### Phase 3: Find Commands (30 minutes)

Add blue sticky notes for commands that cause events.

```mermaid
graph LR
    subgraph "Command-Event Pairs"
        C1[Register Customer] -->|produces| E1[Customer Registered]
        C2[Place Order] -->|produces| E2[Order Placed]
        C3[Process Payment] -->|produces| E3[Payment Processed]
        C3 -->|or produces| E4[Payment Failed]
    end
    
    %% Styling
    style C1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style E1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style E2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style E3 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style E4 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

**Commands represent:**
- User intentions
- System triggers
- Time-based actions
- External system calls

### Phase 4: Identify Aggregates (30 minutes)

Group commands and events around yellow aggregates.

```yaml
Customer Aggregate:
  commands:
    - Register Customer
    - Update Profile
    - Change Password
  events:
    - Customer Registered
    - Profile Updated
    - Password Changed
  state:
    - Customer ID
    - Email
    - Profile Info
    - Preferences

Order Aggregate:
  commands:
    - Place Order
    - Cancel Order
    - Update Shipping Address
  events:
    - Order Placed
    - Order Cancelled
    - Shipping Address Updated
  state:
    - Order ID
    - Customer ID
    - Line Items
    - Status
    - Total Amount
```

### Phase 5: Discover Policies (20 minutes)

Add pink sticky notes for automated policies.

```mermaid
graph LR
    E1[Order Placed] --> P1{Large Order Policy}
    P1 -->|if > $1000| C1[Request Manager Approval]
    
    E2[Stock Level Low] --> P2{Reorder Policy}
    P2 -->|if < reorder point| C2[Create Purchase Order]
    
    E3[Payment Failed] --> P3{Retry Policy}
    P3 -->|retry 3 times| C3[Retry Payment]
    P3 -->|after 3 failures| C4[Cancel Order]
    
    %% Styling
    style E1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style E2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style E3 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style P1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style P2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style P3 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
```

**Policies are:**
- Business rules that trigger automatically
- "When X happens, then Y should happen"
- Often time-based or threshold-based
- The "smart" part of your system

### Phase 6: Define Bounded Contexts (20 minutes)

Draw boundaries around related aggregates.

```mermaid
graph TB
    subgraph "Sales Context"
        A1[Customer]
        A2[Order]
        A3[Pricing]
    end
    
    subgraph "Inventory Context"
        B1[Product]
        B2[Stock Level]
        B3[Warehouse]
    end
    
    subgraph "Shipping Context"
        C1[Shipment]
        C2[Carrier]
        C3[Tracking]
    end
    
    subgraph "Accounting Context"
        D1[Invoice]
        D2[Payment]
        D3[Ledger]
    end
    
    A2 -.->|checks availability| B2
    A2 -.->|creates| C1
    A2 -.->|generates| D1
    
    %% Styling
    style A1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style A2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style B1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style B2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style C1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style D1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

## Documenting Results

### Event Catalog

Create a structured catalog of discovered events:

```yaml
# events-catalog.yaml
domain: E-Commerce
contexts:
  - name: Sales
    events:
      - name: Customer Registered
        triggers:
          - command: Register Customer
          - source: Registration Form
        data:
          - customer_id: UUID
          - email: Email
          - name: String
          - registered_at: Timestamp
        produces:
          - Welcome Email Sent
          - Customer Profile Created
          
      - name: Order Placed
        triggers:
          - command: Place Order
          - source: Shopping Cart
        data:
          - order_id: UUID
          - customer_id: UUID
          - line_items: Array<LineItem>
          - total_amount: Money
          - placed_at: Timestamp
        policies:
          - Large Order Approval (if > $1000)
          - Inventory Reservation
          - Fraud Check
```

### State Machine Discovery

From events, derive state machines:

```rust
// Discovered Order States
pub enum OrderState {
    // Initial state
    Draft {
        items: Vec<LineItem>,
        customer_id: CustomerId,
    },
    
    // After "Order Placed" event
    Placed {
        order_id: OrderId,
        items: Vec<LineItem>,
        total: Money,
    },
    
    // After "Payment Processed" event
    Paid {
        order_id: OrderId,
        payment_id: PaymentId,
    },
    
    // After "Items Shipped" event
    Shipped {
        order_id: OrderId,
        tracking_number: String,
    },
    
    // Terminal states
    Delivered {
        order_id: OrderId,
        delivered_at: Timestamp,
    },
    
    Cancelled {
        order_id: OrderId,
        reason: String,
    },
}
```

### Policy Documentation

```yaml
# policies.yaml
policies:
  - name: Automatic Reorder Policy
    trigger: Stock Level Low
    conditions:
      - current_stock < reorder_point
      - supplier_available = true
      - pending_orders = 0
    actions:
      - Create Purchase Order
      - Notify Purchasing Manager
    implementation: |
      When Stock Level Low event received:
        1. Check current stock against reorder point
        2. Verify no pending orders exist
        3. Check supplier availability
        4. Calculate order quantity (reorder_quantity)
        5. Create and submit purchase order
        6. Update pending orders
```

## Common Patterns

### 1. Saga Pattern
Long-running business processes:

```
[Order Placed] → [Inventory Reserved] → [Payment Processed] → [Items Picked] → [Order Shipped]
       ↓                ↓                      ↓                    ↓
[Insufficient]   [Payment Failed]     [Picking Failed]    [Shipping Failed]
       ↓                ↓                      ↓                    ↓
[Order Failed]   [Inventory Released]  [Order On Hold]    [Retry Shipping]
```

### 2. Event Sourcing Pattern
State changes through events:

```
Initial State: {}
+ Customer Registered → {customer_id, email, name}
+ Address Added → {customer_id, email, name, address}
+ Order Placed → {customer_id, email, name, address, orders: [order1]}
```

### 3. CQRS Pattern
Commands separate from queries:

```yaml
Commands (Write Side):
  - Register Customer
  - Place Order
  - Update Inventory

Events:
  - Customer Registered
  - Order Placed
  - Inventory Updated

Read Models (Query Side):
  - Customer Profile View
  - Order History View
  - Available Inventory View
```

## Tips for Successful Event Storming

### Do's:
- ✅ Include real domain experts
- ✅ Use business language, not technical terms
- ✅ Focus on what happens, not how
- ✅ Embrace chaos in the beginning
- ✅ Ask "what happens next?"
- ✅ Look for temporal triggers ("every Monday", "end of month")
- ✅ Identify external system interactions

### Don'ts:
- ❌ Design the database
- ❌ Discuss implementation details
- ❌ Skip events because they seem "obvious"
- ❌ Let one person dominate
- ❌ Stop at happy paths only
- ❌ Ignore edge cases
- ❌ Rush the process

## From Event Storm to CIM Domain

### 1. Create Domain Structure

```bash
# Based on your event storm
cargo new --lib my-sales-domain
cd my-sales-domain

# Create structure matching discovered contexts
mkdir -p src/{aggregates,commands,events,policies,state_machines}
```

### 2. Implement Events First

```rust
// src/events.rs
// Direct translation from orange sticky notes
#[derive(Debug, Clone, Event)]
pub enum SalesEvent {
    CustomerRegistered {
        customer_id: CustomerId,
        email: String,
        name: String,
    },
    OrderPlaced {
        order_id: OrderId,
        customer_id: CustomerId,
        items: Vec<LineItem>,
        total: Money,
    },
    PaymentProcessed {
        order_id: OrderId,
        payment_id: PaymentId,
        amount: Money,
    },
}
```

### 3. Create State Machines

```rust
// src/state_machines/order.rs
// Based on discovered state transitions
impl StateMachine for OrderState {
    type Event = OrderEvent;
    
    fn apply_event(&mut self, event: Self::Event) {
        match (self, event) {
            (Self::Draft { .. }, OrderEvent::Placed { order_id, .. }) => {
                *self = Self::Placed { order_id, .. };
            }
            // More transitions...
        }
    }
}
```

## Continuous Event Storming

Event Storming isn't a one-time activity:

1. **Regular Reviews**: Every sprint/month
2. **New Feature Discovery**: Before adding features
3. **Problem Investigation**: When issues arise
4. **Domain Evolution**: As business changes

## Tools and Resources

### Digital Tools
- [Miro Event Storming Template](https://miro.com/templates/event-storming/)
- [EventStorming.com](https://www.eventstorming.com/)
- [Domain Storytelling](https://domainstorytelling.org/)

### Books
- "Event Storming" by Alberto Brandolini
- "Domain-Driven Design" by Eric Evans
- "Implementing Domain-Driven Design" by Vaughn Vernon

### Next Steps
- [Context Mapping Guide](./context-mapping-guide.md)
- [State Machine Patterns](./state-machine-patterns.md)
- [Domain Development Guide](./domain-development-guide.md)

---

**Remember**: Event Storming reveals the true complexity of your domain. If it seems simple, you haven't dug deep enough. The messier the wall, the better your understanding.