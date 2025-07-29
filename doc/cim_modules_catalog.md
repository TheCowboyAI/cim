# CIM Modules Catalog

## Overview
This catalog presents all available modules for the Composable Information Machine (CIM) ecosystem, organized by functional categories.

```mermaid
mindmap
  root((CIM Modules))
    Core System
      cim-core-domain
      cim-component
      cim-infrastructure
      cim-bridge
      cim-compose
      cim-start
    AI & Agents
      alchemist
      cim-agent-alchemist
      cim-domain-agent
      cim-domain-dialog
    Knowledge Systems
      cim-conceptual-core
      cim-domain-conceptualspaces
      cim-conceptgraph
      cim-contextgraph
    Business Domains
      cim-domain
      cim-domain-identity
      cim-domain-organization
      cim-domain-person
      cim-identity-context
      cim-domain-location
      cim-domain-document
    Technical Domains
      cim-domain-git
      cim-domain-nix
      cim-domain-workflow
      cim-workflow-graph
      cim-domain-graph
    Security & Policy
      cim-security
      cim-domain-policy
      cim-keys
    Data & Storage
      cim-ipld
      cim-ipld-graph
      cim-subject
    Visualization
      cim-domain-bevy
      cim-viz-bevy
    Edge Computing
      cim-leaf
      cim-leaf-darwin
      cim-portal
      cim-stb
```

## Module Categories

### 🧠 Core System & Infrastructure

```mermaid
graph LR
    subgraph "Core Infrastructure Stack"
        A[cim-start] -->|Uses| B[cim-core-domain]
        B -->|Defines| C[cim-component]
        C -->|Deployed by| D[cim-infrastructure]
        D -->|Connected via| E[cim-bridge]
        E -->|Composed with| F[cim-compose]
        
        G[NATS] -.->|Powers| E
        H[IPLD] -.->|Storage for| D
        
        style A fill:#f96,stroke:#333,stroke-width:2px
        style B fill:#fc9,stroke:#333,stroke-width:2px
        style D fill:#9cf,stroke:#333,stroke-width:2px
    end
```

- **cim-core-domain**: Core domain models and abstractions
- **cim-component**: Core component definitions for the Composable Information Machine
- **cim-infrastructure**: Infrastructure layer with NATS integration and persistence
- **cim-bridge**: CIM Bridge to NATS messaging system
- **cim-compose**: Composition utilities and helpers for CIM
- **cim-start**: Starting template for a CIM

### 🤖 AI & Agent Systems
- **alchemist**: Information Alchemy - The core AI reasoning system
- **cim-agent-alchemist**: Agent for conversing about Alchemist
- **cim-domain-agent**: Agent domain for the Composable Information Machine
- **cim-domain-dialog**: CIM Dialog system to manage conversations

### 📊 Knowledge & Conceptual Systems
- **cim-conceptual-core**: Conceptual space core functionality and category theory implementations
- **cim-domain-conceptualspaces**: Conceptual Spaces Domain
- **cim-conceptgraph**: A Graphic way to work with ConceptualSpaces
- **cim-contextgraph**: Base Context Graph module for a CIM

### 🏢 Business Domains

```mermaid
graph TB
    subgraph "Business Domain Relationships"
        A[cim-domain] -->|Base for| B[cim-domain-person]
        A -->|Base for| C[cim-domain-organization]
        A -->|Base for| D[cim-domain-document]
        A -->|Base for| E[cim-domain-location]
        A -->|Base for| F[cim-domain-identity]
        
        B <-->|Related to| C
        F -->|Manages| B
        F -->|Manages| C
        
        G[cim-identity-context] -->|Bounded Context| F
        G -->|Contains| B
        G -->|Contains| C
        
        C -->|Located at| E
        B -->|Located at| E
        C -->|Creates| D
        B -->|Creates| D
        
        style A fill:#f9f,stroke:#333,stroke-width:4px
        style G fill:#9ff,stroke:#333,stroke-width:2px
    end
```

- **cim-domain**: Domain Generics and traits for a CIM
- **cim-domain-identity**: Identity Domain
- **cim-domain-organization**: Organization domain for the Composable Information Machine
- **cim-domain-person**: Person/People domain for the Composable Information Machine
- **cim-identity-context**: Identity bounded context for person and organization management
- **cim-domain-location**: Location Domain
- **cim-domain-document**: Document domain for the Composable Information Machine

### 🔧 Technical Domains
- **cim-domain-git**: Git Domain for CIM
- **cim-domain-nix**: CIM Domain Aggregates for Nix
- **cim-domain-workflow**: Workflow domain for the Composable Information Machine
- **cim-workflow-graph**: CIM Composition of Workflow and Graphs
- **cim-domain-graph**: CIM Specific Graphs

### 🔐 Security & Policy
- **cim-security**: CIM Security Domain
- **cim-domain-policy**: Policy domain for the Composable Information Machine
- **cim-keys**: Offline Key Management for CIM

### 📡 Data & Storage
- **cim-ipld**: CIM library for IPLD (InterPlanetary Linked Data)
- **cim-ipld-graph**: Graph Composition with IPLDs
- **cim-subject**: Subject management and routing utilities

### 🎮 Visualization & UI
- **cim-domain-bevy**: Bevy Domain Specific module
- **cim-viz-bevy**: Bevy-based visualization components for CIM

### 🌿 Edge Computing & Devices
- **cim-leaf**: CIM Leaf Node
- **cim-leaf-darwin**: CIM Leaf node for Darwin/macOS
- **cim-portal**: A portal module
- **cim-stb**: CIM connected Set Top Box

## AI Interaction Module
For AI interaction and questions about CIM, use:
- **cim-agent-alchemist**: This is the existing module that enables conversation with AI systems (like Claude) about the CIM architecture and capabilities.

## Module Integration Pattern

```mermaid
graph TD
    subgraph "Module Architecture"
        A[Domain Module] -->|Implements| B[Domain-Driven Design]
        A -->|Communicates via| C[NATS Messaging]
        A -->|Persists with| D[IPLD Storage]
        A -->|Emits| E[Domain Events]
        A -->|Supports| F[WASM Extensions]
        
        B --> G[Bounded Contexts]
        C --> H[Pub/Sub Patterns]
        D --> I[Content Addressing]
        E --> J[Event Sourcing]
        F --> K[Plugin System]
        
        style A fill:#f9f,stroke:#333,stroke-width:4px
        style C fill:#bbf,stroke:#333,stroke-width:2px
        style D fill:#bfb,stroke:#333,stroke-width:2px
    end
```

All modules follow a consistent pattern:
1. Domain-driven design principles
2. NATS-based messaging for communication
3. IPLD for data persistence and linking
4. Event-sourced architecture
5. WASM module support for extensibility