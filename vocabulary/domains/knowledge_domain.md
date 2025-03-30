# Knowledge Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Research:Aggregate {
  domain: "Knowledge",
  term: "Research",
  taxonomy: "Knowledge Taxonomy",
  definition: "Systematic investigation to establish facts and reach conclusions",
  usage_context: "Foundation for knowledge creation and validation",
  code_reference: "TBD"
})

(:Evidence:Entity {
  domain: "Knowledge",
  term: "Evidence",
  taxonomy: "Knowledge Taxonomy",
  definition: "Observable data or information that supports or contradicts a claim",
  usage_context: "Basis for fact validation and theory building",
  code_reference: "TBD"
})

(:Method:ValueObject {
  domain: "Knowledge",
  term: "Method",
  taxonomy: "Knowledge Taxonomy",
  definition: "Systematic procedure for collecting and analyzing evidence",
  usage_context: "Ensures reproducibility and validity of research",
  code_reference: "TBD"
})

(:Finding:Entity {
  domain: "Knowledge",
  term: "Finding",
  taxonomy: "Knowledge Taxonomy",
  definition: "Specific result or insight derived from research",
  usage_context: "Building blocks for knowledge construction",
  code_reference: "TBD"
})

(:Citation:ValueObject {
  domain: "Knowledge",
  term: "Citation",
  taxonomy: "Knowledge Taxonomy",
  definition: "Reference to a source of information or evidence",
  usage_context: "Tracking and validating information sources",
  code_reference: "TBD"
})

(:KnowledgeGraph:Aggregate {
  domain: "Knowledge",
  term: "Knowledge Graph",
  taxonomy: "Knowledge Taxonomy",
  definition: "Structured representation of relationships between knowledge entities",
  usage_context: "Visual and programmatic knowledge exploration",
  code_reference: "TBD"
})

(:Fact:Entity {
  domain: "Knowledge",
  term: "Fact",
  taxonomy: "Knowledge Taxonomy",
  definition: "Proven claim with verifiable evidence and reproducible validation",
  usage_context: "Foundation for building reliable knowledge base",
  code_reference: "TBD"
})

(:Claim:Entity {
  domain: "Knowledge",
  term: "Claim",
  taxonomy: "Knowledge Taxonomy",
  definition: "Assertion that has a repeatable construction method",
  usage_context: "Building blocks for theories and knowledge construction",
  code_reference: "TBD"
})

(:Theory:Aggregate {
  domain: "Knowledge",
  term: "Theory",
  taxonomy: "Knowledge Taxonomy",
  definition: "Structured belief system with context, explanation, and sources",
  usage_context: "Framework for understanding complex systems",
  code_reference: "TBD"
})
```

## Relationships

```cypher
// Research relationships
(:Research)-[:CONTAINS {type: "component"}]->(:Evidence)
(:Research)-[:CONTAINS {type: "component"}]->(:Method)
(:Research)-[:CONTAINS {type: "component"}]->(:Finding)
(:Research)-[:VALIDATES {type: "process"}]->(:Claim)
(:Research)-[:PRODUCES {type: "output"}]->(:Knowledge)

// Evidence relationships
(:Evidence)-[:SUPPORTS {type: "validation"}]->(:Fact)
(:Evidence)-[:SUPPORTS {type: "validation"}]->(:Claim)
(:Evidence)-[:PART_OF {type: "component"}]->(:Research)

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

// Knowledge Graph relationships
(:KnowledgeGraph)-[:CONTAINS {type: "component"}]->(:Node)
(:KnowledgeGraph)-[:CONTAINS {type: "component"}]->(:Edge)
(:KnowledgeGraph)-[:REPRESENTS {type: "relationship"}]->(:Relationship)
(:KnowledgeGraph)-[:ENABLES {type: "functionality"}]->(:Navigation)

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

### Knowledge Taxonomy

```cypher
(:Taxonomy {name: "Knowledge Taxonomy"})
-[:CONTAINS]->(:Category {name: "ResearchElements"})
-[:CONTAINS]->(:Category {name: "ValidationElements"})
-[:CONTAINS]->(:Category {name: "KnowledgeConstruction"})

(:Category {name: "ResearchElements"})
-[:CONTAINS]->(:Term {name: "Research"})
-[:CONTAINS]->(:Term {name: "Evidence"})
-[:CONTAINS]->(:Term {name: "Method"})
-[:CONTAINS]->(:Term {name: "Finding"})
-[:CONTAINS]->(:Term {name: "Citation"})

(:Category {name: "ValidationElements"})
-[:CONTAINS]->(:Term {name: "Fact"})
-[:CONTAINS]->(:Term {name: "Claim"})
-[:CONTAINS]->(:Term {name: "Proof"})

(:Category {name: "KnowledgeConstruction"})
-[:CONTAINS]->(:Term {name: "Theory"})
-[:CONTAINS]->(:Term {name: "KnowledgeGraph"})
-[:CONTAINS]->(:Term {name: "Model"})
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

(:UsageContext {name: "KnowledgeModeling"})
-[:APPLIES_TO]->(:Theory)
-[:REQUIRES]->(:Fact)
-[:PRODUCES]->(:KnowledgeGraph)

(:UsageContext {name: "KnowledgeExploration"})
-[:APPLIES_TO]->(:KnowledgeGraph)
-[:REQUIRES]->(:Query)
-[:PRODUCES]->(:Insight)
```

## Code References

```cypher
(:CodeBase {path: "scripts/generate_knowledge_graph.rs"})
-[:IMPLEMENTS]->(:KnowledgeGraph)

(:CodeBase {path: "notes/knowledge/readme.md"})
-[:IMPLEMENTS]->(:Research)

(:CodeBase {path: "notes/knowledge_management.md"})
-[:IMPLEMENTS]->(:Method)

(:CodeBase {path: "vocabulary/knowledge/terms.md"})
-[:IMPLEMENTS]->(:Term)
``` 