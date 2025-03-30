# CIM Ontology Client

A NATS-based client for the CIM Ontology Tool using the Model Context Protocol (MCP).

## Features

- **Core Library**: Robust Rust client for interacting with the CIM Ontology Tool through NATS messaging
- **CLI Interface**: Command-line tool for managing ontologies, terms, and relationships
- **GUI Application**: Graphical interface built with Iced for visualizing and editing ontologies
- **Domain-Driven Design**: Follows DDD principles with a clean separation of concerns
- **Async Support**: Fully asynchronous API built on Tokio and async-nats

## Installation

### From Source

Clone the repository and build the project:

```bash
git clone <repository-url>
cd cim-client
cargo build --release
```

### CLI Only

```bash
cargo install --path . --features cli
```

### GUI Only

```bash
cargo install --path . --features gui
```

## Usage

### Library

```rust
use cim_client::{MCPClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new MCP client connected to the NATS server
    let client = MCPClient::new("nats://localhost:4222").await?;
    
    // List all ontologies
    let ontologies = client.list_ontologies().await?;
    println!("Found {} ontologies", ontologies.len());
    
    // Create a new ontology
    let ontology = client.create_ontology(
        "My Ontology",
        "A description of my ontology",
        "MyDomain",
    ).await?;
    
    println!("Created ontology: {} (ID: {})", ontology.name, ontology.id);
    
    Ok(())
}
```

### CLI

```bash
# List all ontologies
cim-client --nats-url nats://localhost:4222 list-ontologies

# Get a specific ontology
cim-client --nats-url nats://localhost:4222 get-ontology <ontology-id>

# Create a new ontology
cim-client --nats-url nats://localhost:4222 create-ontology "My Ontology" "A description of my ontology" "MyDomain"
```

### GUI

```bash
cim-client-gui
```

## Architecture

The client follows a domain-driven design approach:

- **Domain Layer**: Core entities and value objects (Ontology, Term, Relationship)
- **Application Layer**: Use cases for interacting with the domain
- **Infrastructure Layer**: NATS client implementation
- **Presentation Layer**: CLI and GUI interfaces

## Development

### Prerequisites

- Rust 1.68 or later
- NATS server running locally or remotely
- (Optional) CIM Ontology Tool server

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the example
cargo run --example simple_client
```

## License

MIT License 