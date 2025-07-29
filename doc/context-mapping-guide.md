# Context and Concept Mapping Guide

Context and Concept mapping in CIM is done visually using graphs. These graphs become the source of truth that AI agents use to generate domain code.

## Visual-First Design Philosophy

In CIM, we don't write code to describe relationships - we draw them. The graphs ARE the design.

```
Traditional: Code → Documentation → Understanding
CIM:         Understanding → Graphs → AI-Generated Code
```

## Tools for Context Mapping

### ExcaliDraw for Context Maps

ExcaliDraw is perfect for showing bounded contexts and their relationships:

```json
// CIM Context Map Template
{
  "style": {
    "core_context": {
      "backgroundColor": "#FF6B6B",
      "strokeColor": "#C92A2A",
      "strokeWidth": 4,
      "strokeStyle": "solid"
    },
    "supporting_context": {
      "backgroundColor": "#4ECDC4",
      "strokeColor": "#2B8A89",
      "strokeWidth": 3,
      "strokeStyle": "solid"
    },
    "generic_context": {
      "backgroundColor": "#95E1D3",
      "strokeColor": "#63C7B8",
      "strokeWidth": 2,
      "strokeStyle": "dashed"
    },
    "external_context": {
      "backgroundColor": "#FFE66D",
      "strokeColor": "#FCC419",
      "strokeWidth": 3,
      "strokeStyle": "dotted"
    }
  }
}
```

### Arrows.app for Concept Relationships

Arrows.app excels at showing concept relationships as property graphs:

```cypher
// Example Concept Graph in Arrows.app
// This becomes executable by AI agents

// Nodes (Concepts)
(customer:Concept {name: "Customer", type: "Entity"})
(order:Concept {name: "Order", type: "Aggregate"})
(product:Concept {name: "Product", type: "Entity"})
(inventory:Concept {name: "Inventory", type: "Aggregate"})

// Relationships
(customer)-[:PLACES {cardinality: "1..*"}]->(order)
(order)-[:CONTAINS {cardinality: "1..*"}]->(product)
(product)-[:TRACKED_BY {cardinality: "1..1"}]->(inventory)
```

## Context Mapping Process

### Step 1: Identify Bounded Contexts

Using ExcaliDraw, create visual bounded contexts:

```
┌─────────────────────────────────────┐
│        SALES CONTEXT                │
│  ┌─────────┐     ┌─────────┐      │
│  │Customer │     │  Order  │      │
│  └─────────┘     └─────────┘      │
│                                    │
│  Core Domain - Revenue Generation  │
└─────────────────────────────────────┘
          │
          │ [Customer-Supplier]
          ▼
┌─────────────────────────────────────┐
│      INVENTORY CONTEXT              │
│  ┌─────────┐     ┌─────────┐      │
│  │Product  │     │  Stock  │      │
│  └─────────┘     └─────────┘      │
│                                    │
│  Supporting - Stock Management     │
└─────────────────────────────────────┘
```

### Step 2: Define Context Relationships

#### Relationship Types in ExcaliDraw

Create a visual legend for your team:

```
━━━━━━▶  Partnership (Mutual benefit)
────▶   Customer-Supplier (Upstream provides to downstream)
┅┅┅┅▶   Conformist (Downstream conforms to upstream)
····▶   Anti-Corruption Layer (Translation required)
═══▶    Shared Kernel (Shared code/model)
```

#### Context Map Example

```
Sales ────▶ Inventory
      "Orders consume stock"

Inventory ┅┅┅┅▶ Supplier
         "Must use supplier's catalog format"

Sales ━━━━━━▶ Marketing
      "Share customer insights"

All Contexts ····▶ Legacy System
            "ACL for old payment gateway"
```

### Step 3: Export for AI Processing

```bash
# Export ExcaliDraw context map
Menu → Export → Export to JSON
Save as: contexts/sales-inventory-map.excalidraw

# The AI agent reads this to understand:
# - Which contexts exist
# - How they relate
# - What translation layers are needed
```

## Concept Mapping with Arrows.app

### Step 1: Create Core Concepts

In Arrows.app, model your domain concepts as nodes:

```
Style Guide:
- Entity: Rectangle, Blue (#4ECDC4)
- Value Object: Rounded Rectangle, Green (#95E1D3)
- Aggregate: Double Rectangle, Red (#FF6B6B)
- Event: Diamond, Orange (#FFE66D)
```

### Step 2: Define Relationships

Draw relationships with properties:

```cypher
// In Arrows.app, this visual becomes:
(Order)-[:PLACED_BY {
  since: "timestamp",
  channel: "web|mobile|phone"
}]->(Customer)

(Order)-[:CONTAINS {
  quantity: "integer",
  price: "decimal"
}]->(Product)

(Product)-[:STORED_IN {
  quantity: "integer",
  location: "string"
}]->(Warehouse)
```

### Step 3: Add Constraints and Rules

Use node properties for business rules:

```cypher
(Order:Aggregate {
  invariants: [
    "total > 0",
    "items.count > 0",
    "customer != null"
  ],
  states: [
    "Draft",
    "Placed",
    "Paid",
    "Shipped"
  ]
})
```

### Step 4: Export for Domain Generation

```bash
# From Arrows.app
Export → Cypher → Copy to Clipboard
Save as: concepts/order-domain-concepts.cypher

# Also export as JSON for richer metadata
Export → JSON → Download
Save as: concepts/order-domain-concepts.json
```

## AI Agent Integration

### How AI Agents Use Your Graphs

1. **Read ExcaliDraw Context Maps**:
   ```python
   # AI agent code
   context_map = parse_excalidraw("contexts/sales-inventory-map.excalidraw")
   contexts = extract_bounded_contexts(context_map)
   relationships = extract_relationships(context_map)
   ```

2. **Parse Arrows.app Concepts**:
   ```python
   # AI agent code
   concepts = parse_arrows_graph("concepts/order-domain-concepts.json")
   aggregates = [c for c in concepts if c.type == "Aggregate"]
   entities = [c for c in concepts if c.type == "Entity"]
   ```

3. **Generate Domain Code**:
   ```rust
   // AI-generated from graphs
   pub mod sales_context {
       use cim_domain::prelude::*;
       
       // Generated from Arrows.app concept graph
       #[derive(Aggregate)]
       pub struct Order {
           // Properties from graph
           pub id: OrderId,
           pub customer_id: CustomerId,
           pub items: Vec<LineItem>,
           pub total: Money,
           pub state: OrderState,
       }
       
       // State machine from graph states
       pub enum OrderState {
           Draft,
           Placed,
           Paid,
           Shipped,
       }
   }
   ```

## Best Practices

### 1. Use Consistent Visual Language

Create a team style guide:

```json
// team-style-guide.json
{
  "contexts": {
    "core": { "color": "#FF6B6B", "border": "solid-4px" },
    "supporting": { "color": "#4ECDC4", "border": "solid-3px" },
    "generic": { "color": "#95E1D3", "border": "dashed-2px" }
  },
  "concepts": {
    "aggregate": { "shape": "double-rectangle", "color": "#FF6B6B" },
    "entity": { "shape": "rectangle", "color": "#4ECDC4" },
    "value": { "shape": "rounded-rectangle", "color": "#95E1D3" },
    "event": { "shape": "diamond", "color": "#FFE66D" }
  }
}
```

### 2. Version Control Your Graphs

```bash
project/
├── contexts/
│   ├── README.md
│   ├── sales-inventory-map.excalidraw
│   ├── sales-inventory-map.svg  # For documentation
│   └── all-contexts-overview.excalidraw
├── concepts/
│   ├── sales-concepts.arrows
│   ├── sales-concepts.cypher
│   ├── sales-concepts.json
│   └── inventory-concepts.arrows
└── user-stories/
    ├── order-placement.md
    └── inventory-reorder.md
```

### 3. Link User Stories to Graphs

In your user stories, reference the graphs:

```markdown
# User Story: Place Order

As a customer
I want to place an order
So that I can purchase products

## Concepts
See: `concepts/sales-concepts.arrows#Order`

## Context
See: `contexts/sales-inventory-map.excalidraw#SalesContext`

## Acceptance Criteria
- Order moves through states defined in graph
- Inventory checked per relationship in graph
- Events emitted as shown in concept model
```

## Common Patterns

### 1. Upstream/Downstream Pattern

```
┌─────────────┐         ┌─────────────┐
│  Upstream   │────────▶│ Downstream  │
│  (Supplier) │         │ (Customer)  │
└─────────────┘         └─────────────┘

Upstream decides the interface
Downstream must adapt
```

### 2. Anti-Corruption Layer Pattern

```
┌─────────────┐  ┌─────┐  ┌─────────────┐
│   Modern    │  │ ACL │  │   Legacy    │
│   Context   │◀─┤     ├─▶│   System    │
└─────────────┘  └─────┘  └─────────────┘

ACL translates between models
Protects modern context from legacy complexity
```

### 3. Shared Kernel Pattern

```
┌─────────────┐         ┌─────────────┐
│  Context A  │═════════│  Context B  │
└─────────────┘ Shared  └─────────────┘
                Kernel

Both contexts share core concepts
Changes require coordination
```

## Concept Modeling Patterns

### 1. Aggregate Boundaries

In Arrows.app, show aggregate boundaries clearly:

```cypher
// Order aggregate boundary
(Order)-[:OWNS]->(LineItem)
(Order)-[:OWNS]->(ShippingAddress)
(Order)-[:REFERENCES]->(Customer)  // Different aggregate

// Rule: OWNS = within aggregate boundary
// Rule: REFERENCES = crosses boundary
```

### 2. Event Flows

Show how events flow between concepts:

```cypher
(Customer)-[:TRIGGERS {action: "places order"}]->(OrderPlacedEvent)
(OrderPlacedEvent)-[:CREATES]->(Order)
(OrderPlacedEvent)-[:NOTIFIES]->(Inventory)
(OrderPlacedEvent)-[:SCHEDULES]->(PaymentProcessing)
```

### 3. State Transitions

Model state machines visually:

```cypher
// States as nodes
(DraftState:State)
(PlacedState:State)
(PaidState:State)

// Transitions as relationships
(DraftState)-[:TRANSITIONS_TO {trigger: "submit"}]->(PlacedState)
(PlacedState)-[:TRANSITIONS_TO {trigger: "pay"}]->(PaidState)
```

## Validating Your Maps

### 1. Business Validation

Show graphs to domain experts:
- "Is this how you think about the business?"
- "What's missing?"
- "What relationships are wrong?"

### 2. Technical Validation

Check with developers:
- "Can we implement these boundaries?"
- "Are the relationships feasible?"
- "What technical constraints exist?"

### 3. AI Validation

Let AI analyze your graphs:

```bash
# Use CIM AI to validate
cim ai validate-context contexts/sales-inventory-map.excalidraw

# Output:
# ✓ Contexts have clear boundaries
# ✓ No circular dependencies
# ⚠ Missing error handling context
# ✓ All relationships typed correctly
```

## Evolution and Maintenance

### 1. Living Documents

Graphs should evolve with understanding:

```bash
# Track changes
git diff contexts/sales-inventory-map.excalidraw

# Meaningful commits
git commit -m "Added return processing context after customer feedback"
```

### 2. Impact Analysis

Before changing graphs, analyze impact:

```bash
# AI agent analyzes change impact
cim ai impact-analysis \
  --before contexts/v1/sales.excalidraw \
  --after contexts/v2/sales.excalidraw

# Output:
# Breaking changes:
# - Order aggregate adds required field
# - Customer relationship cardinality changed
# 
# Affected domains:
# - cowboy-ai-orders
# - cowboy-ai-inventory
```

## Deliverables

After context and concept mapping, you should have:

1. **Context Maps** (ExcaliDraw)
   - All bounded contexts identified
   - Relationships documented
   - Team boundaries clear

2. **Concept Graphs** (Arrows.app)
   - All aggregates modeled
   - Relationships with properties
   - Constraints documented

3. **Exported Artifacts**
   - `.excalidraw` files in git
   - `.cypher` and `.json` from Arrows
   - `.svg` exports for documentation

4. **AI-Ready Specs**
   - Graphs that AI can parse
   - User stories linked to graphs
   - Validation completed

## Next Steps

With your context and concept maps complete:

1. **Generate Domain Code**: `cim ai generate-domain --from-graphs ./contexts ./concepts`
2. **Create Workflow Graphs**: [Workflow Graph Guide](./workflow-graph-guide.md)
3. **Implement State Machines**: [State Machine Patterns](./state-machine-patterns.md)

---

**Remember**: In CIM, the graph IS the design. Code is just an implementation detail that AI handles.