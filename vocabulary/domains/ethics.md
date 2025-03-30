# Ethics Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Principle:Aggregate {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "A fundamental moral or ethical rule that guides behavior and decision-making"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Value:ValueObject {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "A core belief or standard that influences ethical choices and behaviors"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Framework:Aggregate {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "A structured system of ethical principles and values for moral reasoning"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Decision:Process {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "The process of making choices based on ethical principles and values"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Impact:ValueObject {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "The effect of decisions and actions on stakeholders and their values"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Stakeholder:Entity {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "An individual or group affected by or concerned with ethical decisions"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Responsibility:ValueObject {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "Moral obligations and duties towards stakeholders and society"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Evaluation:Process {
  domain: "Business",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Decision)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Framework)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Value)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Principle)-[",
  term: "Evaluation",
  term: "Responsibility",
  term: "Stakeholder",
  term: "Impact",
  term: "Decision",
  term: "Framework",
  term: "Value",
  term: "Principle",
  definition: "The assessment of decisions and actions against ethical principles and values"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
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
  code_reference: "TBD",
(:Taxonomy {name: "EthicsProcessing"})
  code_reference: "TBD",
-[:CONTAINS]->(:Category {name: "PrincipleOperations"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "PrincipleDefinition"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ValueAlignment"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ImpactAssessment"})

  code_reference: "TBD",
(:Category {name: "DecisionManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "DecisionEvaluation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "StakeholderConsideration"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ResponsibilityAssignment"})

  code_reference: "TBD",
(:Category {name: "FrameworkManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "FrameworkDevelopment"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "PrincipleIntegration"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ValueHarmonization"})
```

## Usage Contexts

```cypher
  code_reference: "TBD",
(:UsageContext {name: "DecisionMaking"})
-[:APPLIES_TO]->(:Decision)
-[:REQUIRES]->(:Principle)
-[:PRODUCES]->(:Impact)

  code_reference: "TBD",
(:UsageContext {name: "StakeholderEngagement"})
-[:APPLIES_TO]->(:Stakeholder)
-[:REQUIRES]->(:Value)
-[:PRODUCES]->(:Responsibility)

  code_reference: "TBD",
(:UsageContext {name: "EthicalEvaluation"})
-[:APPLIES_TO]->(:Evaluation)
-[:REQUIRES]->(:Framework)
-[:PRODUCES]->(:Assessment)
```

## Code References

```cypher
  code_reference: "TBD",
(:CodeBase {path: "notes/ethics/readme.md"})
-[:IMPLEMENTS]->(:Framework)

  code_reference: "TBD",
(:CodeBase {path: "notes/ethics/principles.md"})
-[:IMPLEMENTS]->(:Principle)

  code_reference: "TBD",
(:CodeBase {path: "notes/ethics/values.md"})
-[:IMPLEMENTS]->(:Value)

  code_reference: "TBD",
(:CodeBase {path: "notes/ethics/decisions.md"})
-[:IMPLEMENTS]->(:Decision)
``` 