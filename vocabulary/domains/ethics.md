# Ethics Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Principle:Aggregate {
  domain: "Business",
  definition: "A fundamental moral or ethical rule that guides behavior and decision-making"
})

(:Value:ValueObject {
  domain: "Business",
  definition: "A core belief or standard that influences ethical choices and behaviors"
})

(:Framework:Aggregate {
  domain: "Business",
  definition: "A structured system of ethical principles and values for moral reasoning"
})

(:Decision:Process {
  domain: "Business",
  definition: "The process of making choices based on ethical principles and values"
})

(:Impact:ValueObject {
  domain: "Business",
  definition: "The effect of decisions and actions on stakeholders and their values"
})

(:Stakeholder:Entity {
  domain: "Business",
  definition: "An individual or group affected by or concerned with ethical decisions"
})

(:Responsibility:ValueObject {
  domain: "Business",
  definition: "Moral obligations and duties towards stakeholders and society"
})

(:Evaluation:Process {
  domain: "Business",
  definition: "The assessment of decisions and actions against ethical principles and values"
})
```

## Relationships

```cypher
// Principle relationships
(:Principle)-[:PART_OF {type: "system"}]->(:Framework)
(:Principle)-[:GUIDES {type: "process"}]->(:Decision)
(:Principle)-[:DEFINES {type: "obligation"}]->(:Responsibility)
(:Principle)-[:EVALUATED_BY {type: "process"}]->(:Evaluation)
(:Principle)-[:IMPACTS {type: "entity"}]->(:Stakeholder)

// Value relationships
(:Value)-[:INFLUENCES {type: "principle"}]->(:Principle)
(:Value)-[:GUIDES {type: "process"}]->(:Decision)
(:Value)-[:HELD_BY {type: "entity"}]->(:Stakeholder)
(:Value)-[:MEASURED_BY {type: "process"}]->(:Evaluation)

// Framework relationships
(:Framework)-[:CONTAINS {type: "component"}]->(:Principle)
(:Framework)-[:INCORPORATES {type: "component"}]->(:Value)
(:Framework)-[:GUIDES {type: "process"}]->(:Decision)
(:Framework)-[:DEFINES {type: "obligation"}]->(:Responsibility)

// Decision relationships
(:Decision)-[:FOLLOWS {type: "principle"}]->(:Principle)
(:Decision)-[:CONSIDERS {type: "value"}]->(:Value)
(:Decision)-[:CREATES {type: "effect"}]->(:Impact)
(:Decision)-[:AFFECTS {type: "entity"}]->(:Stakeholder)
```

## Taxonomies

### Ethics Processing

```cypher
(:Taxonomy {name: "EthicsProcessing"})
-[:CONTAINS]->(:Category {name: "PrincipleOperations"})
-[:CONTAINS]->(:Operation {name: "PrincipleDefinition"})
-[:CONTAINS]->(:Operation {name: "ValueAlignment"})
-[:CONTAINS]->(:Operation {name: "ImpactAssessment"})

(:Category {name: "DecisionManagement"})
-[:CONTAINS]->(:Operation {name: "DecisionEvaluation"})
-[:CONTAINS]->(:Operation {name: "StakeholderConsideration"})
-[:CONTAINS]->(:Operation {name: "ResponsibilityAssignment"})

(:Category {name: "FrameworkManagement"})
-[:CONTAINS]->(:Operation {name: "FrameworkDevelopment"})
-[:CONTAINS]->(:Operation {name: "PrincipleIntegration"})
-[:CONTAINS]->(:Operation {name: "ValueHarmonization"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "DecisionMaking"})
-[:APPLIES_TO]->(:Decision)
-[:REQUIRES]->(:Principle)
-[:PRODUCES]->(:Impact)

(:UsageContext {name: "StakeholderEngagement"})
-[:APPLIES_TO]->(:Stakeholder)
-[:REQUIRES]->(:Value)
-[:PRODUCES]->(:Responsibility)

(:UsageContext {name: "EthicalEvaluation"})
-[:APPLIES_TO]->(:Evaluation)
-[:REQUIRES]->(:Framework)
-[:PRODUCES]->(:Assessment)
```

## Code References

```cypher
(:CodeBase {path: "notes/ethics/readme.md"})
-[:IMPLEMENTS]->(:Framework)

(:CodeBase {path: "notes/ethics/principles.md"})
-[:IMPLEMENTS]->(:Principle)

(:CodeBase {path: "notes/ethics/values.md"})
-[:IMPLEMENTS]->(:Value)

(:CodeBase {path: "notes/ethics/decisions.md"})
-[:IMPLEMENTS]->(:Decision)
``` 