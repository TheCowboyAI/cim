# Arguments Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Argument:Aggregate {
  domain: "Knowledge",
  definition: "A structured reasoning construct that supports or challenges a claim through logical relationships and evidence"
})

(:Evidence:ValueObject {
  domain: "Knowledge",
  definition: "Supporting information or data that strengthens or weakens an argument"
})

(:Context:ValueObject {
  domain: "Knowledge",
  definition: "The specific circumstances or conditions under which an argument is applicable"
})

(:Logic:Service {
  domain: "Knowledge",
  definition: "The system of reasoning principles used to construct and evaluate arguments"
})
```

## Relationships

```cypher
// Argument relationships
(:Argument)-[:PART_OF {context: "framework"}]->(:KnowledgeFramework)
(:Argument)-[:MANAGES {type: "validation"}]->(:Evidence)
(:Argument)-[:PROCESSES {type: "validation"}]->(:Claim)
(:Argument)-[:DEPENDS_ON {type: "reasoning"}]->(:Logic)

// Evidence relationships
(:Evidence)-[:PART_OF {type: "component"}]->(:Argument)
(:Evidence)-[:VALIDATES {type: "support"}]->(:Claim)
(:Evidence)-[:SOURCES {type: "origin"}]->(:Fact)
(:Evidence)-[:DEPENDS_ON {type: "evaluation"}]->(:Context)

// Context relationships
(:Context)-[:CONTAINS {type: "rules"}]->(:Condition)
(:Context)-[:CONFIGURES {scope: "applicability"}]->(:Argument)
(:Context)-[:VALIDATES {type: "scope"}]->(:Applicability)
(:Context)-[:PRECEDES {phase: "evaluation"}]->(:Evaluation)

// Logic relationships
(:Logic)-[:PROCESSES {type: "reasoning"}]->(:Argument)
(:Logic)-[:VALIDATES {type: "consistency"}]->(:Reasoning)
(:Logic)-[:CONTAINS {type: "inference"}]->(:Rule)
(:Logic)-[:CONFIGURES {type: "process"}]->(:Evaluation)
```

## Taxonomies

### Knowledge Processing

```cypher
(:Taxonomy {name: "KnowledgeProcessing"})
-[:CONTAINS]->(:Category {name: "ArgumentConstruction"})
-[:CONTAINS]->(:Operation {name: "StructureDefinition"})

(:Category {name: "ArgumentConstruction"})
-[:CONTAINS]->(:Operation {name: "EvidenceIntegration"})
-[:CONTAINS]->(:Operation {name: "LogicApplication"})

(:Taxonomy {name: "KnowledgeProcessing"})
-[:CONTAINS]->(:Category {name: "ValidationOperations"})
-[:CONTAINS]->(:Operation {name: "ContextVerification"})
-[:CONTAINS]->(:Operation {name: "EvidenceAssessment"})
-[:CONTAINS]->(:Operation {name: "LogicEvaluation"})

(:Taxonomy {name: "KnowledgeProcessing"})
-[:CONTAINS]->(:Category {name: "RelationshipManagement"})
-[:CONTAINS]->(:Operation {name: "ClaimConnections"})
-[:CONTAINS]->(:Operation {name: "EvidenceLinks"})
-[:CONTAINS]->(:Operation {name: "ContextMappings"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "FormalReasoning"})
-[:APPLIES_TO]->(:Argument)
-[:REQUIRES]->(:Logic)

(:UsageContext {name: "ClaimValidation"})
-[:APPLIES_TO]->(:Argument)
-[:REQUIRES]->(:Evidence)

(:UsageContext {name: "KnowledgeVerification"})
-[:APPLIES_TO]->(:Argument)
-[:REQUIRES]->(:Context)
```

## Code References

```cypher
(:CodeBase {path: "notes/arguments/readme.md"})
-[:IMPLEMENTS]->(:Argument)

(:CodeBase {path: "notes/arguments/evidence.md"})
-[:IMPLEMENTS]->(:Evidence)

(:CodeBase {path: "notes/arguments/context.md"})
-[:IMPLEMENTS]->(:Context)

(:CodeBase {path: "notes/arguments/logic.md"})
-[:IMPLEMENTS]->(:Logic)
``` 