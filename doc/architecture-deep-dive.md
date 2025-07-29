# CIM Architecture Deep Dive

Understanding CIM's visual-first, state machine-driven architecture.

## Table of Contents

1. [Architectural Philosophy](#architectural-philosophy)
2. [Visual Architecture](#visual-architecture)
3. [State Machine Foundation](#state-machine-foundation)
4. [Event-Driven Core](#event-driven-core)
5. [Graph-Based Design](#graph-based-design)
6. [Distributed State Machines](#distributed-state-machines)
7. [AI Integration Layer](#ai-integration-layer)
8. [Security Through States](#security-through-states)
9. [Performance Patterns](#performance-patterns)

## Architectural Philosophy

CIM's architecture fundamentally differs from traditional systems:

### 1. Graphs as Executable Specifications

In CIM, visual graphs ARE the system design:

```
Traditional: Requirements → Code → System
CIM:         Visual Graphs → AI Generation → Execution
```

The graphs you draw in ExcaliDraw and Arrows.app directly execute:
- State machines define behavior
- Context maps define boundaries  
- Workflow graphs define processes
- Concept graphs define relationships

### 2. Everything is a State Machine

CIM doesn't have objects, services, or controllers. It has state machines:

```
Traditional Architecture        CIM Architecture
────────────────────────       ─────────────────────
- Services                     - Domain State Machines
- Controllers                  - Workflow State Machines  
- Models                       - Aggregate State Machines
- Repositories                 - Projection State Machines
- Middleware                   - Policy State Machines
```

### 3. AI-Native Design

AI isn't added to CIM - it's built into the foundation:
- AI reads your visual designs
- AI generates state machine implementations
- AI orchestrates cross-domain workflows
- AI maintains consistency across the mesh

## Visual Architecture

CIM's architecture starts with visual design:

```mermaid
graph TB
    subgraph "Design Layer (Human)"
        D1[Event Storming]
        D2[Context Mapping]
        D3[State Machine Design]
        D4[Workflow Graphs]
    end
    
    subgraph "AI Generation Layer"
        AI1[Graph Parser]
        AI2[Pattern Recognition]
        AI3[Code Generator]
        AI4[Consistency Checker]
    end
    
    subgraph "State Machine Layer"
        SM1[Domain State Machines]
        SM2[Workflow State Machines]
        SM3[Policy State Machines]
        SM4[Projection State Machines]
    end
    
    subgraph "Event Mesh Layer"
        E1[Event Streams]
        E2[Event Store]
        E3[Event Correlation]
        E4[Event Replay]
    end
    
    subgraph "Execution Layer"
        X1[NATS Messaging]
        X2[State Persistence]
        X3[Edge Nodes]
        X4[Knowledge Graph]
    end
    
    D1 --> AI1
    D2 --> AI1
    D3 --> AI1
    D4 --> AI1
    
    AI1 --> AI2
    AI2 --> AI3
    AI3 --> SM1
    AI3 --> SM2
    AI3 --> SM3
    AI3 --> SM4
    
    SM1 --> E1
    SM2 --> E1
    SM3 --> E1
    SM4 --> E2
    
    E1 --> X1
    E2 --> X2
    E3 --> X3
    E4 --> X4
    
    %% Styling
    style D1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style D2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style D3 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style D4 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style AI1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style AI2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style AI3 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style AI4 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style SM1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style SM2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style SM3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style SM4 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
```

### Architecture Flow

1. **Visual Design** - Humans create understanding through graphs
2. **AI Generation** - AI reads graphs and generates state machines
3. **State Execution** - State machines process events
4. **Event Flow** - Events flow through the mesh
5. **Distribution** - Execution happens across nodes

## State Machine Foundation

### Everything is a State Machine

In CIM, all behavior is modeled as state machines:

```rust
// Not objects with methods, but states with transitions
pub trait StateMachine {
    type State;
    type Event;
    type Command;
    
    fn current_state(&self) -> &Self::State;
    fn handle_command(&self, cmd: Self::Command) -> Vec<Self::Event>;
    fn apply_event(&mut self, event: Self::Event);
}
```

### State Machine Types

```mermaid
graph TB
    subgraph "Domain State Machines"
        DS1[Order State Machine]
        DS2[Inventory State Machine]
        DS3[Customer State Machine]
    end
    
    subgraph "Workflow State Machines"
        WS1[Order Fulfillment]
        WS2[Return Process]
        WS3[Reorder Workflow]
    end
    
    subgraph "Policy State Machines"
        PS1[Fraud Detection]
        PS2[Auto Reorder]
        PS3[Discount Rules]
    end
    
    subgraph "Saga State Machines"
        SS1[Payment Saga]
        SS2[Shipping Saga]
        SS3[Refund Saga]
    end
    
    DS1 --> WS1
    DS2 --> WS1
    WS1 --> SS1
    PS2 --> DS2
    
    %% Styling
    style DS1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style DS2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style DS3 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style WS1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style WS2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style WS3 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style PS1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style PS2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style PS3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
```

### State Completeness Principle

Each state contains ALL data needed for that state:

```rust
// ❌ Wrong: Incomplete states
enum BadOrderState {
    Placed,      // Where's the order data?
    Paid,        // How much was paid?
    Shipped,     // What's the tracking info?
}

// ✅ Right: Complete states
enum GoodOrderState {
    Placed {
        order_id: OrderId,
        customer: CustomerId,
        items: Vec<LineItem>,
        total: Money,
        placed_at: DateTime,
    },
    Paid {
        order_id: OrderId,
        payment_id: PaymentId,
        amount: Money,
        paid_at: DateTime,
    },
    Shipped {
        order_id: OrderId,
        tracking: TrackingNumber,
        carrier: Carrier,
        shipped_at: DateTime,
    },
}
```

## Graph-Based Domain Design

### From Visual Design to Domain Code

In CIM, domains start as visual graphs that AI transforms into state machines:

```mermaid
graph LR
    subgraph "Design Phase"
        E1[Event Storm]
        E2[Context Map]
        E3[State Diagrams]
        E4[Concept Graphs]
    end
    
    subgraph "AI Generation"
        A1[Parse Graphs]
        A2[Generate State Machines]
        A3[Create Event Types]
        A4[Build Domain Module]
    end
    
    subgraph "Runtime"
        R1[State Machine Execution]
        R2[Event Processing]
        R3[Domain Logic]
    end
    
    E1 --> A1
    E2 --> A1
    E3 --> A1
    E4 --> A1
    
    A1 --> A2
    A2 --> A3
    A3 --> A4
    
    A4 --> R1
    R1 --> R2
    R2 --> R3
    
    %% Styling
    style E1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style E2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style A1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style R1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
```

### Visual Domain Boundaries

Domains are defined visually in ExcaliDraw:

```
┌─────────────────────────────────────┐
│         ORDER DOMAIN                │
│  Everything is a State Machine      │
│                                     │
│  Order State Machine                │
│  ├─ Draft → Placed → Paid          │
│  └─ Cancelled (from any state)     │
│                                     │
│  Order Saga State Machine           │
│  ├─ Started → Processing            │
│  └─ Completed / Failed              │
└─────────────────────────────────────┘
        │ Events │
        ▼        ▼
┌─────────────────────────────────────┐
│      INVENTORY DOMAIN               │
│  Stock State Machine                │
│  ├─ Available → Reserved            │
│  └─ Depleted → Restocking          │
└─────────────────────────────────────┘
```

### State Machine Aggregates

Aggregates in CIM are state machines with complete state data:

```rust
// AI generates this from your visual state diagram
#[derive(StateMachine)]
pub enum OrderAggregate {
    Draft {
        id: OrderId,
        customer: CustomerId,
        items: Vec<LineItem>,
        created_at: DateTime,
    },
    
    Placed {
        id: OrderId,
        customer: CustomerId,
        items: Vec<LineItem>,
        total: Money,
        placed_at: DateTime,
    },
    
    Paid {
        id: OrderId,
        payment_id: PaymentId,
        amount: Money,
        paid_at: DateTime,
    },
    
    Cancelled {
        id: OrderId,
        reason: CancellationReason,
        cancelled_at: DateTime,
        refund_status: Option<RefundStatus>,
    },
}
```

## Visual Conceptual Spaces

### From Concept Graphs to Semantic Understanding

CIM uses visual concept graphs (Arrows.app) to define semantic spaces:

```cypher
// Visual concept graph in Arrows.app
// This becomes semantic understanding

// Quality dimensions as relationships
(Price)-[:DIMENSION {weight: 0.7}]->(Product)
(Quality)-[:DIMENSION {weight: 0.9}]->(Product)
(Availability)-[:DIMENSION {weight: 0.5}]->(Product)

// Concept regions
(Premium:Region {price: "high", quality: "high"})
(Budget:Region {price: "low", quality: "medium"})
(Luxury:Region {price: "very high", quality: "exceptional"})

// Prototypes
(iPhone:Prototype)-[:EXEMPLIFIES]->(Premium)
(GenericPhone:Prototype)-[:EXEMPLIFIES]->(Budget)
(VersacePhone:Prototype)-[:EXEMPLIFIES]->(Luxury)
```

### AI Interprets Visual Concepts

AI agents read your concept graphs and generate semantic understanding:

```rust
// AI-generated from Arrows.app graph
pub struct ProductSpace {
    dimensions: Vec<Dimension>,
    regions: Vec<ConceptRegion>,
}

impl ProductSpace {
    // AI generates similarity from visual relationships
    pub fn find_similar(&self, product: &Product) -> Vec<Product> {
        // Uses graph-defined dimensions and weights
        self.products
            .iter()
            .map(|p| (p, self.calculate_similarity(product, p)))
            .filter(|(_, sim)| *sim > 0.7)
            .map(|(p, _)| p.clone())
            .collect()
    }
}
```

### Visual Concept Evolution

```mermaid
graph TB
    subgraph "Concept Design (Arrows.app)"
        CD1[Define Concepts]
        CD2[Draw Relationships]
        CD3[Set Properties]
        CD4[Export Graph]
    end
    
    subgraph "AI Processing"
        AI1[Parse Concept Graph]
        AI2[Build Semantic Space]
        AI3[Generate Code]
    end
    
    subgraph "Runtime Understanding"
        R1[Concept Matching]
        R2[Similarity Search]
        R3[Semantic Queries]
    end
    
    CD1 --> CD2
    CD2 --> CD3
    CD3 --> CD4
    CD4 --> AI1
    AI1 --> AI2
    AI2 --> AI3
    AI3 --> R1
    R1 --> R2
    R2 --> R3
    
    %% Styling
    style CD1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style AI1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style R1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
```

## Distributed State Machines

### Visual Node Architecture

Nodes are designed visually, showing state machine distribution:

```mermaid
graph TB
    subgraph "Visual Design"
        VD[Node Layout in ExcaliDraw]
    end
    
    subgraph "Core State Machines"
        CS1[Order State Machines]
        CS2[Inventory State Machines]
        CS3[Customer State Machines]
    end
    
    subgraph "Edge State Machines"
        ES1[Local Order Cache]
        ES2[Offline Inventory]
        ES3[Mobile State Sync]
    end
    
    subgraph "AI Nodes"
        AI1[Graph Parser Node]
        AI2[Code Generator Node]
        AI3[State Optimizer Node]
    end
    
    VD --> CS1
    VD --> ES1
    VD --> AI1
    
    CS1 -.->|Replicate| ES1
    CS2 -.->|Sync| ES2
    ES3 -->|Eventually| CS3
    
    AI1 --> AI2
    AI2 --> AI3
    AI3 -->|Optimize| CS1
    
    %% Styling
    style VD fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style CS1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style ES1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style AI1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
```

### State Machine Distribution Pattern

```
Visual Node Design (ExcaliDraw)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

┌─────────────────┐
│   Central Hub   │
│ ┌─────────────┐ │
│ │Order States │ │
│ └─────────────┘ │
└────────┬────────┘
         │
    ┌────┴────┬─────────┐
    ▼         ▼         ▼
┌────────┐ ┌────────┐ ┌────────┐
│Store 1 │ │Store 2 │ │Mobile  │
│┌──────┐│ │┌──────┐│ │┌──────┐│
││Local ││ ││Local ││ ││Cache ││
││States││ ││States││ ││States││
│└──────┘│ │└──────┘│ │└──────┘│
└────────┘ └────────┘ └────────┘

Each node runs state machines
Synchronization via event replication
```

### Distributed State Consistency

```rust
// State machines handle distribution transparently
#[derive(DistributedStateMachine)]
pub struct OrderStateMachine {
    // Local state
    state: OrderState,
    
    // Distribution metadata
    node_id: NodeId,
    version: VectorClock,
    
    // Offline queue
    pending_sync: Vec<Event>,
}

impl OrderStateMachine {
    // State transitions work offline
    pub fn transition(&mut self, event: Event) -> Result<()> {
        match self.state {
            OrderState::Draft { .. } => {
                self.apply_event(event);
                self.version.increment(self.node_id);
                
                if self.is_offline() {
                    self.pending_sync.push(event);
                } else {
                    self.broadcast(event).await?;
                }
            }
            // Other states...
        }
        Ok(())
    }
}
```

## Security Through State Machines

### Visual Security Design

Security is modeled as state machines in your visual designs:

```
Authentication State Machine (ExcaliDraw)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

○ Anonymous
    │
    ├─Present Credentials─→ Authenticating
    │                           │
    │                           ├─Valid─→ Authenticated
    │                           │            │
    │                           └─Invalid─→ Anonymous
    │                                       │
    └───────────────────────────────────────┘
                     
Authenticated State includes:
- user_id: UserId
- permissions: Set<Permission>
- session_token: Token
- expires_at: DateTime
```

### Security State Machines

```mermaid
graph TB
    subgraph "Access Control State Machine"
        A1[No Access]
        A2[Read Access]
        A3[Write Access]
        A4[Admin Access]
    end
    
    subgraph "Session State Machine"
        S1[No Session]
        S2[Active Session]
        S3[Expired Session]
        S4[Revoked Session]
    end
    
    subgraph "Audit State Machine"
        AU1[Event Recorded]
        AU2[Event Verified]
        AU3[Event Archived]
    end
    
    A1 -->|Authenticate| A2
    A2 -->|Elevate| A3
    A3 -->|Grant Admin| A4
    
    S1 -->|Login| S2
    S2 -->|Timeout| S3
    S2 -->|Revoke| S4
    
    %% All transitions generate audit events
    A2 -.->|Log| AU1
    A3 -.->|Log| AU1
    S2 -.->|Log| AU1
    
    %% Styling
    style A1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style S2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style AU1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

### Policy as State Machines

```rust
// Security policies are state machines too
#[derive(PolicyStateMachine)]
pub enum AccessPolicy {
    Evaluating {
        request: AccessRequest,
        context: SecurityContext,
    },
    
    Permitted {
        request: AccessRequest,
        permissions: Vec<Permission>,
        expires_at: DateTime,
    },
    
    Denied {
        request: AccessRequest,
        reason: DenialReason,
        logged_at: DateTime,
    },
}

impl AccessPolicy {
    pub fn evaluate(&mut self, rules: &[Rule]) {
        match self {
            Self::Evaluating { request, context } => {
                let result = rules.iter()
                    .all(|rule| rule.check(request, context));
                
                *self = if result {
                    Self::Permitted {
                        request: request.clone(),
                        permissions: extract_permissions(rules),
                        expires_at: calculate_expiry(context),
                    }
                } else {
                    Self::Denied {
                        request: request.clone(),
                        reason: find_denial_reason(rules),
                        logged_at: Utc::now(),
                    }
                };
            }
            _ => {} // Already evaluated
        }
    }
}
```

## Performance Through Visual Design

### Performance Patterns in Graphs

Performance optimization starts in your visual designs:

```
Performance-Oriented State Design (ExcaliDraw)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

┌─────────────────────┐
│   Hot Path States   │ ← Mark frequently accessed
│  ┌──────────────┐  │
│  │ Active Order │  │ Properties:
│  │ (Cached)     │  │ - cache_ttl: 5min
│  └──────────────┘  │ - index_on: [customer_id, date]
└─────────────────────┘

┌─────────────────────┐
│   Cold Storage     │ ← Mark archival states  
│  ┌──────────────┐  │
│  │  Completed   │  │ Properties:
│  │  (Archive)   │  │ - compress: true
│  └──────────────┘  │ - retention: 7years
└─────────────────────┘
```

### State Machine Performance Patterns

```rust
// AI generates optimized code from visual hints
#[derive(StateMachine, HotPath)] // From visual annotation
pub enum OrderState {
    // Frequently accessed - kept in memory
    #[hot_state(cache_ttl = "5min")]
    Active {
        // Indexed fields from visual design
        #[indexed]
        customer_id: CustomerId,
        #[indexed]
        created_at: DateTime,
        items: Vec<LineItem>,
    },
    
    // Archived after completion
    #[cold_state(compress = true)]
    Completed {
        order_id: OrderId,
        completed_at: DateTime,
        #[compressed]
        history: Vec<Event>,
    },
}
```

### Visual Performance Monitoring

```mermaid
graph LR
    subgraph "Design Time"
        D1[Mark Hot Paths]
        D2[Identify Bottlenecks]
        D3[Plan Caching]
    end
    
    subgraph "Runtime Monitoring"
        R1[State Transition Metrics]
        R2[Event Flow Rates]
        R3[Cache Hit Rates]
    end
    
    subgraph "AI Optimization"
        A1[Analyze Patterns]
        A2[Suggest Improvements]
        A3[Update Graphs]
    end
    
    D1 --> R1
    D2 --> R2
    D3 --> R3
    
    R1 --> A1
    R2 --> A1
    R3 --> A1
    
    A1 --> A2
    A2 --> A3
    A3 -->|Feedback| D1
    
    %% Styling
    style D1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style R1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style A1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
```

## Extending CIM Through Graphs

### Visual Domain Extension Process

```mermaid
graph TB
    subgraph "1. Visual Discovery"
        V1[Event Storm Your Domain]
        V2[Draw State Machines]
        V3[Map Concepts]
        V4[Design Workflows]
    end
    
    subgraph "2. Export Artifacts"
        E1[ExcaliDraw → JSON]
        E2[Arrows → Cypher]
        E3[Package Designs]
    end
    
    subgraph "3. AI Generation"
        A1[Parse Visual Designs]
        A2[Generate Domain Code]
        A3[Create State Machines]
        A4[Build Event Types]
    end
    
    subgraph "4. Custom Logic"
        C1[Add Business Rules]
        C2[Implement Policies]
        C3[Define Integrations]
    end
    
    V1 --> V2
    V2 --> V3
    V3 --> V4
    V4 --> E1
    E1 --> E2
    E2 --> E3
    E3 --> A1
    A1 --> A2
    A2 --> A3
    A3 --> A4
    A4 --> C1
    C1 --> C2
    C2 --> C3
    
    %% Styling
    style V1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style A1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style C1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
```

### Creating a New Domain Extension

```bash
# 1. Start with visual design
mkdir -p cowboy-ai-inventory/design/graphs
cd cowboy-ai-inventory/design/graphs

# 2. Create your Event Storm
# Open ExcaliDraw and design:
# - Domain Events (orange sticky notes)
# - Commands (blue sticky notes)  
# - Aggregates (yellow sticky notes)
# - Policies (purple sticky notes)

# 3. Export and generate
cim ai new-domain \
  --from-graphs ./design/graphs \
  --name cowboy-ai-inventory

# AI reads your graphs and generates:
# - State machine implementations
# - Event definitions
# - Command handlers
# - Policy state machines
```

### Visual Plugin Architecture

```
Plugin System Design (ExcaliDraw)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

┌─────────────────┐
│  Core CIM       │
│  State Engine   │
└────────┬────────┘
         │
    ╔════╧════╗
    ║ Plugin  ║
    ║  API    ║
    ╚═╤═══╤═══╝
      │   │
      │   └────────────┐
      ▼                ▼
┌──────────┐    ┌──────────────┐
│ Storage  │    │ Custom State │
│ Plugin   │    │  Machines    │
└──────────┘    └──────────────┘

Plugins are state machines too!
```

## Visual-First Best Practices

### 1. Start with Graphs, Not Code

```
✅ RIGHT WAY
━━━━━━━━━━━
1. Event Storm in ExcaliDraw
2. Draw state machines
3. Map concepts in Arrows
4. Export for AI generation
5. Add custom business logic

❌ WRONG WAY
━━━━━━━━━━━
1. Write code
2. Create diagrams later
3. Documentation as afterthought
```

### 2. Everything is a State Machine

```
✅ CORRECT THINKING
━━━━━━━━━━━━━━━━━━
- Aggregates = State Machines
- Workflows = State Machines
- Policies = State Machines
- Sagas = State Machines
- Security = State Machines

❌ INCORRECT THINKING
━━━━━━━━━━━━━━━━━━━━
- Services and repositories
- Controllers and models
- Managers and handlers
```

### 3. Complete State Principle

```rust
// ✅ Good: Each state has ALL needed data
enum OrderState {
    Active {
        id: OrderId,
        customer: Customer,
        items: Vec<Item>,
        total: Money,
    },
}

// ❌ Bad: State requires external lookups
enum OrderState {
    Active {
        id: OrderId,
        // Where's the data?
    },
}
```

### 4. Visual Documentation IS the Documentation

```bash
# Your documentation structure
docs/
├── event-storms/      # Discovery sessions
├── state-machines/    # Behavior designs
├── context-maps/      # Boundaries
├── concept-graphs/    # Relationships
└── workflows/         # Process flows

# NOT this:
docs/
├── api-reference.md   # Generated after
├── class-diagrams.md  # We don't have classes
└── database-schema.md # Events are the schema
```

## The Visual Architecture Revolution

CIM's architecture represents a fundamental shift in how we build systems:

### Traditional vs CIM Architecture

```
TRADITIONAL ARCHITECTURE          CIM VISUAL ARCHITECTURE
━━━━━━━━━━━━━━━━━━━━━━━          ━━━━━━━━━━━━━━━━━━━━━
Code → Documentation             Graphs → Code
Objects and Services             State Machines Everywhere  
Developers Write Code            Developers Draw Behavior
AI Assists Coding                AI Generates from Graphs
Code is Truth                    Visual Design is Truth
```

### Why This Matters

1. **Business Understanding**: Graphs are universally understood
2. **AI-Native**: AI excels at interpreting visual patterns
3. **Correctness**: Invalid states impossible by design
4. **Evolution**: Change graphs, regenerate code
5. **Collaboration**: Everyone can contribute to design

### The State Machine Advantage

```mermaid
graph LR
    subgraph "Everything is Predictable"
        S1[Known States]
        S2[Explicit Transitions]
        S3[Complete Data]
        S4[No Surprises]
    end
    
    S1 --> S2
    S2 --> S3
    S3 --> S4
    
    style S1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style S4 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

### Getting Started

1. **Forget everything you know** about services and repositories
2. **Think in states** - What states can things be in?
3. **Draw before coding** - If you can't draw it, you don't understand it
4. **Let AI help** - It's better at generating code than you are
5. **Trust the process** - Visual-first works

---

*Next: Start your journey with the [Event Storming Guide](./event-storming-guide.md)*