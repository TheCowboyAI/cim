# Communications Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Message:Entity {
  domain: "Communications",
  term: "Message",
  definition: "A fundamental unit of communication in CIM containing an ID, metadata, and domain content (Event, Command, or Query).",
  taxonomy: "Communication",
  usage_context: "System-wide communication and event handling",
  code_reference: "cim/src/comms/message"
})

(:DomainContent:ValueObject {
  domain: "Communications",
  term: "DomainContent",
  definition: "The payload of a message, representing either an Event, Command, or Query in the domain.",
  taxonomy: "Communication",
  usage_context: "Message content representation",
  code_reference: "cim/src/comms/content"
})

(:Subject:ValueObject {
  domain: "Communications",
  term: "Subject",
  definition: "A dot-delimited string that defines the routing and categorization of messages in the system.",
  taxonomy: "Communication",
  usage_context: "Message routing and categorization",
  code_reference: "cim/src/comms/subject"
})

(:EventStore:Service {
  domain: "Communications",
  term: "EventStore",
  definition: "A persistent storage system that maintains an immutable log of all domain events.",
  taxonomy: "Storage",
  usage_context: "Event persistence and replay",
  code_reference: "cim/src/storage/event_store"
})

(:ObjectStore:Service {
  domain: "Communications",
  term: "ObjectStore",
  definition: "A content-addressed storage system for large objects and files with CID-based retrieval.",
  taxonomy: "Storage",
  usage_context: "Large content storage and retrieval",
  code_reference: "cim/src/storage/object_store"
})

(:ContentAddress:ValueObject {
  domain: "Communications",
  term: "ContentAddress",
  definition: "A unique identifier for content in the Object Store, based on the content's hash and metadata.",
  taxonomy: "Identification",
  usage_context: "Content identification and retrieval",
  code_reference: "cim/src/storage/cid"
})

(:Comms:Service {
  domain: "Communications",
  term: "Comms",
  definition: "The core communication system of CIM that handles message routing, transformation, and delivery using NATS.",
  taxonomy: "Communication",
  usage_context: "System-wide communication management",
  code_reference: "cim/src/comms"
})

(:Benthos:Tool {
  domain: "Communications",
  term: "Benthos",
  definition: "A stream processing tool used in CIM for transforming data streams and message formats.",
  taxonomy: "Data Processing",
  usage_context: "Stream processing and data transformation",
  code_reference: "cim/src/benthos"
})

(:MessageHandling:Service {
  domain: "Communications",
  term: "MessageHandling",
  definition: "A system component responsible for processing, routing, and managing messages throughout the CIM ecosystem.",
  taxonomy: "Communication",
  usage_context: "Core message processing and handling",
  code_reference: "cim/src/comms/handling"
})

(:SubjectBasedRouting:Service {
  domain: "Communications",
  term: "SubjectBasedRouting",
  definition: "A routing mechanism that uses hierarchical subject strings to determine message destinations and processing rules.",
  taxonomy: "Communication",
  usage_context: "Message routing and distribution",
  code_reference: "cim/src/comms/routing"
})

(:EventObjectStorage:Service {
  domain: "Communications",
  term: "EventObjectStorage",
  definition: "A combined storage system that manages both events and content objects with consistent addressing and retrieval.",
  taxonomy: "Storage",
  usage_context: "Unified storage management",
  code_reference: "cim/src/storage/unified"
})
```

## Relationships

```cypher
// Message relationships
(:Message)-[:CONTAINS {type: "content"}]->(:DomainContent)
(:Message)-[:CONTAINS {type: "metadata"}]->(:Metadata)
(:Message)-[:USED_BY {type: "system"}]->(:Comms)
(:Message)-[:IMPLEMENTS {type: "protocol"}]->(:CommunicationProtocol)

// DomainContent relationships
(:DomainContent)-[:PART_OF {type: "message"}]->(:Message)
(:DomainContent)-[:CONTAINS {type: "data"}]->(:DomainSpecificData)
(:DomainContent)-[:IMPLEMENTS {type: "contract"}]->(:DomainContract)

// Subject relationships
(:Subject)-[:USED_BY {type: "messaging"}]->(:NATS)
(:Subject)-[:DEFINES {type: "routing"}]->(:MessageRouting)
(:Subject)-[:PART_OF {type: "model"}]->(:DomainModel)

// EventStore relationships
(:EventStore)-[:CONTAINS {type: "log"}]->(:Event)
(:EventStore)-[:IMPLEMENTS {type: "pattern"}]->(:EventSourcing)
(:EventStore)-[:USED_BY {type: "model"}]->(:DomainModel)

// ObjectStore relationships
(:ObjectStore)-[:CONTAINS {type: "storage"}]->(:ContentObject)
(:ObjectStore)-[:USES {type: "addressing"}]->(:ContentAddressing)
(:ObjectStore)-[:IMPLEMENTS {type: "storage"}]->(:FileStorage)

// ContentAddress relationships
(:ContentAddress)-[:IDENTIFIES {type: "content"}]->(:ContentObject)
(:ContentAddress)-[:USED_BY {type: "storage"}]->(:ObjectStore)
(:ContentAddress)-[:IMPLEMENTS {type: "compatibility"}]->(:IPFSCompatibility)

// Comms relationships
(:Comms)-[:USES {type: "messaging"}]->(:NATS)
(:Comms)-[:USES {type: "processing"}]->(:Benthos)
(:Comms)-[:MANAGES {type: "communication"}]->(:Message)
(:Comms)-[:IMPLEMENTS {type: "protocol"}]->(:CommunicationProtocol)

// Benthos relationships
(:Benthos)-[:USED_BY {type: "system"}]->(:Comms)
(:Benthos)-[:PROCESSES {type: "data"}]->(:MessageStream)
(:Benthos)-[:IMPLEMENTS {type: "transformation"}]->(:DataTransformation)

// MessageHandling relationships
(:MessageHandling)-[:USES {type: "message"}]->(:Message)
(:MessageHandling)-[:USES {type: "content"}]->(:DomainContent)
(:MessageHandling)-[:IMPLEMENTS {type: "pipeline"}]->(:MessageProcessingPipeline)
(:MessageHandling)-[:MANAGES {type: "lifecycle"}]->(:MessageLifecycle)

// SubjectBasedRouting relationships
(:SubjectBasedRouting)-[:USES {type: "addressing"}]->(:Subject)
(:SubjectBasedRouting)-[:IMPLEMENTS {type: "routing"}]->(:MessageRouting)
(:SubjectBasedRouting)-[:PART_OF {type: "system"}]->(:Comms)

// EventObjectStorage relationships
(:EventObjectStorage)-[:CONTAINS {type: "event_storage"}]->(:EventStore)
(:EventObjectStorage)-[:CONTAINS {type: "object_storage"}]->(:ObjectStore)
(:EventObjectStorage)-[:IMPLEMENTS {type: "interface"}]->(:UnifiedStorageInterface)
(:EventObjectStorage)-[:USES {type: "addressing"}]->(:ContentAddress)
```

## Taxonomies

### Communication Taxonomy

```cypher
(:Taxonomy {name: "CommunicationTaxonomy"})
-[:CONTAINS]->(:Category {name: "MessageTypes"})
-[:CONTAINS]->(:Operation {name: "MessageRouting"})
-[:CONTAINS]->(:Operation {name: "MessageTransformation"})
-[:CONTAINS]->(:Operation {name: "MessageDelivery"})

(:Category {name: "ProtocolTypes"})
-[:CONTAINS]->(:Operation {name: "SubjectBasedRouting"})
-[:CONTAINS]->(:Operation {name: "EventSourcing"})
-[:CONTAINS]->(:Operation {name: "ContentAddressing"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "SystemCommunication"})
-[:APPLIES_TO]->(:Message)
-[:REQUIRES]->(:Comms)
-[:PRODUCES]->(:DomainEvent)

(:UsageContext {name: "ContentStorage"})
-[:APPLIES_TO]->(:ObjectStore)
-[:REQUIRES]->(:ContentAddress)
-[:PRODUCES]->(:StoredContent)

(:UsageContext {name: "EventProcessing"})
-[:APPLIES_TO]->(:EventStore)
-[:REQUIRES]->(:DomainEvent)
-[:PRODUCES]->(:ProcessedEvent)
```

## Code References

```cypher
(:CodeBase {path: "cim/src/comms/message"})
-[:IMPLEMENTS]->(:Message)

(:CodeBase {path: "cim/src/comms/content"})
-[:IMPLEMENTS]->(:DomainContent)

(:CodeBase {path: "cim/src/comms/subject"})
-[:IMPLEMENTS]->(:Subject)

(:CodeBase {path: "cim/src/storage/event_store"})
-[:IMPLEMENTS]->(:EventStore)

(:CodeBase {path: "cim/src/storage/object_store"})
-[:IMPLEMENTS]->(:ObjectStore)

(:CodeBase {path: "cim/src/storage/cid"})
-[:IMPLEMENTS]->(:ContentAddress)

(:CodeBase {path: "cim/src/comms"})
-[:IMPLEMENTS]->(:Comms)

(:CodeBase {path: "cim/src/benthos"})
-[:IMPLEMENTS]->(:Benthos)

(:CodeBase {path: "cim/src/comms/handling"})
-[:IMPLEMENTS]->(:MessageHandling)

(:CodeBase {path: "cim/src/comms/routing"})
-[:IMPLEMENTS]->(:SubjectBasedRouting)

(:CodeBase {path: "cim/src/storage/unified"})
-[:IMPLEMENTS]->(:EventObjectStorage)
```