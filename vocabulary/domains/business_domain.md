# Business Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:ValueProposition:Aggregate {
  domain: "Business",
  term: "Value Proposition",
  definition: "The unique value offered by a solution or service",
  taxonomy: "Business Rules",
  usage_context: "Defining business value and market positioning",
  code_reference: "TBD"
})

(:Solution:Entity {
  domain: "Business",
  term: "Solution",
  definition: "A specific implementation addressing business needs",
  taxonomy: "Business Rules",
  usage_context: "Concrete implementations of business value",
  code_reference: "TBD"
})
```

## Relationships

```cypher
// Value Proposition relationships
(:ValueProposition)-[:SUPPORTS {type: "strategic"}]->(:BusinessModel)
(:ValueProposition)-[:CONTAINS {type: "component"}]->(:Solution)
(:ValueProposition)-[:TARGETS {type: "objective"}]->(:Goal)

// Solution relationships
(:Solution)-[:PART_OF {type: "component"}]->(:ValueProposition)
(:Solution)-[:USES {type: "resource"}]->(:Model)
(:Solution)-[:ACHIEVES {type: "outcome"}]->(:Goal)
```

## Taxonomies

### Business Taxonomy

```cypher
(:Taxonomy {name: "Business Taxonomy"})
-[:CONTAINS]->(:Category {name: "Business Concepts"})
-[:CONTAINS]->(:Category {name: "Business Rules"})

(:Category {name: "Business Concepts"})
-[:CONTAINS]->(:Term {name: "Value Proposition"})
-[:CONTAINS]->(:Term {name: "Solution"})

(:Category {name: "Business Rules"})
-[:CONTAINS]->(:Term {name: "Business Model"})
-[:CONTAINS]->(:Term {name: "Goal"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "Business Strategy"})
-[:APPLIES_TO]->(:ValueProposition)
-[:REQUIRES]->(:BusinessModel)
-[:PRODUCES]->(:Goal)

(:UsageContext {name: "Implementation"})
-[:APPLIES_TO]->(:Solution)
-[:REQUIRES]->(:Model)
-[:PRODUCES]->(:Value)
```

## Code References

```cypher
(:CodeBase {path: "cim/src/business"})
-[:IMPLEMENTS]->(:ValueProposition)
-[:IMPLEMENTS]->(:Solution)
```

