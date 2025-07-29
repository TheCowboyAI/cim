# CIM Quick Start Guide

Get up and running with the Composable Information Machine in 15 minutes.

## What You'll Build

In this guide, you'll create a simple information system that:
- Tracks people and organizations
- Handles document management
- Responds to business events
- Provides AI-powered insights

## Prerequisites

- Git installed
- Docker or Podman (for NATS)
- Development environment for your preferred language (Rust, Python, or Node.js)
- Basic understanding of event-driven architecture

## Step 1: Install NATS (5 minutes)

CIM uses NATS for event messaging. Start a NATS server with JetStream enabled:

```bash
# Using Docker
docker run -d --name nats-cim -p 4222:4222 -p 8222:8222 nats:latest -js

# Or using Podman
podman run -d --name nats-cim -p 4222:4222 -p 8222:8222 nats:latest -js

# Verify it's running
curl http://localhost:8222/varz
```

## Step 2: Create Your First CIM Project (5 minutes)

### Option A: Using Rust

```bash
# Clone the starter template
git clone https://github.com/thecowboyai/cim-start my-cim-project
cd my-cim-project

# Install dependencies
cargo build

# Run the example
cargo run --example basic_domain
```

### Option B: Using Python

```bash
# Clone the Python starter
git clone https://github.com/thecowboyai/cim-python-start my-cim-project
cd my-cim-project

# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run the example
python examples/basic_domain.py
```

### Option C: Using Node.js

```bash
# Clone the Node.js starter
git clone https://github.com/thecowboyai/cim-node-start my-cim-project
cd my-cim-project

# Install dependencies
npm install

# Run the example
npm run example:basic
```

## Step 3: Understanding the Basic Structure (3 minutes)

Your project contains these key components:

```
my-cim-project/
├── domains/           # Business domain definitions
│   ├── person.rs     # Person aggregate
│   └── organization.rs # Organization aggregate
├── events/           # Domain events
│   └── mod.rs       # Event definitions
├── handlers/         # Command and query handlers
│   ├── commands.rs  # Business operations
│   └── queries.rs   # Data retrieval
└── main.rs          # Application entry point
```

### Core Concepts in Action

1. **Domain Aggregate** - Business entity with behavior:
```rust
// domains/person.rs
pub struct Person {
    pub id: String,
    pub name: String,
    pub email: String,
    pub organization_id: Option<String>,
}
```

2. **Domain Event** - Something that happened:
```rust
// events/mod.rs
pub enum PersonEvent {
    Created { id: String, name: String, email: String },
    JoinedOrganization { person_id: String, org_id: String },
}
```

3. **Command Handler** - Business operation:
```rust
// handlers/commands.rs
pub async fn create_person(
    name: String,
    email: String,
) -> Result<String, Error> {
    let person = Person::new(name, email);
    store.save(person).await?;
    Ok(person.id)
}
```

## Step 4: Run Your First Workflow (2 minutes)

Let's create a person and have them join an organization:

```bash
# Using the CLI tool (if using Rust starter)
./target/debug/my-cim-project create-person "Alice Smith" "alice@example.com"
# Output: Created person with ID: person_123abc

./target/debug/my-cim-project create-org "Acme Corp"
# Output: Created organization with ID: org_456def

./target/debug/my-cim-project join-org person_123abc org_456def
# Output: Alice Smith joined Acme Corp
```

## Step 5: See Events in Action

The system automatically generated and processed these events:

1. `PersonCreated` event
2. `OrganizationCreated` event
3. `PersonJoinedOrganization` event

Check the event log:

```bash
# View recent events
./target/debug/my-cim-project show-events --recent 10
```

## What Just Happened?

You've just:
1. ✅ Set up a distributed messaging system (NATS)
2. ✅ Created domain models (Person, Organization)
3. ✅ Implemented event sourcing
4. ✅ Executed commands that generate events
5. ✅ Queried the current state

## Next Steps

### 1. Add AI Capabilities

```bash
# Clone the Alchemist AI agent
git clone https://github.com/thecowboyai/cim-agent-alchemist
cd cim-agent-alchemist

# Configure it to connect to your NATS instance
echo "NATS_URL=nats://localhost:4222" > .env

# Run the AI agent
cargo run
```

Now you can ask questions in natural language:
- "Show me all people in Acme Corp"
- "What organizations exist?"
- "Create a new person named Bob"

### 2. Deploy to the Edge

```bash
# Clone CIM Leaf for edge deployment
git clone https://github.com/thecowboyai/cim-leaf
cd cim-leaf

# Build for your edge device
cargo build --release --target=aarch64-unknown-linux-gnu
```

### 3. Add More Domains

Explore available domains:
- `cim-domain-finance` - Financial transactions
- `cim-domain-inventory` - Inventory management
- `cim-domain-workflow` - Business workflows
- `cim-domain-documents` - Document management

### 4. Create Visual Workflows

```bash
# Install the workflow designer
git clone https://github.com/thecowboyai/cim-workflow-designer
cd cim-workflow-designer
npm install && npm start
```

## Common Patterns

### Event-Driven Integration

```rust
// Subscribe to events from any domain
event_bus.subscribe("PersonCreated", |event| {
    println!("New person: {:?}", event);
    // Trigger workflows, send notifications, etc.
});
```

### Cross-Domain Queries

```rust
// Query across domains using the knowledge graph
let results = knowledge_graph.query(r#"
    MATCH (p:Person)-[:WORKS_AT]->(o:Organization)
    WHERE o.name = "Acme Corp"
    RETURN p.name, p.email
"#).await?;
```

### Policy-Based Automation

```rust
// Define business policies
policy_engine.add_rule(
    "auto_approve_small_purchases",
    When::amount_less_than(1000),
    Then::auto_approve(),
);
```

## Troubleshooting

### NATS Connection Failed
```bash
# Check if NATS is running
docker ps | grep nats-cim

# View NATS logs
docker logs nats-cim
```

### Domain Not Found
```bash
# List available domains
ls ~/git/thecowboyai/cim-domain-*

# Install a domain module
git clone https://github.com/thecowboyai/cim-domain-finance
```

### Events Not Processing
```bash
# Check event stream health
curl http://localhost:8222/streaming/serverz
```

## Learn More

- **[Architecture Deep Dive](./architecture-deep-dive.md)** - Understand the internals
- **[Domain Development Guide](./domain-development-guide.md)** - Build custom domains
- **[CIM Comprehensive Manual](./cim_comprehensive_manual.md)** - Complete reference
- **[Module Catalog](./cim_modules_catalog.md)** - Available components

## Get Help

- 📖 Documentation: This repository
- 🐛 Issues: [GitHub Issues](https://github.com/thecowboyai/cim/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/thecowboyai/cim/discussions)
- 🤖 AI Assistant: Use `cim-agent-alchemist` for interactive help

---

**Congratulations!** You've successfully set up your first CIM information system. The power of CIM lies in its composability - start small and grow your system as your needs evolve.

*Remember: In CIM, information flows like water - it finds its own level and connects naturally.*