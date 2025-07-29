# Workflow Graph Guide

In CIM, workflows are visual graphs that represent business processes. These graphs are directly executable by the CIM runtime, with AI agents handling the implementation details.

## Philosophy: Graphs as Business Logic

Traditional workflow engines require code. CIM requires only graphs.

```
Traditional: BPMN → Code → Execution
CIM:         Business Understanding → Visual Graph → Direct Execution
```

## Tools for Workflow Design

### Primary: ExcaliDraw with CIM Workflow Library

```json
// CIM Workflow Elements Library
{
  "elements": {
    "start": {
      "shape": "circle",
      "color": "#95E1D3",
      "label": "Start"
    },
    "activity": {
      "shape": "rectangle",
      "color": "#4ECDC4",
      "label": "Activity"
    },
    "decision": {
      "shape": "diamond",
      "color": "#FFE66D",
      "label": "Decision?"
    },
    "event": {
      "shape": "rounded-rectangle",
      "color": "#FF9F43",
      "label": "Event"
    },
    "end": {
      "shape": "double-circle",
      "color": "#FF6B6B",
      "label": "End"
    }
  }
}
```

### Alternative: Draw.io with CIM Templates

For teams already using draw.io:
- CIM Workflow template available
- Export as `.drawio` (XML)
- AI agents can parse both formats

## Workflow Patterns

### 1. Sequential Flow

The simplest pattern - one step after another:

```
○ Start
  │
  ▼
┌─────────────┐
│   Receive   │
│    Order    │
└─────────────┘
  │
  ▼
┌─────────────┐
│   Validate  │
│    Order    │
└─────────────┘
  │
  ▼
┌─────────────┐
│   Process   │
│   Payment   │
└─────────────┘
  │
  ▼
◎ End
```

### 2. Conditional Flow

Decisions change the path:

```
┌─────────────┐
│Check Credit │
└─────────────┘
       │
       ▼
    ◆─────◆
   ╱ Credit ╲    Yes
  ◆ OK?      ◆────────▶ [Process Order]
   ╲         ╱
    ◆─────◆
       │ No
       ▼
[Require Prepayment]
```

### 3. Parallel Flow

Multiple activities happen simultaneously:

```
       ┌──────────────┐
       │ Order Placed │
       └──────────────┘
              │
     ╔════════╧════════╗
     ║  Fork (AND)     ║
     ╚═══╤═══════╤═════╝
         │       │
         ▼       ▼
   ┌──────────┐ ┌──────────────┐
   │ Reserve  │ │Send Customer │
   │Inventory │ │Confirmation  │
   └──────────┘ └──────────────┘
         │       │
     ╔═══╧═══════╧═════╗
     ║  Join (AND)     ║
     ╚════════╤════════╝
              ▼
       [Continue Process]
```

### 4. Event-Driven Flow

Workflows triggered by events:

```
╭─────────────╮     ╭─────────────╮     ╭─────────────╮
│Stock Level  │     │  Customer   │     │   Time      │
│Low Event    │     │Registration │     │  Based      │
╰─────────────╯     ╰─────────────╯     ╰─────────────╯
      │                    │                    │
      ▼                    ▼                    ▼
[Reorder Flow]    [Welcome Flow]    [Monthly Report]
```

## Creating Executable Workflows

### Step 1: Start with Business Understanding

In ExcaliDraw, begin with the happy path:

```
"Customer places order → We ship it → Customer is happy"
```

### Step 2: Add Business Rules

Add decision points for rules:

```
Order Placed
     │
     ▼
◆ In Stock? ◆──No──▶ [Backorder Flow]
     │Yes
     ▼
◆ Payment OK? ◆──No──▶ [Payment Retry]
     │Yes
     ▼
Ship Order
```

### Step 3: Add Error Handling

Every activity can fail:

```
┌─────────────┐
│Process Order│
└─────────────┘
  │Success  │Error
  ▼         ▼
[Next]   [Compensate]
```

### Step 4: Export for Execution

```bash
# Export from ExcaliDraw
Menu → Export → JSON
Save as: workflows/order-fulfillment.excalidraw

# AI agent generates executable workflow
cim ai generate-workflow \
  --graph workflows/order-fulfillment.excalidraw \
  --domain cowboy-ai-orders
```

## Advanced Workflow Patterns

### 1. Saga Pattern

Long-running transactions with compensations:

```
Start Transaction
       │
       ▼
┌─────────────┐
│Book Flight  │────❌───▶ [No Compensation Needed]
└─────────────┘
   │✓
   ▼
┌─────────────┐
│Book Hotel   │────❌───▶ [Cancel Flight]
└─────────────┘
   │✓
   ▼
┌─────────────┐
│Charge Card  │────❌───▶ [Cancel Hotel] → [Cancel Flight]
└─────────────┘
   │✓
   ▼
Transaction Complete
```

### 2. Human-in-the-Loop

Workflows requiring human decisions:

```
┌─────────────────┐
│ Auto-Generated  │
│    Report       │
└─────────────────┘
         │
         ▼
  ╭─────────────╮
  │Send to Human│
  │ for Review  │
  ╰─────────────╯
         │
    ◆────┴────◆
   ╱ Approved? ╲    Yes
  ◆             ◆────────▶ [Publish]
   ╲           ╱
    ◆─────────◆
         │ No
         ▼
   [Request Changes]
         │
         └────────┐
                  │
                  ▼
           [Loop Back to Edit]
```

### 3. Time-Based Workflows

Workflows with temporal elements:

```
Order Placed
     │
     ▼
┌─────────────────┐
│Wait for Payment│
│  Max: 24 hours │
└─────────────────┘
   │              │Timeout
   │Paid          ▼
   ▼         [Cancel Order]
[Process Order]
```

### 4. Multi-Domain Workflows

Workflows spanning multiple domains:

```
Sales Domain          Inventory Domain       Shipping Domain
     │                      │                      │
Order Placed ─────────▶ Check Stock
     │                      │
     │                 Available? ──No──▶ [End: Out of Stock]
     │                      │Yes
     │                      ▼
     │                Reserve Items
     │                      │
     │◀─────────────────────┘
     ▼
Process Payment
     │
     └─────────────────────────────────▶ Create Shipment
                                              │
                                              ▼
                                         Ship Package
```

## Workflow Metadata

### Adding Properties to Your Graphs

In ExcaliDraw, use text annotations:

```
┌─────────────────┐
│ Process Payment │
└─────────────────┘
Properties:
- timeout: 30s
- retry: 3
- fallback: manual_process
- domain: payments
- actor: PaymentService
```

### Defining SLAs

```
┌─────────────────────┐
│ Complete Order      │
│ SLA: 2 hours       │
│ Alert: ops@company │
└─────────────────────┘
```

## Workflow Testing

### Visual Test Cases

Create test scenarios as graphs:

```
Test: Payment Failure
━━━━━━━━━━━━━━━━━━━
Given: Valid Order
When: Payment Fails 3 Times
Then: Order Cancelled

○ Start Test
     │
     ▼
[Create Order]
     │
     ▼
[Inject: Payment Failure × 3]
     │
     ▼
◆ Order Status? ◆
     │
Should be: Cancelled
```

### Property Testing

Define invariants visually:

```
Invariant: No Double Charging
━━━━━━━━━━━━━━━━━━━━━━━━━━━
┌─────────────┐     ┌─────────────┐
│   Charge    │ ──▶ │   Charged   │
│  Attempted  │     │   Once      │
└─────────────┘     └─────────────┘
                          │
                    ❌ Cannot transition back
```

## Integration with State Machines

Workflows orchestrate state machines:

```
Workflow Step               State Machine Transition
─────────────               ───────────────────────
Order Placed        ──▶     Order.Draft → Order.Placed
     │
Payment Processed   ──▶     Order.Placed → Order.Paid
     │                      Payment.Pending → Payment.Completed
     │
Items Shipped       ──▶     Order.Paid → Order.Shipped
                           Inventory.Reserved → Inventory.Shipped
```

## Best Practices

### 1. Start Simple

Begin with the happy path:
```
Start → Do Thing → End
```

Then add complexity:
```
Start → Do Thing → Did it work? → Yes: End
                        │
                        No: Handle Error → End
```

### 2. Name Activities Clearly

❌ Bad:
```
Process → Check → Update → Done
```

✅ Good:
```
Validate Order → Check Inventory → Update Stock Level → Order Complete
```

### 3. Show Domain Boundaries

Use swimlanes or colors:

```
┌─────Sales Domain─────┐ ┌────Inventory Domain────┐
│                      │ │                        │
│ Order Placed         │ │                        │
│      │               │ │                        │
│      └───────────────┼─┼──▶ Check Stock        │
│                      │ │         │              │
│                      │ │         ▼              │
│                      │ │    Reserve Items      │
│                      │ │         │              │
│      ◀───────────────┼─┼─────────┘              │
│      │               │ │                        │
└──────────────────────┘ └────────────────────────┘
```

### 4. Version Your Workflows

```bash
workflows/
├── v1/
│   └── order-fulfillment.excalidraw  # Original
├── v2/
│   └── order-fulfillment.excalidraw  # Added express shipping
└── current/
    └── order-fulfillment.excalidraw  # Symlink to latest
```

## Workflow Execution

### How CIM Executes Graphs

1. **Parse Graph**: Extract nodes and edges
2. **Map to Domains**: Each activity maps to domain commands
3. **Generate Execution Plan**: Determine order and parallelism
4. **Execute**: Run with monitoring and error handling

### Runtime Monitoring

CIM provides real-time workflow visualization:

```
Current Execution: Order #12345
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[✓] Order Placed        (2ms)
[✓] Inventory Checked   (15ms)
[▶] Payment Processing  (waiting...)
[ ] Shipping Label
[ ] Send Confirmation
```

## Common Pitfalls

### 1. Over-Engineering

❌ Don't create 50-step workflows
✅ Break into smaller, composable workflows

### 2. Missing Error Paths

❌ Only showing happy path
✅ Every activity can fail - show how to handle it

### 3. Tight Coupling

❌ Workflow knows internal domain details
✅ Workflow orchestrates domains via commands

### 4. No Timeouts

❌ Waiting forever for human approval
✅ Set reasonable timeouts with escalation

## Examples

### E-Commerce Order Fulfillment

```
○ Order Received
       │
       ▼
┌─────────────────┐
│ Validate Order  │
└─────────────────┘
       │
    ◆──┴──◆
   ╱ Valid? ╲    No
  ◆          ◆────────▶ [Notify Customer: Invalid Order] → ◎
   ╲        ╱
    ◆──────◆
       │ Yes
       ▼
  ╔═══════════╗
  ║   Fork    ║
  ╚═══╤═══╤═══╝
      │   │
      │   └──────────┐
      ▼              ▼
[Check Stock]  [Check Credit]
      │              │
      ▼              ▼
[Reserve Items] [Authorize Payment]
      │              │
  ╔═══╧═══════╧═══╗
  ║     Join      ║
  ╚═══════╤═══════╝
          ▼
   [Create Shipment]
          │
          ▼
    [Send Confirmation]
          │
          ▼
          ◎
```

### Approval Workflow

```
○ Document Submitted
         │
         ▼
  ┌──────────────┐
  │ Route to     │
  │ Approver     │
  └──────────────┘
         │
         ▼
  ╭──────────────╮
  │Wait: 48 hours│
  ╰──────────────╯
    │         │Timeout
    │         ▼
    │    [Escalate to Manager]
    ▼
◆ Decision? ◆
 ╱│╲
Approve│Reject│Request Info
 │    │      │
 ▼    ▼      ▼
[Publish] [Archive] [Return to Author]
 │    │             │
 ▼    ▼             └──▶ [Loop back]
 ◎    ◎
```

## Next Steps

1. **Create Your First Workflow**: Start with a simple business process
2. **Export and Test**: Use `cim ai test-workflow` to validate
3. **Deploy**: Workflows are automatically deployed with domains
4. **Monitor**: Use CIM Portal to see workflows in action

---

**Remember**: In CIM, workflows are visual specifications. If you can draw it, CIM can run it.