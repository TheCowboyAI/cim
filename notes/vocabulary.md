# CIM Vocabulary

[← Back to Index](index.md)

## Core Concepts
*For detailed implementation, see [Architecture Overview](architecture.md)*

### Architecture Terms
- **CIM (Composable Information Machine)**: A framework for building distributed systems that transform scattered information into organized, actionable knowledge
- **Domain**: A unique set of ideas and concepts that cannot be further reduced, representing a specific area of knowledge or business function
- **Pod**: A self-contained functional unit within CIM that provides specific services (e.g., AI, communications, documentation)
- **Event**: A record of something that happened within the CIM system
- **Command**: An instruction that modifies CIM state
- **Message**: Any transmission within the CIM system
- **Observation**: A snapshot of state at a particular time

### Technical Terms
*For implementation details, see [Technical Infrastructure](technical.md)*
- **DDD (Domain-Driven Design)**: A software design approach focusing on modeling software to match a domain according to expert input
- **EDA (Event-Driven Architecture)**: An architectural pattern where components communicate through events
- **FRP (Functional Reactive Programming)**: Programming paradigm for reactive programming using functional programming concepts
- **MCP (Model Context Protocol)**: Protocol for AI systems to communicate with data sources and tools
- **NATS**: Message broker system used for internal CIM communication
- **Content-Addressing**: Method of storing and retrieving information based on its content rather than location

### Implementation Components
*For detailed examples, see [Implementation Guide](implementation.md)*
- **Event Store**: System for storing and managing event streams
- **Object Store**: System for storing immutable data with content addressing
- **BMC (Board Management Controller)**: Component for hardware management
- **Fabricator**: Component for system construction and deployment
- **Ontology**: System for managing relationships and classifications
- **Library**: Collection of reusable components and resources

## Domain Categories
*For full domain documentation, see [Domain Structure](domain_structure.md)*

### Knowledge Management
*Detailed in [Knowledge Management](knowledge_management.md)*
- **Fact**: A proven claim with verifiable evidence
- **Claim**: An idea with repeatable construction
- **Theory**: A belief with supporting context and sources
- **Idea**: A preliminary thought without formal theory
- **Argument**: Support or opposition for a claim

### Organizational
*Detailed in [Organizational Structure](organizational.md)*
- **Goal**: A defined achievement target
- **Organization**: A structural entity
- **Operator**: A system controller
- **Account**: A managed group or entity

### Business
*Detailed in [Business and Value](business.md)*
- **Business Model**: Operational framework for value creation
- **Value Proposition**: Benefit offering to stakeholders
- **Solution**: Resolution to a defined problem
- **Proposal**: Formal suggestion for action

### Governance
*Detailed in [Governance](governance.md)*
- **Policy**: Operational guideline
- **Law**: Regulatory framework
- **Ethics**: Moral principles
- **Politics**: Power dynamics and relationships

### Technical
*Detailed in [Technical Infrastructure](technical.md)*
- **Model**: System representation
- **Equipment**: Physical resource
- **Environment**: Contextual setting
- **Location**: Spatial information
- **Secret**: Protected information

## Implementation Types
*For specific implementations, see [Implementation Guide](implementation.md)*

### Infrastructure
*See specific implementations: [STB](stb_implementation.md), [Portal](portal_implementation.md)*
- **STB (Set-Top Box)**: Hardware implementation for media streaming and control
- **Portal**: Web-based interface implementation
- **Domain Implementation**: Specific instance of CIM for a particular organization or purpose

### Security
*Detailed in [Security Model](security.md)*
- **mTLS**: Mutual Transport Layer Security authentication
- **YubiKey**: Hardware authentication device
- **OpenPGP**: Encryption standard
- **OpenSSL**: Cryptographic software library

---
*For contribution guidelines, see [Contributing](index.md#contributing)* 