### Term: Actor
- **Category**: Domain Object
- **Type**: Entity
- **Taxonomy**: Runtime Components
- **Definition**: A lightweight, stateless wasmCloud component that encapsulates business logic and can be dynamically deployed across a distributed lattice.
- **Relationships**:
  * Part-Of: wasmCloud
  * Uses: Capability Provider
  * Deployed-In: Lattice
- **Usage Context**: Distributed application development
- **Code Reference**: `cim/src/actor`

### Term: Capability Provider
- **Category**: Domain Object
- **Type**: Service
- **Taxonomy**: Runtime Components
- **Definition**: A wasmCloud component that provides specific functionality (e.g., storage, messaging) to actors through standardized interfaces.
- **Relationships**:
  * Part-Of: wasmCloud
  * Serves: Actor
  * Implements: System Capabilities
- **Usage Context**: System capability implementation
- **Code Reference**: `cim/src/provider`

### Term: Lattice
- **Category**: Domain Object
- **Type**: Aggregate
- **Taxonomy**: Runtime Components
- **Definition**: A distributed runtime environment in wasmCloud where actors and capability providers can be dynamically deployed and scaled.
- **Relationships**:
  * Contains: Actor, Capability Provider
  * Part-Of: wasmCloud
  * Enables: Distributed Deployment
- **Usage Context**: Application deployment and scaling
- **Code Reference**: `cim/src/lattice`

### Term: Entity
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: ECS Components
- **Definition**: A unique identifier that represents a distinct object or concept in the ECS architecture, serving as a container for components.
- **Relationships**:
  * Contains: Component
  * Part-Of: ECS
  * Processed-By: System
- **Usage Context**: Data representation in ECS
- **Code Reference**: `cim/src/ecs/entity`

### Term: Component
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: ECS Components
- **Definition**: A data structure that defines properties or attributes that can be attached to entities in the ECS architecture.
- **Relationships**:
  * Attached-To: Entity
  * Part-Of: ECS
  * Processed-By: System
- **Usage Context**: Property definition in ECS
- **Code Reference**: `cim/src/ecs/component`

### Term: System
- **Category**: Domain Object
- **Type**: Service
- **Taxonomy**: ECS Components
- **Definition**: A logic unit in the ECS architecture that processes entities with specific component combinations to implement behavior.
- **Relationships**:
  * Processes: Entity, Component
  * Part-Of: ECS
  * Implements: Business Logic
- **Usage Context**: Logic implementation in ECS
- **Code Reference**: `cim/src/ecs/system` 