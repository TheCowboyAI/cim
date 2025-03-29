### Term: Domain
- **Category**: Domain Object
- **Type**: Aggregate
- **Taxonomy**: Core Concepts
- **Definition**: A bounded context representing a specific business area with its own ubiquitous language, rules, and processes.
- **Relationships**:
  * Contains: Entities, Values, Policies
  * Defines: Ubiquitous Language
  * Implements: Business Logic
- **Usage Context**: Business modeling and system organization
- **Code Reference**: `cim/src/domain`

### Term: Subdomain
- **Category**: Domain Object
- **Type**: Entity
- **Taxonomy**: Domain Organization
- **Definition**: A distinct part of the overall domain, categorized as Core, Supporting, or Generic based on business value.
- **Relationships**:
  * Part-Of: Domain
  * Contains: Domain Logic
  * Implements: Business Rules
- **Usage Context**: Domain partitioning and organization
- **Code Reference**: `cim/src/domain/subdomain`

### Term: Bounded Context
- **Category**: Domain Object
- **Type**: Aggregate
- **Taxonomy**: Domain Organization
- **Definition**: A logical boundary that separates different parts of the system where terms and concepts may have different meanings.
- **Relationships**:
  * Contains: Domain Model
  * Defines: Context Boundaries
  * Implements: Domain Separation
- **Usage Context**: Domain model organization and context separation
- **Code Reference**: `cim/src/domain/context`

### Term: Domain Model
- **Category**: Domain Object
- **Type**: Aggregate
- **Taxonomy**: Domain Organization
- **Definition**: A conceptual model representing the key concepts, relationships, and behaviors within a domain.
- **Relationships**:
  * Contains: Entities, Value Objects
  * Part-Of: Bounded Context
  * Implements: Business Rules
- **Usage Context**: Domain representation and business logic implementation
- **Code Reference**: `cim/src/domain/model`

### Term: Nix Flake
- **Category**: Technical Concept
- **Type**: Configuration
- **Taxonomy**: System Configuration
- **Definition**: A standardized format for packaging Nix expressions with dependencies and metadata for reproducible builds.
- **Relationships**:
  * Contains: Dependencies, Outputs
  * Implements: Configuration Management
  * Used-By: CIM
- **Usage Context**: System configuration and deployment
- **Code Reference**: `cim/src/config/flake`

### Term: Configuration
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: System Management
- **Definition**: A collection of settings and parameters that define how a system component or service should operate.
- **Relationships**:
  * Part-Of: Domain
  * Defines: System Behavior
  * Used-By: Components
- **Usage Context**: System setup and management
- **Code Reference**: `cim/src/config`

### Term: Domain Event
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Communication
- **Definition**: A record of something significant that happened within the domain, stored immutably in the event store.
- **Relationships**:
  * Part-Of: Event Store
  * Records: State Changes
  * Used-By: Domain Model
- **Usage Context**: Event sourcing and state management
- **Code Reference**: `cim/src/domain/event`

### Term: Domain Command
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Communication
- **Definition**: A request to perform an action that will change the state of the domain.
- **Relationships**:
  * Produces: Domain Event
  * Changes: Domain State
  * Used-By: Domain Model
- **Usage Context**: State modification and business operations
- **Code Reference**: `cim/src/domain/command`

### Term: Domain Query
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Communication
- **Definition**: A request to retrieve information from the domain without modifying state.
- **Relationships**:
  * Retrieves: Domain State
  * Used-By: Domain Model
  * Produces: Projections
- **Usage Context**: State retrieval and data access
- **Code Reference**: `cim/src/domain/query`

### Term: DomainModeling
- **Category**: Technical Concept
- **Type**: Process
- **Taxonomy**: Domain Organization
- **Definition**: The process of identifying, analyzing, and structuring domain concepts and their relationships to create effective domain models.
- **Relationships**:
  * Produces: Domain Model
  * Uses: Bounded Context
  * Implements: Business Requirements
- **Usage Context**: Domain analysis and design
- **Code Reference**: `cim/src/domain/modeling`

### Term: ConfigurationManagement
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: System Management
- **Definition**: A system service responsible for managing, validating, and applying configuration changes across the system.
- **Relationships**:
  * Manages: Configuration
  * Uses: Nix Flake
  * Implements: System Setup
- **Usage Context**: System configuration and maintenance
- **Code Reference**: `cim/src/config/management`

### Term: DomainEventProcessing
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Communication
- **Definition**: A service responsible for processing and managing the lifecycle of domain events, commands, and queries.
- **Relationships**:
  * Processes: Domain Event, Domain Command, Domain Query
  * Implements: Event Processing Pipeline
  * Uses: Event Store
- **Usage Context**: Domain event handling and processing
- **Code Reference**: `cim/src/domain/processing` 