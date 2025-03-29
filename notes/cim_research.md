# Composable Information Machines (CIM) Research

## Core Vision
- A continuously improving, distributed system that transforms scattered information into organized, actionable, reactive knowledge
- Maintains security and accessibility across digital environments
- Moves beyond paper-centric information management to modern digital paradigms
- Adapts to industry-specific thinking and workflows rather than forcing rigid structures
- Combines AI, secure infrastructure, and mathematical precision for domain understanding

### Goals
- Create portable, secure information systems usable across devices
- Build self-evolving systems that grow over time
- Scale to any business size
- Integrate AI tools for easier information creation, sharing, and use
- Represent digital content beyond paper (screens, telemetry, sensors, storage)

### Scope Boundaries
Focuses on integration and enhancement rather than replacement:
- Uses existing message transport systems
- Leverages existing data storage solutions
- Utilizes existing query and alerting systems
- Builds on existing capabilities

## Core Concepts

### CIM Architecture
- Based on Domain-Driven Design (DDD) and Event-Driven Architecture
- Emphasizes "bottom-up" approach starting from single packages
- Uses Nix Flakes for reproducibility and organization
- Supports multiple development states: dev, test, prod
- Focuses on modular construction and minimal environment requirements
- Security Features:
  - mTLS authentication
  - YubiKey integration
  - OpenPGP support
  - OpenSSL encryption
- Infrastructure Components:
  - NixOS containers (systemd-nspawn)
  - NVIDIA GPU sharing
  - Wayland + Hyprland
  - S3 compatible storage (Minio/Wasabi)

### Information Architecture
- Object Storage Foundation
  - Content-addressed storage for immutable data
  - Two primary "buckets": cim-objects and cim-events
  - Content-Address system for data identification and verification
  - Immutable storage with version tracking through graphs

- Event-Driven State Management
  - Results and State Change Deltas in Object Store
  - Event Store for tracking changes
  - Immutable history with modification tracking
  - Graph-based relationship tracking

### Research and Knowledge Management

#### Trails System
- AI Research Assistant capabilities
- Features:
  - Integration with Cursor.sh's AI capabilities
  - Automated research document generation
  - Vocabulary and knowledge graph management
  - Support for public (arXiv) and private (MCP) data sources
- Research Workflow:
  1. Research Initiation (scope, vocabulary, concepts)
  2. Data Collection (AI-assisted, multiple sources)
  3. Analysis & Documentation
  4. Review & Iteration

#### Cursor Integration
- Custom rules for AI code generation
- MCP (Model Context Protocol) support
- Project-specific instructions
- Integration with development workflow
- Local-first architecture with git-based tracking

### Model Context Protocol (MCP)
- Structured approach for AI systems to communicate with data sources and tools
- Key Components:
  - MCP host: Programs like Claude Desktop, IDEs, or AI tools
  - MCP clients: Protocol clients with 1:1 server connections
  - MCP servers: Lightweight programs exposing standardized capabilities
  - Local/Remote resources: Computer or internet-accessible resources
- Implementation in CIM:
  - NixMCP for package and system information
  - Integration with development environments
  - Support for multiple server implementations
  - Flake-based configuration and deployment

### Package Capabilities
- Build functionality (`nix build`)
  - Package building with Cargo.toml integration
  - Extensible buildPackage sequence
  - CI/CD pipeline adaptation
  - Event emission for build status
  
- Runtime functionality (`nix run`)
  - Package execution
  - Package and service addition
  - Module integration
  - Event handling
  - Container support
  - Web interface capabilities
  - NATS integration (local and distributed)
  - Subject-based communication
  - Monitoring and control features

## Modular Architecture

### AI Module (module.ai.cim)
- Installable as a NixOS Module
- Provides local LLM capabilities through Ollama integration
- Supports multiple models (Mistral, Llama2, CodeLlama, etc.)
- Features:
  - Model management (pull, push, copy, delete)
  - Chat functionality with streaming responses
  - Embedding generation and management
  - Object storage for files and images
  - Settings management
  - Event-driven architecture with NATS integration

### Communications Module (module.comms.cim)  
- Manages internal and external communications
- Features:
  - Matrix for external messaging
  - NATS for internal messaging
  - Bridge capabilities to other platforms (Telegram, WhatsApp, Discord, IRC)
  - Decentralized chat rooms
  - End-to-end encryption
  - User management and authentication
  - Profile management
  - Federation support

### Other Modules
- module.apis.cim: API management
- module.db.cim: Database services
- module.docs.cim: Documentation management
- module.feeds.cim: Feed processing
- module.git.cim: Git integration
- module.mail.cim: Email services
    - module.ns.cim: Namespace services
- module.notes.cim: Note management
- module.policy.cim: Policy enforcement
- module.search.cim: Search capabilities
- module.vault.cim: Secure storage
- module.web.cim: Web interface
- module.wf.cim: Workflow management

## Domain Structure

### Domain Definition
A CIM operates on Domains, which represent unique ideas and concepts that cannot be further reduced. Domains can utilize elements from other domains while maintaining their distinct characteristics.

### Core Domain Categories
1. Knowledge Management
   - Facts: Proven claims
   - Claims: Ideas with repeatable construction
   - Theories: Beliefs with context and sources
   - Ideas: Preliminary thoughts without formal theory
   - Arguments: Claim support/opposition

2. Organizational
   - Goals: Achievement targets
   - Organizations: Structural entities
   - People: Human resources
   - Operators: System controllers
   - Accounts: Group management

3. Business and Value
   - Business Model: Operational framework
   - Value Propositions: Benefit offerings
   - Solutions: Problem resolutions
   - Proposals: Formal suggestions
   - Funding: Financial resources

4. Governance
   - Policies: Operational guidelines
   - Laws: Regulatory framework
   - Ethics: Moral principles
   - Politics: Power dynamics
   - Relationships: Inter-entity connections

5. Technical and Environmental
   - Models: System representations
   - Equipment: Physical resources
   - Environment: Contextual settings
   - Location: Spatial information
   - Secrets: Protected information

### Domain Implementation
- Each domain category maintains its own readme.md
- Domains are rigid with meaningful categorization
- Information richness aids in operation patterns
- Future integration with Language Agents for Domain Expert AIs
- Contributes to Domain-specific language development

## Development Philosophy
- Modular and composable architecture
- Event-driven communication
- Domain-specific categorization
- Extensible and adaptable design
- Self-contained data management post-deployment
- Focus on enhancing existing tools rather than replacement
- Contribution to open source projects
- Emphasis on secure information sharing
- Maintenance of permanent, immutable system of record

## CIM Implementations

### Primary Implementation (Cowboy AI)
- Core reference implementation by CIM creators
- Defines standard architecture and patterns
- Provides foundational tooling and documentation
- Establishes best practices and patterns
- Drives core development and innovation
- Maintains primary documentation and specifications

### Set-Top Box (cim-stb)
- Custom set-top box implementation using NixOS on NanoPC T4 OR OTHER ARM devices
- Features:
  - Integration with streaming services (Plex, Netflix, Amazon Prime)
  - RSS feed control system
  - Video calling capabilities
  - Custom UI development options
  - Hardware optimization for NVMe storage
  - DRM support for commercial streaming

### Portal (cim-portal)
- Web interface implementation in Rust
- End-to-end testing support
- Public-facing interface components
- Style customization capabilities

### Customer Implementations
1. thecowboy.ai
   - Focus on compute and network infrastructure
   - Modular architecture with specialized components
   - Documentation for DevOps and K8s engineering
   - Implementation tooling specifications
   - Hardware configuration management

2. Keco Capital
   - Enterprise implementation of CIM
   - Custom domain management for financial services
   - Specialized security and compliance features
   - Integration with financial systems
   - Asset and portfolio management focus

3. sebastion.org
   - Virtual STB management
   - Domain-specific documentation
   - Compute resource organization
   - Flake-based configuration

### Research Materials (cim-thecowboyai)
- Comprehensive collection of foundational papers:
  - Domain-Driven Design (DDD) principles
  - Event Sourcing methodologies
  - Functional Reactive Programming (FRP)
  - Category theory applications
  - Push-pull FRP implementations
- Implementation components:
  - BMC (Board Management Controller)
  - Event and Object stores
  - Ontology management
  - Library system
  - Fabricator component

## Domain Objects

### Core Aggregates
- **CIM System**
  - Definition: A distributed system transforming information into organized, actionable knowledge
  - Relationships: Contains(Modules), Manages(Information), Implements(Domains)
  - Usage Context: Primary system aggregate

- **Information Unit**
  - Definition: Atomic piece of managed information with metadata
  - Relationships: Part-Of(CIM System), Contains(Content, Metadata)
  - Usage Context: Basic unit of information management

### Entities
- **Module**
  - Definition: Self-contained functional unit providing specific services
  - Types: AI, Communications, Documentation, etc.
  - Relationships: Part-Of(CIM System), Manages(Services)

- **Domain**
  - Definition: Unique set of ideas and concepts that cannot be further reduced
  - Relationships: Part-Of(CIM System), Contains(Facts, Claims, Theories)
  - Usage Context: Organizational unit for knowledge

### Value Objects
- **Event**
  - Definition: Immutable record of system state change
  - Relationships: Part-Of(Event Store), Triggers(State Changes)

- **Content Address**
  - Definition: Unique identifier based on content hash
  - Relationships: Identifies(Information Unit), Part-Of(Storage System)

## Technical Concepts

### Architecture Patterns
- **Event-Driven Architecture**
  - Definition: System design based on event production and consumption
  - Relationships: Implements(Event Store), Uses(NATS)

- **Domain-Driven Design**
  - Definition: Design approach matching software to domain expertise
  - Relationships: Shapes(System Design), Influences(Domain Structure)

### Protocols
- **Model Context Protocol (MCP)**
  - Definition: Protocol for AI systems to communicate with data sources
  - Components: Host, Clients, Servers, Resources
  - Relationships: Enables(AI Integration), Uses(Standardized Interfaces)

### Storage Taxonomy
- **Object Storage**
  - Primary Buckets: cim-objects, cim-events
  - Operations: Content-addressing, Version tracking
  - Relationships: Implements(Storage Operations), Manages(Data Integrity)

## Business Concepts

### Processes
- **Research Workflow**
  - Steps: Initiation, Collection, Analysis, Documentation, Review
  - Relationships: Uses(AI Assistant), Produces(Documentation)

### Implementation Types
- **Reference Implementation**
  - Owner: Cowboy AI
  - Purpose: Define standards and patterns
  - Relationships: Guides(Customer Implementations)

- **Customer Implementations**
  - Types: Enterprise, Specialized, Virtual
  - Examples: Keco Capital (Financial Services), sebastion.org (Virtual STB)
  - Relationships: Extends(Reference Implementation)

## Cross-Cutting Terms

### Security
- **Authentication Methods**
  - Types: mTLS, YubiKey, OpenPGP
  - Relationships: Protects(System Access), Implements(Security Policy)

### Configuration
- **Nix Flakes**
  - Purpose: Reproducible builds and organization
  - Relationships: Manages(Dependencies), Enables(Reproducibility)

### Monitoring
- **Event Store**
  - Purpose: Track system changes and state
  - Relationships: Implements(Monitoring), Maintains(History) 