# Organization Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Operator:Entity {
  domain: "Business",
  term: "Operator",
  definition: "Organization responsible for operating a CIM instance",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Primary administrative entity for CIM operations",
  code_reference: "src/organization/operator.md"
})

(:Account:Entity {
  domain: "Business",
  term: "Account",
  definition: "A group or individual identity within the CIM system",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Access control and resource management",
  code_reference: "src/organization/account.md"
})

(:Tenant:Entity {
  domain: "Business",
  term: "Tenant",
  definition: "An organization that uses the CIM instance",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Multi-tenancy and resource isolation",
  code_reference: "src/organization/tenant.md"
})

(:Policy:Entity {
  domain: "Business",
  term: "Policy",
  definition: "Rules and constraints governing CIM operations",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Governance and compliance",
  code_reference: "src/organization/policy.md"
})

(:User:Entity {
  domain: "Business",
  term: "User",
  definition: "Individual with access to the CIM system",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Authentication and authorization",
  code_reference: "src/organization/user.md"
})

(:Permission:Entity {
  domain: "Business",
  term: "Permission",
  definition: "Access rights granted to accounts and users",
  taxonomy: "Security Taxonomy",
  usage_context: "Access control management",
  code_reference: "src/organization/permission.md"
})
```

## Relationships

```cypher
// Operator relationships
(:Operator)-[:MANAGES {type: "administration"}]->(:Tenant)
(:Operator)-[:CONFIGURES {type: "administration"}]->(:Policy)
(:Operator)-[:CONTAINS {type: "composition"}]->(:Account)

// Account relationships
(:Account)-[:PART_OF {type: "composition"}]->(:Operator)
(:Account)-[:CONTAINS {type: "composition"}]->(:User)
(:Account)-[:HAS {type: "association"}]->(:Permission)

// User relationships
(:User)-[:BELONGS_TO {type: "association"}]->(:Account)
(:User)-[:ASSIGNED {type: "association"}]->(:Permission)

// Permission relationships
(:Permission)-[:CONTROLS_ACCESS_TO {type: "security"}]->(:Resource)
```

## Taxonomies

### Configuration Taxonomy

```cypher
(:Taxonomy {name: "Configuration Taxonomy"})
-[:CONTAINS]->(:Category {name: "Organization Entities"})
-[:CONTAINS]->(:Term {name: "Operator"})
-[:CONTAINS]->(:Term {name: "Account"})
-[:CONTAINS]->(:Term {name: "Tenant"})

(:Taxonomy {name: "Configuration Taxonomy"})
-[:CONTAINS]->(:Category {name: "Access Control"})
-[:CONTAINS]->(:Term {name: "User"})
-[:CONTAINS]->(:Term {name: "Policy"})

(:Taxonomy {name: "Security Taxonomy"})
-[:CONTAINS]->(:Category {name: "Access Rights"})
-[:CONTAINS]->(:Term {name: "Permission"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "Primary administrative entity for CIM operations"})
-[:APPLIES_TO]->(:Operator)
-[:REQUIRES]->(:Policy)
-[:PRODUCES]->(:Tenant)

(:UsageContext {name: "Access control and resource management"})
-[:APPLIES_TO]->(:Account)
-[:REQUIRES]->(:User)
-[:PRODUCES]->(:Permission)

(:UsageContext {name: "Multi-tenancy and resource isolation"})
-[:APPLIES_TO]->(:Tenant)
-[:REQUIRES]->(:Operator)
-[:PRODUCES]->(:Resource)

(:UsageContext {name: "Authentication and authorization"})
-[:APPLIES_TO]->(:User)
-[:REQUIRES]->(:Account)
-[:PRODUCES]->(:Permission)

(:UsageContext {name: "Access control management"})
-[:APPLIES_TO]->(:Permission)
-[:REQUIRES]->(:Policy)
-[:PRODUCES]->(:AccessDecision)

(:UsageContext {name: "Governance and compliance"})
-[:APPLIES_TO]->(:Policy)
-[:REQUIRES]->(:Operator)
-[:PRODUCES]->(:Rule)
```

## Code References

```cypher
(:CodeBase {path: "src/organization/operator.md"})
-[:IMPLEMENTS]->(:Operator)

(:CodeBase {path: "src/organization/account.md"})
-[:IMPLEMENTS]->(:Account)

(:CodeBase {path: "src/organization/tenant.md"})
-[:IMPLEMENTS]->(:Tenant)

(:CodeBase {path: "src/organization/policy.md"})
-[:IMPLEMENTS]->(:Policy)

(:CodeBase {path: "src/organization/user.md"})
-[:IMPLEMENTS]->(:User)

(:CodeBase {path: "src/organization/permission.md"})
-[:IMPLEMENTS]->(:Permission)
```

