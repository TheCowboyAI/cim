# Claims Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Claim:Aggregate {
  domain: "Knowledge",
  definition: "A statement or assertion that requires verification through evidence and arguments"
})

(:ResearchClaim:Entity {
  domain: "Knowledge",
  definition: "A claim derived from research papers, documentation, or academic sources"
})

(:ImplementationClaim:Entity {
  domain: "Knowledge",
  definition: "A claim about technical implementation patterns, architectures, or approaches"
})

(:ArchitecturalClaim:Entity {
  domain: "Knowledge",
  definition: "A claim about system architecture, design patterns, or structural decisions"
})

(:Assessment:Process {
  domain: "Knowledge",
  definition: "The process of evaluating a claim's validity and applicability"
})

(:Evidence:ValueObject {
  domain: "Knowledge",
  definition: "Supporting information or data that strengthens or challenges a claim"
})

(:Context:ValueObject {
  domain: "Knowledge",
  definition: "The specific circumstances or conditions under which a claim is applicable"
})

(:Verification:Process {
  domain: "Knowledge",
  definition: "The process of validating a claim through evidence and logical reasoning"
})
```

## Relationships

```cypher
// Claim relationships
(:Claim)-[:SUPPORTED_BY {type: "validation"}]->(:Evidence)
(:Claim)-[:APPLIES_TO {type: "scope"}]->(:Context)
(:Claim)-[:VERIFIED_BY {type: "process"}]->(:Verification)
(:Claim)-[:ASSESSED_BY {type: "process"}]->(:Assessment)

// ResearchClaim relationships
(:ResearchClaim)-[:CITES {type: "source"}]->(:Paper)
(:ResearchClaim)-[:REFERENCES {type: "document"}]->(:Documentation)
(:ResearchClaim)-[:SUPPORTS {type: "validation"}]->(:Theory)
(:ResearchClaim)-[:CONTRIBUTES_TO {type: "knowledge"}]->(:Domain)

// ImplementationClaim relationships
(:ImplementationClaim)-[:DESCRIBES {type: "pattern"}]->(:Pattern)
(:ImplementationClaim)-[:USES {type: "technology"}]->(:Technology)
(:ImplementationClaim)-[:IMPACTS {type: "system"}]->(:Architecture)
(:ImplementationClaim)-[:REQUIRES {type: "resource"}]->(:Resource)

// ArchitecturalClaim relationships
(:ArchitecturalClaim)-[:DEFINES {type: "structure"}]->(:Architecture)
(:ArchitecturalClaim)-[:FOLLOWS {type: "pattern"}]->(:Pattern)
(:ArchitecturalClaim)-[:INFLUENCES {type: "design"}]->(:System)
(:ArchitecturalClaim)-[:REQUIRES {type: "component"}]->(:Component)
```

## Taxonomies

### Claim Processing

```cypher
(:Taxonomy {name: "ClaimProcessing"})
-[:CONTAINS]->(:Category {name: "ResearchValidation"})
-[:CONTAINS]->(:Operation {name: "PaperAnalysis"})
-[:CONTAINS]->(:Operation {name: "DocumentationReview"})
-[:CONTAINS]->(:Operation {name: "PatternValidation"})

(:Category {name: "ImplementationVerification"})
-[:CONTAINS]->(:Operation {name: "PatternTesting"})
-[:CONTAINS]->(:Operation {name: "TechnologyValidation"})
-[:CONTAINS]->(:Operation {name: "ResourceVerification"})

(:Category {name: "ArchitecturalAssessment"})
-[:CONTAINS]->(:Operation {name: "StructureAnalysis"})
-[:CONTAINS]->(:Operation {name: "PatternEvaluation"})
-[:CONTAINS]->(:Operation {name: "SystemImpactAssessment"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "ResearchValidation"})
-[:APPLIES_TO]->(:ResearchClaim)
-[:REQUIRES]->(:Evidence)
-[:PRODUCES]->(:Verification)

(:UsageContext {name: "ImplementationAssessment"})
-[:APPLIES_TO]->(:ImplementationClaim)
-[:REQUIRES]->(:Pattern)
-[:PRODUCES]->(:Assessment)

(:UsageContext {name: "ArchitecturalEvaluation"})
-[:APPLIES_TO]->(:ArchitecturalClaim)
-[:REQUIRES]->(:Architecture)
-[:PRODUCES]->(:Verification)
```

## Code References

```cypher
(:CodeBase {path: "notes/claims/readme.md"})
-[:IMPLEMENTS]->(:Claim)

(:CodeBase {path: "notes/claims/ecs-research.md"})
-[:IMPLEMENTS]->(:ImplementationClaim)

(:CodeBase {path: "notes/claims/eda-research.md"})
-[:IMPLEMENTS]->(:ArchitecturalClaim)

(:CodeBase {path: "notes/claims/cqrs-papers.md"})
-[:IMPLEMENTS]->(:ResearchClaim)
``` 