# Business Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:BusinessModel:Aggregate {
  domain: "Business",
  definition: "A structured representation of how an organization creates, delivers, and captures value"
})

(:ValueProposition:ValueObject {
  domain: "Business",
  definition: "The unique value offered to customers that addresses their needs and differentiates from competitors"
})

(:CustomerSegment:Entity {
  domain: "Business",
  definition: "A distinct group of customers with common characteristics, needs, and behaviors"
})

(:Revenue:ValueObject {
  domain: "Business",
  definition: "Income streams generated from delivering value to customers"
})

(:Resource:Entity {
  domain: "Business",
  definition: "Key assets and capabilities required to deliver the value proposition"
})

(:Channel:Service {
  domain: "Business",
  definition: "Methods and platforms used to reach and interact with customers"
})

(:Partnership:Aggregate {
  domain: "Business",
  definition: "Strategic relationships with external entities that enhance business capabilities"
})

(:Cost:ValueObject {
  domain: "Business",
  definition: "Financial resources required to operate the business model"
})
```

## Relationships

```cypher
// BusinessModel relationships
(:BusinessModel)-[:DELIVERS {type: "value"}]->(:ValueProposition)
(:BusinessModel)-[:SERVES {type: "customer"}]->(:CustomerSegment)
(:BusinessModel)-[:GENERATES {type: "income"}]->(:Revenue)
(:BusinessModel)-[:REQUIRES {type: "resource"}]->(:Resource)
(:BusinessModel)-[:USES {type: "channel"}]->(:Channel)
(:BusinessModel)-[:INCLUDES {type: "partner"}]->(:Partnership)
(:BusinessModel)-[:INCURS {type: "expense"}]->(:Cost)

// ValueProposition relationships
(:ValueProposition)-[:TARGETS {type: "segment"}]->(:CustomerSegment)
(:ValueProposition)-[:DELIVERED_THROUGH {type: "channel"}]->(:Channel)
(:ValueProposition)-[:REQUIRES {type: "resource"}]->(:Resource)
(:ValueProposition)-[:GENERATES {type: "income"}]->(:Revenue)

// CustomerSegment relationships
(:CustomerSegment)-[:ACCESSED_VIA {type: "channel"}]->(:Channel)
(:CustomerSegment)-[:GENERATES {type: "income"}]->(:Revenue)
(:CustomerSegment)-[:REQUIRES {type: "value"}]->(:ValueProposition)

// Resource relationships
(:Resource)-[:SUPPORTS {type: "value"}]->(:ValueProposition)
(:Resource)-[:INCURS {type: "expense"}]->(:Cost)
(:Resource)-[:ENHANCED_BY {type: "partner"}]->(:Partnership)
```

## Taxonomies

### Business Processing

```cypher
(:Taxonomy {name: "BusinessProcessing"})
-[:CONTAINS]->(:Category {name: "ValueOperations"})
-[:CONTAINS]->(:Operation {name: "ValueCreation"})
-[:CONTAINS]->(:Operation {name: "ValueDelivery"})
-[:CONTAINS]->(:Operation {name: "ValueCapture"})

(:Category {name: "CustomerOperations"})
-[:CONTAINS]->(:Operation {name: "CustomerAcquisition"})
-[:CONTAINS]->(:Operation {name: "CustomerRetention"})
-[:CONTAINS]->(:Operation {name: "CustomerEngagement"})

(:Category {name: "ResourceManagement"})
-[:CONTAINS]->(:Operation {name: "ResourceAllocation"})
-[:CONTAINS]->(:Operation {name: "ResourceOptimization"})
-[:CONTAINS]->(:Operation {name: "PartnershipManagement"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "ValueDelivery"})
-[:APPLIES_TO]->(:ValueProposition)
-[:REQUIRES]->(:Resource)
-[:PRODUCES]->(:Revenue)

(:UsageContext {name: "CustomerManagement"})
-[:APPLIES_TO]->(:CustomerSegment)
-[:REQUIRES]->(:Channel)
-[:PRODUCES]->(:Engagement)

(:UsageContext {name: "ResourceOptimization"})
-[:APPLIES_TO]->(:Resource)
-[:REQUIRES]->(:Partnership)
-[:PRODUCES]->(:Efficiency)
```

## Code References

```cypher
(:CodeBase {path: "notes/businessmodel/readme.md"})
-[:IMPLEMENTS]->(:BusinessModel)

(:CodeBase {path: "notes/businessmodel/value.md"})
-[:IMPLEMENTS]->(:ValueProposition)

(:CodeBase {path: "notes/businessmodel/customers.md"})
-[:IMPLEMENTS]->(:CustomerSegment)

(:CodeBase {path: "notes/businessmodel/resources.md"})
-[:IMPLEMENTS]->(:Resource)
``` 