# Governance Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Policy:Aggregate {
  domain: "Business",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Decision",
  term: "Audit",
  term: "Control",
  term: "Risk",
  term: "Compliance",
  term: "Policy",
  definition: "A defined set of rules, guidelines, and controls governing system behavior and operations"
  taxonomy: "Governance Taxonomy",
  usage_context: "Governance related operations and processes",
  code_reference: "TBD",
})

(:Compliance:Process {
  domain: "Business",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Decision",
  term: "Audit",
  term: "Control",
  term: "Risk",
  term: "Compliance",
  term: "Policy",
  definition: "The process of ensuring adherence to policies, laws, and regulations"
  taxonomy: "Governance Taxonomy",
  usage_context: "Governance related operations and processes",
  code_reference: "TBD",
})

(:Risk:ValueObject {
  domain: "Business",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Decision",
  term: "Audit",
  term: "Control",
  term: "Risk",
  term: "Compliance",
  term: "Policy",
  definition: "A measured potential for adverse effects on system operations or objectives"
  taxonomy: "Governance Taxonomy",
  usage_context: "Governance related operations and processes",
  code_reference: "TBD",
})

(:Control:Service {
  domain: "Business",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Decision",
  term: "Audit",
  term: "Control",
  term: "Risk",
  term: "Compliance",
  term: "Policy",
  definition: "A mechanism for enforcing policies and managing risk within the system"
  taxonomy: "Governance Taxonomy",
  usage_context: "Governance related operations and processes",
  code_reference: "TBD",
})

(:Audit:Process {
  domain: "Business",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Decision",
  term: "Audit",
  term: "Control",
  term: "Risk",
  term: "Compliance",
  term: "Policy",
  definition: "The systematic review and verification of system compliance and control effectiveness"
  taxonomy: "Governance Taxonomy",
  usage_context: "Governance related operations and processes",
  code_reference: "TBD",
})

(:Decision:Entity {
  domain: "Business",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Control)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Risk)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Compliance)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Policy)-[",
  term: "Decision",
  term: "Audit",
  term: "Control",
  term: "Risk",
  term: "Compliance",
  term: "Policy",
  definition: "A formal choice or determination made within governance frameworks"
  taxonomy: "Governance Taxonomy",
  usage_context: "Governance related operations and processes",
  code_reference: "TBD",
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
  code_reference: "TBD",
(:Taxonomy {name: "GovernanceProcessing"})
  code_reference: "TBD",
-[:CONTAINS]->(:Category {name: "PolicyOperations"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "PolicyDefinition"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ComplianceMonitoring"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "RiskManagement"})

  code_reference: "TBD",
(:Category {name: "ControlManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ControlImplementation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ControlMonitoring"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ControlOptimization"})

  code_reference: "TBD",
(:Category {name: "AuditProcessing"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "AuditPlanning"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "AuditExecution"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "FindingManagement"})
```

## Usage Contexts

```cypher
  code_reference: "TBD",
(:UsageContext {name: "PolicyEnforcement"})
-[:APPLIES_TO]->(:Policy)
-[:REQUIRES]->(:Control)
-[:PRODUCES]->(:Compliance)

  code_reference: "TBD",
(:UsageContext {name: "RiskControl"})
-[:APPLIES_TO]->(:Risk)
-[:REQUIRES]->(:Control)
-[:PRODUCES]->(:Mitigation)

  code_reference: "TBD",
(:UsageContext {name: "ComplianceAudit"})
-[:APPLIES_TO]->(:Compliance)
-[:REQUIRES]->(:Audit)
-[:PRODUCES]->(:Finding)
```

## Code References

```cypher
  code_reference: "TBD",
(:CodeBase {path: "notes/governance/readme.md"})
-[:IMPLEMENTS]->(:Policy)

  code_reference: "TBD",
(:CodeBase {path: "notes/governance/compliance.md"})
-[:IMPLEMENTS]->(:Compliance)

  code_reference: "TBD",
(:CodeBase {path: "notes/governance/risk.md"})
-[:IMPLEMENTS]->(:Risk)

  code_reference: "TBD",
(:CodeBase {path: "notes/governance/control.md"})
-[:IMPLEMENTS]->(:Control)
``` 