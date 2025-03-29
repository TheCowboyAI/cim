# Governance Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Policy:Aggregate {
  domain: "Business",
  definition: "A defined set of rules, guidelines, and controls governing system behavior and operations"
})

(:Compliance:Process {
  domain: "Business",
  definition: "The process of ensuring adherence to policies, laws, and regulations"
})

(:Risk:ValueObject {
  domain: "Business",
  definition: "A measured potential for adverse effects on system operations or objectives"
})

(:Control:Service {
  domain: "Business",
  definition: "A mechanism for enforcing policies and managing risk within the system"
})

(:Audit:Process {
  domain: "Business",
  definition: "The systematic review and verification of system compliance and control effectiveness"
})

(:Decision:Entity {
  domain: "Business",
  definition: "A formal choice or determination made within governance frameworks"
})
```

## Relationships

```cypher
// Policy relationships
(:Policy)-[:ENFORCES {type: "rule"}]->(:Standard)
(:Policy)-[:MANAGES {type: "control"}]->(:Risk)
(:Policy)-[:REQUIRES {type: "process"}]->(:Compliance)
(:Policy)-[:VERIFIED_BY {type: "process"}]->(:Audit)
(:Policy)-[:GUIDES {type: "process"}]->(:Decision)

// Compliance relationships
(:Compliance)-[:VALIDATES {type: "adherence"}]->(:Policy)
(:Compliance)-[:MONITORS {type: "control"}]->(:Control)
(:Compliance)-[:REPORTS {type: "status"}]->(:Audit)
(:Compliance)-[:FOLLOWS {type: "framework"}]->(:Standard)

// Risk relationships
(:Risk)-[:ASSESSED_BY {type: "process"}]->(:Control)
(:Risk)-[:MITIGATED_BY {type: "policy"}]->(:Policy)
(:Risk)-[:MONITORED_BY {type: "process"}]->(:Audit)
(:Risk)-[:IMPACTS {type: "process"}]->(:Decision)

// Control relationships
(:Control)-[:IMPLEMENTS {type: "policy"}]->(:Policy)
(:Control)-[:MANAGES {type: "risk"}]->(:Risk)
(:Control)-[:AUDITED_BY {type: "process"}]->(:Audit)
(:Control)-[:ENFORCES {type: "framework"}]->(:Standard)
```

## Taxonomies

### Governance Processing

```cypher
(:Taxonomy {name: "GovernanceProcessing"})
-[:CONTAINS]->(:Category {name: "PolicyOperations"})
-[:CONTAINS]->(:Operation {name: "PolicyDefinition"})
-[:CONTAINS]->(:Operation {name: "ComplianceMonitoring"})
-[:CONTAINS]->(:Operation {name: "RiskManagement"})

(:Category {name: "ControlManagement"})
-[:CONTAINS]->(:Operation {name: "ControlImplementation"})
-[:CONTAINS]->(:Operation {name: "ControlMonitoring"})
-[:CONTAINS]->(:Operation {name: "ControlOptimization"})

(:Category {name: "AuditProcessing"})
-[:CONTAINS]->(:Operation {name: "AuditPlanning"})
-[:CONTAINS]->(:Operation {name: "AuditExecution"})
-[:CONTAINS]->(:Operation {name: "FindingManagement"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "PolicyEnforcement"})
-[:APPLIES_TO]->(:Policy)
-[:REQUIRES]->(:Control)
-[:PRODUCES]->(:Compliance)

(:UsageContext {name: "RiskControl"})
-[:APPLIES_TO]->(:Risk)
-[:REQUIRES]->(:Control)
-[:PRODUCES]->(:Mitigation)

(:UsageContext {name: "ComplianceAudit"})
-[:APPLIES_TO]->(:Compliance)
-[:REQUIRES]->(:Audit)
-[:PRODUCES]->(:Finding)
```

## Code References

```cypher
(:CodeBase {path: "notes/governance/readme.md"})
-[:IMPLEMENTS]->(:Policy)

(:CodeBase {path: "notes/governance/compliance.md"})
-[:IMPLEMENTS]->(:Compliance)

(:CodeBase {path: "notes/governance/risk.md"})
-[:IMPLEMENTS]->(:Risk)

(:CodeBase {path: "notes/governance/control.md"})
-[:IMPLEMENTS]->(:Control)
``` 