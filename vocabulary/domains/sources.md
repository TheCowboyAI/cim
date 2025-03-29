# Sources Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Source:Aggregate {
  domain: "Technical",
  definition: "A verified origin of information with tracked provenance and trust metrics"
})

(:Verification:Process {
  domain: "Technical",
  definition: "The process of validating source authenticity, reliability, and accuracy"
})

(:Trust:ValueObject {
  domain: "Technical",
  definition: "A quantified measure of source reliability and credibility"
})

(:Access:Service {
  domain: "Technical",
  definition: "The system managing source availability and retrieval mechanisms"
})

(:Update:Process {
  domain: "Technical",
  definition: "The process of maintaining and refreshing source information"
})

(:Link:Entity {
  domain: "Technical",
  definition: "A connection between sources or to external references"
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
(:Taxonomy {name: "SourceProcessing"})
-[:CONTAINS]->(:Category {name: "SourceOperations"})
-[:CONTAINS]->(:Operation {name: "SourceVerification"})
-[:CONTAINS]->(:Operation {name: "TrustAssessment"})
-[:CONTAINS]->(:Operation {name: "AccessControl"})

(:Category {name: "UpdateManagement"})
-[:CONTAINS]->(:Operation {name: "UpdateTracking"})
-[:CONTAINS]->(:Operation {name: "VersionControl"})
-[:CONTAINS]->(:Operation {name: "ChangeValidation"})

(:Category {name: "LinkManagement"})
-[:CONTAINS]->(:Operation {name: "LinkValidation"})
-[:CONTAINS]->(:Operation {name: "ReferenceTracking"})
-[:CONTAINS]->(:Operation {name: "ConnectionMonitoring"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "SourceValidation"})
-[:APPLIES_TO]->(:Source)
-[:REQUIRES]->(:Verification)
-[:PRODUCES]->(:Trust)

(:UsageContext {name: "InformationRetrieval"})
-[:APPLIES_TO]->(:Source)
-[:REQUIRES]->(:Access)
-[:PRODUCES]->(:Information)

(:UsageContext {name: "SourceMaintenance"})
-[:APPLIES_TO]->(:Source)
-[:REQUIRES]->(:Update)
-[:PRODUCES]->(:Version)
```

## Code References

```cypher
(:CodeBase {path: "notes/sources/readme.md"})
-[:IMPLEMENTS]->(:Source)

(:CodeBase {path: "notes/sources/verification.md"})
-[:IMPLEMENTS]->(:Verification)

(:CodeBase {path: "notes/sources/trust.md"})
-[:IMPLEMENTS]->(:Trust)

(:CodeBase {path: "notes/sources/access.md"})
-[:IMPLEMENTS]->(:Access)
``` 