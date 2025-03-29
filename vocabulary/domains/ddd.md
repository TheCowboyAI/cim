# DDD Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:BoundedContext:Aggregate {
  domain: "DDD",
  definition: "A boundary within which a domain model is defined and applicable, with its own ubiquitous language"
})

(:DomainModel:Aggregate {
  domain: "DDD",
  definition: "A structured representation of domain knowledge and business rules"
})

(:UbiquitousLanguage:ValueObject {
  domain: "DDD",
  definition: "A shared language between domain experts and developers that evolves with the domain model"
})

(:AggregateRoot:Entity {
  domain: "DDD",
  definition: "The primary entity in an aggregate that ensures consistency and enforces invariants"
})

(:Entity:Entity {
  domain: "DDD",
  definition: "An object with a distinct identity that persists over time"
})

(:ValueObject:ValueObject {
  domain: "DDD",
  definition: "An immutable object that describes characteristics of a domain concept"
})

(:DomainEvent:Event {
  domain: "DDD",
  definition: "A record of something significant that happened in the domain"
})

(:DomainService:Service {
  domain: "DDD",
  definition: "An operation that doesn't naturally belong to any entity or value object"
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
(:Taxonomy {name: "DDDProcessing"})
-[:CONTAINS]->(:Category {name: "StrategicDesign"})
-[:CONTAINS]->(:Operation {name: "BoundedContextDefinition"})
-[:CONTAINS]->(:Operation {name: "UbiquitousLanguageEvolution"})
-[:CONTAINS]->(:Operation {name: "ContextMapping"})

(:Category {name: "TacticalDesign"})
-[:CONTAINS]->(:Operation {name: "AggregateDesign"})
-[:CONTAINS]->(:Operation {name: "EntityModeling"})
-[:CONTAINS]->(:Operation {name: "ValueObjectDefinition"})

(:Category {name: "PatternIntegration"})
-[:CONTAINS]->(:Operation {name: "CQRSImplementation"})
-[:CONTAINS]->(:Operation {name: "ECSIntegration"})
-[:CONTAINS]->(:Operation {name: "EDAImplementation"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "DomainModeling"})
-[:APPLIES_TO]->(:DomainModel)
-[:REQUIRES]->(:UbiquitousLanguage)
-[:PRODUCES]->(:BoundedContext)

(:UsageContext {name: "AggregateDesign"})
-[:APPLIES_TO]->(:AggregateRoot)
-[:REQUIRES]->(:BusinessRule)
-[:PRODUCES]->(:ConsistencyBoundary)

(:UsageContext {name: "EventSourcing"})
-[:APPLIES_TO]->(:DomainEvent)
-[:REQUIRES]->(:EventStore)
-[:PRODUCES]->(:EventStream)
```

## Code References

```cypher
(:CodeBase {path: "notes/ddd/readme.md"})
-[:IMPLEMENTS]->(:DomainModel)

(:CodeBase {path: "notes/ddd/bounded_contexts.md"})
-[:IMPLEMENTS]->(:BoundedContext)

(:CodeBase {path: "notes/ddd/aggregates.md"})
-[:IMPLEMENTS]->(:AggregateRoot)

(:CodeBase {path: "notes/ddd/events.md"})
-[:IMPLEMENTS]->(:DomainEvent)
``` 