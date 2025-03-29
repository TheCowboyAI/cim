### Term: Conceptual Space
- **Category**: Technical Concept
- **Type**: Pattern
- **Taxonomy**: AI Architecture
- **Definition**: A geometric framework based on Peter Gärdenfors' theory for representing concepts and their relationships in a multidimensional space.
- **Relationships**:
  * Contains: Quality Dimensions, Regions
  * Used-By: AI Agents
  * Enables: Semantic Understanding
- **Usage Context**: Knowledge representation and reasoning
- **Code Reference**: `cim/src/ai/conceptual_space`

### Term: Quality Dimension
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: AI Architecture
- **Definition**: A measurable attribute or feature that forms an axis in a conceptual space, representing a specific aspect of concepts.
- **Relationships**:
  * Part-Of: Conceptual Space
  * Defines: Measurement Scale
  * Used-By: AI Analysis
- **Usage Context**: Concept measurement and comparison
- **Code Reference**: `cim/src/ai/dimension`

### Term: Semantic Trajectory
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: AI Processing
- **Definition**: A path through conceptual space representing the evolution of meaning or state in a reasoning process.
- **Relationships**:
  * Part-Of: Conceptual Space
  * Represents: State Changes
  * Used-By: AI Reasoning
- **Usage Context**: AI reasoning and decision paths
- **Code Reference**: `cim/src/ai/trajectory`

### Term: Cooperative Game Theory
- **Category**: Technical Concept
- **Type**: Pattern
- **Taxonomy**: Decision Making
- **Definition**: A branch of game theory focusing on how groups of entities can work together to achieve mutual benefits.
- **Relationships**:
  * Part-Of: Game Theory
  * Uses: Coalition Formation
  * Optimizes: Resource Sharing
- **Usage Context**: Resource allocation and collaboration
- **Code Reference**: `cim/src/gametheory/cooperative`

### Term: Non-Cooperative Game Theory
- **Category**: Technical Concept
- **Type**: Pattern
- **Taxonomy**: Decision Making
- **Definition**: A branch of game theory analyzing strategic decision-making where entities act independently to maximize their own benefits.
- **Relationships**:
  * Part-Of: Game Theory
  * Uses: Nash Equilibrium
  * Optimizes: Individual Strategy
- **Usage Context**: Competitive strategy and conflict resolution
- **Code Reference**: `cim/src/gametheory/noncooperative`

### Term: Coalition
- **Category**: Domain Object
- **Type**: Entity
- **Taxonomy**: Game Theory
- **Definition**: A group of entities working together in a cooperative game to achieve shared objectives.
- **Relationships**:
  * Part-Of: Cooperative Game
  * Contains: Members
  * Shares: Resources
- **Usage Context**: Resource sharing and collaborative decision-making
- **Code Reference**: `cim/src/gametheory/coalition`

### Term: Utility Function
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Game Theory
- **Definition**: A mathematical function that measures the satisfaction or benefit an entity derives from a particular outcome.
- **Relationships**:
  * Used-By: Game Theory
  * Measures: Outcomes
  * Guides: Decision Making
- **Usage Context**: Decision evaluation and optimization
- **Code Reference**: `cim/src/gametheory/utility`

### Term: Strategy Component
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Game Theory
- **Definition**: A component that encodes decision-making strategies based on game-theoretic principles.
- **Relationships**:
  * Part-Of: Entity
  * Uses: Game Theory
  * Implements: Decision Logic
- **Usage Context**: Strategic decision implementation
- **Code Reference**: `cim/src/gametheory/strategy`

### Term: Vector Embedding
- **Category**: Technical Concept
- **Type**: Value Object
- **Taxonomy**: AI Processing
- **Definition**: A high-dimensional vector representation of concepts or data points in a semantic space.
- **Relationships**:
  * Used-By: AI Models
  * Represents: Semantic Content
  * Enables: Similarity Comparison
- **Usage Context**: Semantic analysis and retrieval
- **Code Reference**: `cim/src/ai/embedding`

### Term: AIAgent
- **Category**: Domain Object
- **Type**: Entity
- **Taxonomy**: AI Architecture
- **Definition**: An autonomous software entity capable of perceiving its environment, making decisions, and taking actions to achieve specific goals.
- **Relationships**:
  * Uses: Conceptual Space, Strategy Component
  * Implements: Decision Making
  * Contains: AI Components
- **Usage Context**: Autonomous decision-making and task execution
- **Code Reference**: `cim/src/ai/agent`

### Term: DecisionStrategy
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Decision Making
- **Definition**: A service that implements decision-making algorithms combining game theory, conceptual spaces, and utility optimization.
- **Relationships**:
  * Uses: Utility Function, Game Theory
  * Implements: Strategic Planning
  * Produces: Decisions
- **Usage Context**: Strategic decision-making and planning
- **Code Reference**: `cim/src/ai/strategy`

### Term: GameTheoryEngine
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Game Theory
- **Definition**: A computational engine that applies game theory principles to optimize multi-agent interactions and resource allocation.
- **Relationships**:
  * Uses: Cooperative Game Theory, Non-Cooperative Game Theory
  * Implements: Game Analysis
  * Produces: Strategic Solutions
- **Usage Context**: Game-theoretic analysis and optimization
- **Code Reference**: `cim/src/gametheory/engine` 