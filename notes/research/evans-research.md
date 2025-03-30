# Eric Evans' Research on Messaging Patterns

## Core Messaging Concepts in DDD

### 1. Domain Events
- **Definition**: A record of something significant that happened in the domain
- **Characteristics**:
  - Named in past tense (e.g., OrderPlaced, PaymentProcessed)
  - Immutable once created
  - Contains relevant state at the time of occurrence
  - Used for communication between bounded contexts

### 2. Command Pattern
- **Purpose**: Represents a request to perform an action
- **Characteristics**:
  - Named in imperative mood (e.g., PlaceOrder, ProcessPayment)
  - Contains all necessary data to execute the action
  - May be rejected if business rules are violated
  - Often results in domain events

### 3. Query Pattern
- **Purpose**: Represents a request for information
- **Characteristics**:
  - Named as questions (e.g., GetOrderStatus, FindCustomerOrders)
  - Does not modify domain state
  - Returns data in a format suitable for the requesting context
  - May be optimized for specific use cases

## Integration with Bounded Contexts

### 1. Context Mapping
- **Shared Kernel**: Common code shared between contexts
- **Customer-Supplier**: One context depends on another
- **Conformist**: One context adapts to another's model
- **Anti-corruption Layer**: Translates between contexts
- **Open Host Service**: Published language for integration
- **Published Language**: Common language for communication

### 2. Message Translation
- **Purpose**: Maintain model integrity across contexts
- **Approaches**:
  - Direct translation of domain events
  - Transformation of commands
  - Adaptation of queries
  - Anti-corruption layer implementation

## Event Sourcing

### 1. Core Principles
- Store all changes as a sequence of events
- Rebuild current state by replaying events
- Maintain event history for audit and replay
- Support multiple views of the same data

### 2. Implementation Patterns
- Event store for persistence
- Event replay for state reconstruction
- Snapshot mechanism for performance
- Event versioning for evolution

## References
1. Evans, Eric. "Domain-Driven Design: Tackling Complexity in the Heart of Software"
2. Evans, Eric. "Domain-Driven Design Reference"
3. Vernon, Vaughn. "Implementing Domain-Driven Design" 