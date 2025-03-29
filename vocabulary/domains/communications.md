### Term: Message
- **Category**: Domain Object
- **Type**: Entity
- **Taxonomy**: Communication
- **Definition**: A fundamental unit of communication in CIM containing an ID, metadata, and domain content (Event, Command, or Query).
- **Relationships**:
  * Contains: DomainContent, Metadata
  * Used-By: Comms
  * Implements: Communication Protocol
- **Usage Context**: System-wide communication and event handling
- **Code Reference**: `cim/src/comms/message`

### Term: DomainContent
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Communication
- **Definition**: The payload of a message, representing either an Event, Command, or Query in the domain.
- **Relationships**:
  * Part-Of: Message
  * Contains: Domain-specific data
  * Implements: Domain Contracts
- **Usage Context**: Message content representation
- **Code Reference**: `cim/src/comms/content`

### Term: Subject
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Communication
- **Definition**: A dot-delimited string that defines the routing and categorization of messages in the system.
- **Relationships**:
  * Used-By: NATS
  * Defines: Message Routing
  * Part-Of: Domain Model
- **Usage Context**: Message routing and categorization
- **Code Reference**: `cim/src/comms/subject`

### Term: Event Store
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Storage
- **Definition**: A persistent storage system that maintains an immutable log of all domain events.
- **Relationships**:
  * Contains: Events
  * Implements: Event Sourcing
  * Used-By: Domain Model
- **Usage Context**: Event persistence and replay
- **Code Reference**: `cim/src/storage/event_store`

### Term: Object Store
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Storage
- **Definition**: A content-addressed storage system for large objects and files with CID-based retrieval.
- **Relationships**:
  * Contains: Content Objects
  * Uses: Content Addressing
  * Implements: File Storage
- **Usage Context**: Large content storage and retrieval
- **Code Reference**: `cim/src/storage/object_store`

### Term: Content Address (CID)
- **Category**: Domain Object
- **Type**: Value Object
- **Taxonomy**: Identification
- **Definition**: A unique identifier for content in the Object Store, based on the content's hash and metadata.
- **Relationships**:
  * Identifies: Content Objects
  * Used-By: Object Store
  * Implements: IPFS Compatibility
- **Usage Context**: Content identification and retrieval
- **Code Reference**: `cim/src/storage/cid`

### Term: Comms
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Communication
- **Definition**: The core communication system of CIM that handles message routing, transformation, and delivery using NATS.
- **Relationships**:
  * Uses: NATS, Benthos
  * Manages: Messages
  * Implements: Communication Protocol
- **Usage Context**: System-wide communication management
- **Code Reference**: `cim/src/comms`

### Term: Benthos
- **Category**: Technical Concept
- **Type**: Tool
- **Taxonomy**: Data Processing
- **Definition**: A stream processing tool used in CIM for transforming data streams and message formats.
- **Relationships**:
  * Used-By: Comms
  * Processes: Message Streams
  * Implements: Data Transformation
- **Usage Context**: Stream processing and data transformation
- **Code Reference**: `cim/src/benthos` 