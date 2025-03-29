# Knowledge Domain Vocabulary

## Domain Objects

### Core Knowledge Entities

```cypher
(:Research:Aggregate {
  domain: "Knowledge",
  definition: "A systematic investigation to establish facts, test theories, and develop new understanding"
})

(:Evidence:Entity {
  domain: "Knowledge",
  definition: "Observable data or information that supports or contradicts a claim"
})

(:Method:ValueObject {
  domain: "Knowledge",
  definition: "A systematic procedure for collecting and analyzing evidence"
})

(:Finding:Entity {
  domain: "Knowledge",
  definition: "A specific result or insight derived from research"
})

(:Citation:ValueObject {
  domain: "Knowledge",
  definition: "A reference to a source of information or evidence"
})

(:Fact:Entity {
  domain: "Knowledge",
  definition: "A proven claim with verifiable evidence and reproducible validation"
})

(:Claim:Entity {
  domain: "Knowledge",
  definition: "An assertion that has a repeatable construction method"
})

(:Theory:Aggregate {
  domain: "Knowledge",
  definition: "A structured belief system with context, explanation, and sources"
})
```

## Relationships

```cypher
// Research relationships
(:Research)-[:CONTAINS {type: "component"}]->(:Evidence)
(:Research)-[:CONTAINS {type: "component"}]->(:Method)
(:Research)-[:PRODUCES {type: "output"}]->(:Finding)
(:Research)-[:VALIDATES {type: "process"}]->(:Claim)

// Evidence relationships
(:Evidence)-[:SUPPORTS {type: "validation"}]->(:Fact)
(:Evidence)-[:SUPPORTS {type: "validation"}]->(:Claim)
(:Evidence)-[:PART_OF {type: "component"}]->(:Research)
(:Evidence)-[:HAS {type: "property"}]->(:Classification)

// Method relationships
(:Method)-[:PART_OF {type: "component"}]->(:Research)
(:Method)-[:PRODUCES {type: "output"}]->(:Evidence)
(:Method)-[:FOLLOWS {type: "compliance"}]->(:Standard)

// Finding relationships
(:Finding)-[:BASED_ON {type: "source"}]->(:Evidence)
(:Finding)-[:SUPPORTS {type: "validation"}]->(:Claim)
(:Finding)-[:PART_OF {type: "component"}]->(:Research)

// Citation relationships
(:Citation)-[:REFERENCES {type: "link"}]->(:Source)
(:Citation)-[:VALIDATES {type: "process"}]->(:Claim)
(:Citation)-[:SUPPORTS {type: "validation"}]->(:Research)

// Fact relationships
(:Fact)-[:VALIDATES {type: "process"}]->(:Claim)
(:Fact)-[:SUPPORTS {type: "validation"}]->(:Theory)
(:Fact)-[:CONTAINS {type: "component"}]->(:Proof)

// Claim relationships
(:Claim)-[:DEPENDS_ON {type: "requirement"}]->(:Fact)
(:Claim)-[:SUPPORTS {type: "validation"}]->(:Theory)
(:Claim)-[:CONTAINS {type: "component"}]->(:Argument)

// Theory relationships
(:Theory)-[:CONTAINS {type: "component"}]->(:Claim)
(:Theory)-[:USES {type: "dependency"}]->(:Fact)
(:Theory)-[:SUPPORTS {type: "validation"}]->(:Model)
```

## Taxonomies

### Knowledge Processing

```cypher
(:Taxonomy {name: "KnowledgeProcessing"})
-[:CONTAINS]->(:Category {name: "ResearchOperations"})
-[:CONTAINS]->(:Operation {name: "EvidenceCollection"})
-[:CONTAINS]->(:Operation {name: "MethodApplication"})
-[:CONTAINS]->(:Operation {name: "FindingAnalysis"})

(:Category {name: "ValidationOperations"})
-[:CONTAINS]->(:Operation {name: "FactVerification"})
-[:CONTAINS]->(:Operation {name: "ClaimValidation"})
-[:CONTAINS]->(:Operation {name: "TheoryTesting"})

(:Category {name: "KnowledgeConstruction"})
-[:CONTAINS]->(:Operation {name: "FactBuilding"})
-[:CONTAINS]->(:Operation {name: "ClaimFormulation"})
-[:CONTAINS]->(:Operation {name: "TheoryDevelopment"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "KnowledgeCreation"})
-[:APPLIES_TO]->(:Research)
-[:REQUIRES]->(:Method)
-[:PRODUCES]->(:Finding)

(:UsageContext {name: "KnowledgeValidation"})
-[:APPLIES_TO]->(:Evidence)
-[:REQUIRES]->(:Citation)
-[:PRODUCES]->(:Fact)

(:UsageContext {name: "KnowledgeEvolution"})
-[:APPLIES_TO]->(:Theory)
-[:REQUIRES]->(:Claim)
-[:PRODUCES]->(:Model)
```

## Code References

```cypher
(:CodeBase {path: "notes/knowledge/research.md"})
-[:IMPLEMENTS]->(:Research)

(:CodeBase {path: "notes/knowledge/evidence.md"})
-[:IMPLEMENTS]->(:Evidence)

(:CodeBase {path: "notes/knowledge/method.md"})
-[:IMPLEMENTS]->(:Method)

(:CodeBase {path: "notes/knowledge/finding.md"})
-[:IMPLEMENTS]->(:Finding)
``` 