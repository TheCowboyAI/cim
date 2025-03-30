# Claims Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Claim:Aggregate {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "A statement or assertion that requires verification through evidence and arguments"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
})

(:ResearchClaim:Entity {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "A claim derived from research papers, documentation, or academic sources"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
})

(:ImplementationClaim:Entity {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "A claim about technical implementation patterns, architectures, or approaches"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
})

(:ArchitecturalClaim:Entity {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "A claim about system architecture, design patterns, or structural decisions"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
})

(:Assessment:Process {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "The process of evaluating a claim's validity and applicability"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
})

(:Evidence:ValueObject {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "Supporting information or data that strengthens or challenges a claim"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
})

(:Context:ValueObject {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "The specific circumstances or conditions under which a claim is applicable"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
})

(:Verification:Process {
  domain: "Knowledge",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ArchitecturalClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ImplementationClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "ResearchClaim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Claim)-[",
  term: "Verification",
  term: "Context",
  term: "Evidence",
  term: "Assessment",
  term: "ArchitecturalClaim",
  term: "ImplementationClaim",
  term: "ResearchClaim",
  term: "Claim",
  definition: "The process of validating a claim through evidence and logical reasoning"
  taxonomy: "Knowledge Taxonomy",
  usage_context: "Knowledge related operations and processes",
  code_reference: "TBD",
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
  code_reference: "TBD",
(:Taxonomy {name: "ClaimProcessing"})
  code_reference: "TBD",
-[:CONTAINS]->(:Category {name: "ResearchValidation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "PaperAnalysis"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "DocumentationReview"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "PatternValidation"})

  code_reference: "TBD",
(:Category {name: "ImplementationVerification"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "PatternTesting"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "TechnologyValidation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ResourceVerification"})

  code_reference: "TBD",
(:Category {name: "ArchitecturalAssessment"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "StructureAnalysis"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "PatternEvaluation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "SystemImpactAssessment"})
```

## Usage Contexts

```cypher
  code_reference: "TBD",
(:UsageContext {name: "ResearchValidation"})
-[:APPLIES_TO]->(:ResearchClaim)
-[:REQUIRES]->(:Evidence)
-[:PRODUCES]->(:Verification)

  code_reference: "TBD",
(:UsageContext {name: "ImplementationAssessment"})
-[:APPLIES_TO]->(:ImplementationClaim)
-[:REQUIRES]->(:Pattern)
-[:PRODUCES]->(:Assessment)

  code_reference: "TBD",
(:UsageContext {name: "ArchitecturalEvaluation"})
-[:APPLIES_TO]->(:ArchitecturalClaim)
-[:REQUIRES]->(:Architecture)
-[:PRODUCES]->(:Verification)
```

## Code References

```cypher
  code_reference: "TBD",
(:CodeBase {path: "notes/claims/readme.md"})
-[:IMPLEMENTS]->(:Claim)

  code_reference: "TBD",
(:CodeBase {path: "notes/claims/ecs-research.md"})
-[:IMPLEMENTS]->(:ImplementationClaim)

  code_reference: "TBD",
(:CodeBase {path: "notes/claims/eda-research.md"})
-[:IMPLEMENTS]->(:ArchitecturalClaim)

  code_reference: "TBD",
(:CodeBase {path: "notes/claims/cqrs-papers.md"})
-[:IMPLEMENTS]->(:ResearchClaim)
``` 