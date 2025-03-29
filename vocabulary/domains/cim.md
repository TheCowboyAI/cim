### Term: Composable Information Machine (CIM)
- **Category**: Domain Object
- **Type**: Aggregate
- **Taxonomy**: System Architecture
- **Definition**: A transformative framework that combines Domain-Driven Design, real-time conceptual spaces, AI-driven decision-making, and game theory optimization to create business systems that align with domain logic.
- **Relationships**:
  * Contains: Conceptual Space, Quality Dimension, Domain
  * Uses: Domain-Driven Design, Game Theory Optimization
  * Implements: Modular Architecture
- **Usage Context**: Core system architecture and business logic implementation
- **Code Reference**: `cim/src/core`

### Term: Conceptual Space
- **Category**: Domain Object
- **Type**: Entity
- **Taxonomy**: Knowledge Management
- **Definition**: A geometric framework for representing business domains in real-time, based on Peter Gärdenfors' theory, enabling analysis of relationships and trends.
- **Relationships**:
  * Contains: Quality Dimension, Region, Prototype
  * Part-Of: CIM
  * Manages: Domain Representation
- **Usage Context**: Data analysis, trend identification, and decision-making
- **Code Reference**: `cim/src/conceptual`

### Term: Quality Dimension
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Metrics
- **Definition**: A measurable attribute relevant to the business domain, used to define points in a conceptual space.
- **Relationships**:
  * Part-Of: Conceptual Space
  * Defines: Domain Metrics
  * Contains: Measurement Values
- **Usage Context**: Metric definition and measurement in conceptual spaces
- **Code Reference**: `cim/src/metrics`

### Term: Domain-Driven Design (DDD)
- **Category**: Technical Concept
- **Type**: Pattern
- **Taxonomy**: Architecture
- **Definition**: A software design approach that matches system structure and language with business domain logic.
- **Relationships**:
  * Contains: Bounded Context, Ubiquitous Language
  * Used-By: CIM
  * Manages: Domain Logic
- **Usage Context**: System design and domain modeling
- **Code Reference**: `cim/src/ddd`

### Term: Game Theory Optimization
- **Category**: Technical Concept
- **Type**: Pattern
- **Taxonomy**: Decision Making
- **Definition**: Mathematical modeling of strategic interactions for resource allocation and decision optimization.
- **Relationships**:
  * Contains: Cooperative Game Theory, Non-Cooperative Game Theory
  * Used-By: CIM
  * Optimizes: Resource Allocation
- **Usage Context**: Strategic decision-making and resource optimization
- **Code Reference**: `cim/src/gametheory`

### Term: Modular Architecture
- **Category**: Technical Concept
- **Type**: Pattern
- **Taxonomy**: System Design
- **Definition**: A system design approach using self-contained, reusable, and interoperable components.
- **Relationships**:
  * Implements: Component System
  * Used-By: CIM
  * Enables: Scalability
- **Usage Context**: System implementation and component design
- **Code Reference**: `cim/src/architecture` 