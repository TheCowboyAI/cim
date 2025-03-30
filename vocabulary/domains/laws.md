# Laws Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Law:Aggregate {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "A formal rule or regulation imposed by an authority that governs behavior and operations"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Regulation:Entity {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "A specific set of requirements and standards established by regulatory bodies"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Jurisdiction:ValueObject {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "The legal authority and scope within which laws and regulations apply"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Requirement:ValueObject {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "A specific obligation or condition that must be met for legal compliance"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Authority:Entity {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "An entity with the power to create, enforce, or oversee laws and regulations"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Enforcement:Process {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "The process of ensuring and compelling compliance with laws and regulations"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Penalty:ValueObject {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "Consequences or sanctions for non-compliance with laws and regulations"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
})

(:Documentation:Service {
  domain: "Business",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Enforcement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Requirement)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Regulation)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Law)-[",
  term: "Documentation",
  term: "Penalty",
  term: "Enforcement",
  term: "Authority",
  term: "Requirement",
  term: "Jurisdiction",
  term: "Regulation",
  term: "Law",
  definition: "The system for maintaining and providing evidence of legal compliance"
  taxonomy: "Business Taxonomy",
  usage_context: "Business related operations and processes",
  code_reference: "TBD",
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
  code_reference: "TBD",
(:Taxonomy {name: "LegalProcessing"})
  code_reference: "TBD",
-[:CONTAINS]->(:Category {name: "LegalOperations"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ComplianceVerification"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "RequirementManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "EnforcementExecution"})

  code_reference: "TBD",
(:Category {name: "DocumentationManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "EvidenceCollection"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "RecordKeeping"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "AuditPreparation"})

  code_reference: "TBD",
(:Category {name: "JurisdictionManagement"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ScopeDefinition"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "AuthorityValidation"})
  code_reference: "TBD",
-[:CONTAINS]->(:Operation {name: "ConflictResolution"})
```

## Usage Contexts

```cypher
  code_reference: "TBD",
(:UsageContext {name: "ComplianceManagement"})
-[:APPLIES_TO]->(:Law)
-[:REQUIRES]->(:Enforcement)
-[:PRODUCES]->(:Documentation)

  code_reference: "TBD",
(:UsageContext {name: "RequirementValidation"})
-[:APPLIES_TO]->(:Requirement)
-[:REQUIRES]->(:Documentation)
-[:PRODUCES]->(:Verification)

  code_reference: "TBD",
(:UsageContext {name: "EnforcementAction"})
-[:APPLIES_TO]->(:Enforcement)
-[:REQUIRES]->(:Authority)
-[:PRODUCES]->(:Penalty)
```

## Code References

```cypher
  code_reference: "TBD",
(:CodeBase {path: "notes/laws/readme.md"})
-[:IMPLEMENTS]->(:Law)

  code_reference: "TBD",
(:CodeBase {path: "notes/laws/regulations.md"})
-[:IMPLEMENTS]->(:Regulation)

  code_reference: "TBD",
(:CodeBase {path: "notes/laws/requirements.md"})
-[:IMPLEMENTS]->(:Requirement)

  code_reference: "TBD",
(:CodeBase {path: "notes/laws/enforcement.md"})
-[:IMPLEMENTS]->(:Enforcement)
``` 