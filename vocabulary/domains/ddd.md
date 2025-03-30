# DDD Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:BoundedContext:Aggregate {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "A boundary within which a domain model is defined and applicable, with its own ubiquitous language"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})

(:DomainModel:Aggregate {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "A structured representation of domain knowledge and business rules"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})

(:UbiquitousLanguage:ValueObject {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "A shared language between domain experts and developers that evolves with the domain model"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})

(:AggregateRoot:Entity {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "The primary entity in an aggregate that ensures consistency and enforces invariants"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})

(:Entity:Entity {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "An object with a distinct identity that persists over time"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})

(:ValueObject:ValueObject {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "An immutable object that describes characteristics of a domain concept"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})

(:DomainEvent:Event {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "A record of something significant that happened in the domain"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})

(:DomainService:Service {
  domain: "DDD",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "Entity)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "AggregateRoot)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "DomainModel)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "BoundedContext)-[",
  term: "DomainService",
  term: "DomainEvent",
  term: "ValueObject",
  term: "Entity",
  term: "AggregateRoot",
  term: "UbiquitousLanguage",
  term: "DomainModel",
  term: "BoundedContext",
  definition: "An operation that doesn't naturally belong to any entity or value object"
  taxonomy: "DDD Taxonomy",
  usage_context: "DDD related operations and processes",
  code_reference: "TBD",
})
```

## Relationships

```cypher
// BoundedContext relationships
(:BoundedContext)-[:DEFINES {type: "boundary"}]->(:DomainModel)
(:BoundedContext)-[:USES {type: "language"}]->(:UbiquitousLanguage)
(:BoundedContext)-[:CONTAINS {type: "aggregate"}]->(:AggregateRoot)
(:BoundedContext)-[:PROVIDES {type: "service"}]->(:DomainService)

// DomainModel relationships
(:DomainModel)-[:IMPLEMENTS {type: "pattern"}]->(:Pattern)
(:DomainModel)-[:FOLLOWS {type: "principle"}]->(:DDDPrinciple)
(:DomainModel)-[:USES {type: "language"}]->(:UbiquitousLanguage)
(:DomainModel)-[:CONTAINS {type: "concept"}]->(:DomainConcept)

// AggregateRoot relationships
(:AggregateRoot)-[:ENFORCES {type: "invariant"}]->(:BusinessRule)
(:AggregateRoot)-[:CONTAINS {type: "boundary"}]->(:Entity)
(:AggregateRoot)-[:MANAGES {type: "lifecycle"}]->(:ValueObject)
(:AggregateRoot)-[:PUBLISHES {type: "event"}]->(:DomainEvent)

// Entity relationships
(:Entity)-[:HAS {type: "identity"}]->(:Identity)
(:Entity)-[:CONTAINS {type: "state"}]->(:ValueObject)
(:Entity)-[:FOLLOWS {type: "rule"}]->(:BusinessRule)
(:Entity)-[:RAISES {type: "event"}]->(:DomainEvent)
```

## Taxonomies

### DDD Processing

```cypher
  code_reference: "TBD",
(:Taxonomy {name: "DDDProcessing"})
  code_reference: "TBD",
-[:CONTAINS]->(:Category {name: "StrategicDesign"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "BoundedContextDefinition"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "UbiquitousLanguageEvolution"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ContextMapping"})

  code_reference: "TBD",
(:Category {name: "TacticalDesign"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "AggregateDesign"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "EntityModeling"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ValueObjectDefinition"})

  code_reference: "TBD",
(:Category {name: "PatternIntegration"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "CQRSImplementation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ECSIntegration"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "EDAImplementation"})
```

## Usage Contexts

```cypher
  code_reference: "TBD",
(:UsageContext {name: "DomainModeling"})
-[:APPLIES_TO]->(:DomainModel)
-[:REQUIRES]->(:UbiquitousLanguage)
-[:PRODUCES]->(:BoundedContext)

  code_reference: "TBD",
(:UsageContext {name: "AggregateDesign"})
-[:APPLIES_TO]->(:AggregateRoot)
-[:REQUIRES]->(:BusinessRule)
-[:PRODUCES]->(:ConsistencyBoundary)

  code_reference: "TBD",
(:UsageContext {name: "EventSourcing"})
-[:APPLIES_TO]->(:DomainEvent)
-[:REQUIRES]->(:EventStore)
-[:PRODUCES]->(:EventStream)
```

## Code References

```cypher
  code_reference: "TBD",
(:CodeBase {path: "notes/ddd/readme.md"})
-[:IMPLEMENTS]->(:DomainModel)

  code_reference: "TBD",
(:CodeBase {path: "notes/ddd/bounded_contexts.md"})
-[:IMPLEMENTS]->(:BoundedContext)

  code_reference: "TBD",
(:CodeBase {path: "notes/ddd/aggregates.md"})
-[:IMPLEMENTS]->(:AggregateRoot)

  code_reference: "TBD",
(:CodeBase {path: "notes/ddd/events.md"})
-[:IMPLEMENTS]->(:DomainEvent)
``` 