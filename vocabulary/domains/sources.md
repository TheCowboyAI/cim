# Sources Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Source:Aggregate {
  domain: "Technical",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Link",
  term: "Update",
  term: "Access",
  term: "Trust",
  term: "Verification",
  term: "Source",
  definition: "A verified origin of information with tracked provenance and trust metrics"
  taxonomy: "Technical Taxonomy",
  usage_context: "Technical related operations and processes",
  code_reference: "TBD",
})

(:Verification:Process {
  domain: "Technical",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Link",
  term: "Update",
  term: "Access",
  term: "Trust",
  term: "Verification",
  term: "Source",
  definition: "The process of validating source authenticity, reliability, and accuracy"
  taxonomy: "Technical Taxonomy",
  usage_context: "Technical related operations and processes",
  code_reference: "TBD",
})

(:Trust:ValueObject {
  domain: "Technical",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Link",
  term: "Update",
  term: "Access",
  term: "Trust",
  term: "Verification",
  term: "Source",
  definition: "A quantified measure of source reliability and credibility"
  taxonomy: "Technical Taxonomy",
  usage_context: "Technical related operations and processes",
  code_reference: "TBD",
})

(:Access:Service {
  domain: "Technical",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Link",
  term: "Update",
  term: "Access",
  term: "Trust",
  term: "Verification",
  term: "Source",
  definition: "The system managing source availability and retrieval mechanisms"
  taxonomy: "Technical Taxonomy",
  usage_context: "Technical related operations and processes",
  code_reference: "TBD",
})

(:Update:Process {
  domain: "Technical",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Link",
  term: "Update",
  term: "Access",
  term: "Trust",
  term: "Verification",
  term: "Source",
  definition: "The process of maintaining and refreshing source information"
  taxonomy: "Technical Taxonomy",
  usage_context: "Technical related operations and processes",
  code_reference: "TBD",
})

(:Link:Entity {
  domain: "Technical",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Access)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Trust)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Verification)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Source)-[",
  term: "Link",
  term: "Update",
  term: "Access",
  term: "Trust",
  term: "Verification",
  term: "Source",
  definition: "A connection between sources or to external references"
  taxonomy: "Technical Taxonomy",
  usage_context: "Technical related operations and processes",
  code_reference: "TBD",
})
```

## Relationships

```cypher
// Source relationships
(:Source)-[:VERIFIED_BY {type: "process"}]->(:Verification)
(:Source)-[:HAS {type: "metric"}]->(:Trust)
(:Source)-[:ACCESSED_THROUGH {type: "service"}]->(:Access)
(:Source)-[:UPDATED_BY {type: "process"}]->(:Update)
(:Source)-[:CONNECTED_TO {type: "reference"}]->(:Link)

// Verification relationships
(:Verification)-[:VALIDATES {type: "process"}]->(:Source)
(:Verification)-[:PRODUCES {type: "metric"}]->(:Trust)
(:Verification)-[:FOLLOWS {type: "standard"}]->(:Protocol)
(:Verification)-[:LOGS {type: "record"}]->(:Audit)

// Trust relationships
(:Trust)-[:MEASURES {type: "metric"}]->(:Reliability)
(:Trust)-[:EVALUATES {type: "metric"}]->(:Credibility)
(:Trust)-[:IMPACTS {type: "value"}]->(:Usage)
(:Trust)-[:TRACKED_BY {type: "process"}]->(:Monitor)

// Access relationships
(:Access)-[:MANAGES {type: "service"}]->(:Retrieval)
(:Access)-[:ENFORCES {type: "control"}]->(:Permission)
(:Access)-[:LOGS {type: "record"}]->(:Usage)
(:Access)-[:REQUIRES {type: "validation"}]->(:Authentication)
```

## Taxonomies

### Source Processing

```cypher
  code_reference: "TBD",
(:Taxonomy {name: "SourceProcessing"})
  code_reference: "TBD",
-[:CONTAINS]->(:Category {name: "SourceOperations"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "SourceVerification"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "TrustAssessment"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "AccessControl"})

  code_reference: "TBD",
(:Category {name: "UpdateManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "UpdateTracking"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "VersionControl"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ChangeValidation"})

  code_reference: "TBD",
(:Category {name: "LinkManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "LinkValidation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ReferenceTracking"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ConnectionMonitoring"})
```

## Usage Contexts

```cypher
  code_reference: "TBD",
(:UsageContext {name: "SourceValidation"})
-[:APPLIES_TO]->(:Source)
-[:REQUIRES]->(:Verification)
-[:PRODUCES]->(:Trust)

  code_reference: "TBD",
(:UsageContext {name: "InformationRetrieval"})
-[:APPLIES_TO]->(:Source)
-[:REQUIRES]->(:Access)
-[:PRODUCES]->(:Information)

  code_reference: "TBD",
(:UsageContext {name: "SourceMaintenance"})
-[:APPLIES_TO]->(:Source)
-[:REQUIRES]->(:Update)
-[:PRODUCES]->(:Version)
```

## Code References

```cypher
  code_reference: "TBD",
(:CodeBase {path: "notes/sources/readme.md"})
-[:IMPLEMENTS]->(:Source)

  code_reference: "TBD",
(:CodeBase {path: "notes/sources/verification.md"})
-[:IMPLEMENTS]->(:Verification)

  code_reference: "TBD",
(:CodeBase {path: "notes/sources/trust.md"})
-[:IMPLEMENTS]->(:Trust)

  code_reference: "TBD",
(:CodeBase {path: "notes/sources/access.md"})
-[:IMPLEMENTS]->(:Access)
``` 