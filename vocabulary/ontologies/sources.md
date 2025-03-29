# Sources Domain Ontology

## Core Relationships

```cypher
// Hierarchical Relationships
(:Technical:Domain)
-[:CONTAINS]->(:Source:Aggregate)
-[:CONTAINS]->(:Verification:Process)
-[:CONTAINS]->(:Trust:ValueObject)
-[:CONTAINS]->(:Access:Service)
-[:CONTAINS]->(:Update:Process)
-[:CONTAINS]->(:Link:Entity)

// Functional Relationships
(:Source:Aggregate)
-[:VERIFIED_BY {type: "process"}]->(:Verification:Process)
-[:HAS {type: "metric"}]->(:Trust:ValueObject)
-[:ACCESSED_THROUGH {type: "service"}]->(:Access:Service)
-[:UPDATED_BY {type: "process"}]->(:Update:Process)

(:Verification:Process)
-[:VALIDATES {type: "process"}]->(:Source:Aggregate)
-[:PRODUCES {type: "metric"}]->(:Trust:ValueObject)
-[:FOLLOWS {type: "standard"}]->(:Protocol:Entity)

(:Trust:ValueObject)
-[:MEASURES {type: "metric"}]->(:Reliability:Entity)
-[:EVALUATES {type: "metric"}]->(:Credibility:Entity)
-[:IMPACTS {type: "value"}]->(:Usage:Entity)

// Temporal Relationships
(:Source:Aggregate)
-[:PRECEDES {phase: "verification"}]->(:SourceVerification:Process)
-[:PRECEDES {phase: "assessment"}]->(:TrustAssessment:Process)
-[:PRECEDES {phase: "access"}]->(:AccessControl:Process)

(:Update:Process)
-[:PRECEDES {phase: "tracking"}]->(:UpdateTracking:Process)
-[:PRECEDES {phase: "versioning"}]->(:VersionControl:Process)
-[:PRECEDES {phase: "validation"}]->(:ChangeValidation:Process)

(:Link:Entity)
-[:PRECEDES {phase: "validation"}]->(:LinkValidation:Process)
-[:PRECEDES {phase: "tracking"}]->(:ReferenceTracking:Process)
-[:PRECEDES {phase: "monitoring"}]->(:ConnectionMonitoring:Process)

## Cross-Domain Relationships

```cypher
// Source-Knowledge Relationships
(:Source:Aggregate)
-[:PROVIDES {type: "input"}]->(:Evidence:Entity)
-[:SUPPORTS {type: "validation"}]->(:Fact:Entity)
-[:REFERENCED_BY {type: "citation"}]->(:Finding:Entity)

(:Verification:Process)
-[:APPLIES {type: "method"}]->(:Method:ValueObject)
-[:PRODUCES {type: "output"}]->(:Evidence:Entity)

// Source-Agent Relationships
(:Source:Aggregate)
-[:MANAGED_BY {type: "agent"}]->(:Agent:Aggregate)
-[:VALIDATED_BY {type: "process"}]->(:Behavior:Process)

(:Access:Service)
-[:CONTROLLED_BY {type: "capability"}]->(:Capability:ValueObject)
-[:MONITORED_BY {type: "process"}]->(:Monitor:Service)

// Source-Technical Relationships
(:Source:Aggregate)
-[:STORED_IN {type: "system"}]->(:Repository:Service)
-[:ACCESSED_VIA {type: "service"}]->(:API:Service)

(:Link:Entity)
-[:USES {type: "protocol"}]->(:Network:Service)
-[:TRACKED_BY {type: "system"}]->(:Monitor:Service)
```

## Domain Rules

```cypher
// Source Rules
(:Rule {name: "SourceVerification"})
-[:REQUIRES]->(:Protocol)
-[:PRODUCES]->(:ValidationResult)
-[:ENFORCES]->(:Standard {name: "VerificationProtocol"})

(:Rule {name: "TrustAssessment"})
-[:REQUIRES]->(:Metric)
-[:PRODUCES]->(:TrustScore)
-[:ENFORCES]->(:Standard {name: "TrustMetrics"})

(:Rule {name: "AccessControl"})
-[:REQUIRES]->(:Permission)
-[:PRODUCES]->(:AccessGrant)
-[:ENFORCES]->(:Standard {name: "AccessProtocol"})

// Process Rules
(:Rule {name: "UpdateProcess"})
-[:DEFINES]->(:Workflow {name: "UpdateMethodology"})
-[:REQUIRES]->(:Change)
-[:PRODUCES]->(:Version)

(:Rule {name: "LinkProcess"})
-[:DEFINES]->(:Workflow {name: "LinkMethodology"})
-[:REQUIRES]->(:Reference)
-[:PRODUCES]->(:Connection)

(:Rule {name: "ValidationProcess"})
-[:DEFINES]->(:Workflow {name: "ValidationMethodology"})
-[:REQUIRES]->(:Protocol)
-[:PRODUCES]->(:ValidationResult)
``` 