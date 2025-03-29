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