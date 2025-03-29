# Information Domain Vocabulary

## Term: Information Entity
- **Category**: Domain Object
- **Type**: Entity
- **Taxonomy**: Storage Taxonomy
- **Definition**: A discrete unit of information that can be stored, retrieved, and processed within the CIM system
- **Relationships**:
  * Contains: Information Attributes
  * Is-A: Composable Unit
  * Manages: Information State
- **Usage Context**: Core building block for representing any piece of information in the system
- **Code Reference**: TBD

## Term: Information State
- **Category**: Technical Concept
- **Type**: Value Object
- **Taxonomy**: Storage Taxonomy
- **Definition**: The current representation and status of an Information Entity within the system
- **Relationships**:
  * Part-Of: Information Entity
  * Validates: Information Integrity
  * Triggers: State Changes
- **Usage Context**: Used to track and manage the lifecycle of information within the system
- **Code Reference**: TBD

## Term: Composable Unit
- **Category**: Domain Object
- **Type**: Aggregate
- **Taxonomy**: Storage Taxonomy
- **Definition**: A self-contained piece of information that can be combined with other units to form larger information structures
- **Relationships**:
  * Contains: Information Entities
  * Extends: Information Capabilities
  * Configures: Unit Composition
- **Usage Context**: Fundamental building block for creating complex information structures
- **Code Reference**: TBD

## Term: Information Storage
- **Category**: Technical Concept
- **Type**: Value Object
- **Taxonomy**: Storage Taxonomy
- **Definition**: The physical or logical location where information is persistently stored
- **Relationships**:
  * Part-Of: Persistence Layer
  * Contains: Information Entities
  * Configures: Storage Layout
- **Usage Context**: Represents the actual storage mechanisms used by the system
- **Code Reference**: TBD 