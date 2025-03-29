# Laws Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Law:Aggregate {
  domain: "Business",
  definition: "A formal rule or regulation imposed by an authority that governs behavior and operations"
})

(:Regulation:Entity {
  domain: "Business",
  definition: "A specific set of requirements and standards established by regulatory bodies"
})

(:Jurisdiction:ValueObject {
  domain: "Business",
  definition: "The legal authority and scope within which laws and regulations apply"
})

(:Requirement:ValueObject {
  domain: "Business",
  definition: "A specific obligation or condition that must be met for legal compliance"
})

(:Authority:Entity {
  domain: "Business",
  definition: "An entity with the power to create, enforce, or oversee laws and regulations"
})

(:Enforcement:Process {
  domain: "Business",
  definition: "The process of ensuring and compelling compliance with laws and regulations"
})

(:Penalty:ValueObject {
  domain: "Business",
  definition: "Consequences or sanctions for non-compliance with laws and regulations"
})

(:Documentation:Service {
  domain: "Business",
  definition: "The system for maintaining and providing evidence of legal compliance"
})
```

## Relationships

```cypher
// Law relationships
(:Law)-[:ENFORCED_BY {type: "authority"}]->(:Authority)
(:Law)-[:APPLIES_TO {type: "scope"}]->(:Jurisdiction)
(:Law)-[:DEFINES {type: "obligation"}]->(:Requirement)
(:Law)-[:IMPOSES {type: "consequence"}]->(:Penalty)
(:Law)-[:REQUIRES {type: "evidence"}]->(:Documentation)

// Regulation relationships
(:Regulation)-[:IMPLEMENTS {type: "law"}]->(:Law)
(:Regulation)-[:OVERSEEN_BY {type: "authority"}]->(:Authority)
(:Regulation)-[:SPECIFIES {type: "requirement"}]->(:Requirement)
(:Regulation)-[:ENFORCED_THROUGH {type: "process"}]->(:Enforcement)

// Requirement relationships
(:Requirement)-[:PART_OF {type: "regulation"}]->(:Regulation)
(:Requirement)-[:VERIFIED_BY {type: "process"}]->(:Enforcement)
(:Requirement)-[:DOCUMENTED_IN {type: "evidence"}]->(:Documentation)
(:Requirement)-[:HAS {type: "consequence"}]->(:Penalty)

// Enforcement relationships
(:Enforcement)-[:CONDUCTED_BY {type: "authority"}]->(:Authority)
(:Enforcement)-[:VERIFIES {type: "compliance"}]->(:Requirement)
(:Enforcement)-[:APPLIES {type: "penalty"}]->(:Penalty)
(:Enforcement)-[:REQUIRES {type: "evidence"}]->(:Documentation)
```

## Taxonomies

### Legal Processing

```cypher
(:Taxonomy {name: "LegalProcessing"})
-[:CONTAINS]->(:Category {name: "LegalOperations"})
-[:CONTAINS]->(:Operation {name: "ComplianceVerification"})
-[:CONTAINS]->(:Operation {name: "RequirementManagement"})
-[:CONTAINS]->(:Operation {name: "EnforcementExecution"})

(:Category {name: "DocumentationManagement"})
-[:CONTAINS]->(:Operation {name: "EvidenceCollection"})
-[:CONTAINS]->(:Operation {name: "RecordKeeping"})
-[:CONTAINS]->(:Operation {name: "AuditPreparation"})

(:Category {name: "JurisdictionManagement"})
-[:CONTAINS]->(:Operation {name: "ScopeDefinition"})
-[:CONTAINS]->(:Operation {name: "AuthorityValidation"})
-[:CONTAINS]->(:Operation {name: "ConflictResolution"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "ComplianceManagement"})
-[:APPLIES_TO]->(:Law)
-[:REQUIRES]->(:Enforcement)
-[:PRODUCES]->(:Documentation)

(:UsageContext {name: "RequirementValidation"})
-[:APPLIES_TO]->(:Requirement)
-[:REQUIRES]->(:Documentation)
-[:PRODUCES]->(:Verification)

(:UsageContext {name: "EnforcementAction"})
-[:APPLIES_TO]->(:Enforcement)
-[:REQUIRES]->(:Authority)
-[:PRODUCES]->(:Penalty)
```

## Code References

```cypher
(:CodeBase {path: "notes/laws/readme.md"})
-[:IMPLEMENTS]->(:Law)

(:CodeBase {path: "notes/laws/regulations.md"})
-[:IMPLEMENTS]->(:Regulation)

(:CodeBase {path: "notes/laws/requirements.md"})
-[:IMPLEMENTS]->(:Requirement)

(:CodeBase {path: "notes/laws/enforcement.md"})
-[:IMPLEMENTS]->(:Enforcement)
``` 