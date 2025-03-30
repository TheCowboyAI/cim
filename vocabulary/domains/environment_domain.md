# Environment Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Equipment:Entity {
  domain: "Technical",
  term: "Equipment",
  definition: "Physical or virtual resources used by the CIM",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Resource management and deployment",
  code_reference: "src/environment/equipment.md"
})

(:Location:ValueObject {
  domain: "Technical",
  term: "Location",
  definition: "Physical or logical placement of CIM components",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Geographic and logical resource organization",
  code_reference: "src/environment/location.md"
})

(:Environment:Entity {
  domain: "Technical",
  term: "Environment",
  definition: "Operational context and runtime conditions for CIM components",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Deployment and operational management",
  code_reference: "src/environment/environment.md"
})

(:Preference:ValueObject {
  domain: "Technical",
  term: "Preference",
  definition: "Configuration settings for equipment in specific environments",
  taxonomy: "Configuration Taxonomy",
  usage_context: "Equipment customization and optimization",
  code_reference: "src/environment/preference.md"
})

(:Policy:Entity {
  domain: "Technical",
  term: "Policy",
  definition: "Rules governing resource usage and deployment",
  taxonomy: "Governance Taxonomy",
  usage_context: "Environment compliance and management",
  code_reference: "src/environment/policy.md"
})

(:Solution:Entity {
  domain: "Technical",
  term: "Solution",
  definition: "Functional applications built on the CIM platform",
  taxonomy: "Application Taxonomy",
  usage_context: "End-user applications and services",
  code_reference: "src/environment/solution.md"
})
```

## Relationships

```cypher
// Equipment relationships
(:Equipment)-[:LOCATED_IN {type: "placement"}]->(:Environment)
(:Equipment)-[:SUPPORTS {type: "capability"}]->(:Solution)
(:Equipment)-[:HAS {type: "configuration"}]->(:Preference)
(:Equipment)-[:DEPLOYED_AT {type: "location"}]->(:Location)

// Location relationships
(:Location)-[:CONTAINS {type: "composition"}]->(:Equipment)
(:Location)-[:FOLLOWS {type: "compliance"}]->(:Policy)
(:Location)-[:HAS {type: "association"}]->(:Environment)
(:Location)-[:PROVIDES {type: "capability"}]->(:Resource)

// Environment relationships
(:Environment)-[:HOSTS {type: "container"}]->(:Equipment)
(:Environment)-[:GOVERNED_BY {type: "regulation"}]->(:Policy)
(:Environment)-[:SUPPORTS {type: "capability"}]->(:Solution)

// Policy relationships
(:Policy)-[:APPLIES_TO {type: "governance"}]->(:Environment)
(:Policy)-[:REGULATES {type: "control"}]->(:Equipment)
(:Policy)-[:ENFORCES {type: "compliance"}]->(:Standard)

// Solution relationships
(:Solution)-[:REQUIRES {type: "dependency"}]->(:Equipment)
(:Solution)-[:DEPLOYED_IN {type: "container"}]->(:Environment)
(:Solution)-[:ACCESSED_FROM {type: "location"}]->(:Location)

// Preference relationships
(:Preference)-[:CONFIGURES {type: "customization"}]->(:Equipment)
(:Preference)-[:APPLIES_TO {type: "context"}]->(:Environment)
```

## Taxonomies

### Environment Configuration Taxonomy

```cypher
(:Taxonomy {name: "Configuration Taxonomy"})
-[:CONTAINS]->(:Category {name: "Physical Resources"})
-[:CONTAINS]->(:Term {name: "Equipment"})
-[:CONTAINS]->(:Term {name: "Hardware"})

(:Taxonomy {name: "Configuration Taxonomy"})
-[:CONTAINS]->(:Category {name: "Logical Resources"})
-[:CONTAINS]->(:Term {name: "Environment"})
-[:CONTAINS]->(:Term {name: "Location"})

(:Taxonomy {name: "Configuration Taxonomy"})
-[:CONTAINS]->(:Category {name: "Settings"})
-[:CONTAINS]->(:Term {name: "Preference"})
-[:CONTAINS]->(:Term {name: "Configuration"})

(:Taxonomy {name: "Governance Taxonomy"})
-[:CONTAINS]->(:Category {name: "Rules"})
-[:CONTAINS]->(:Term {name: "Policy"})
-[:CONTAINS]->(:Term {name: "Standard"})

(:Taxonomy {name: "Application Taxonomy"})
-[:CONTAINS]->(:Category {name: "End User Applications"})
-[:CONTAINS]->(:Term {name: "Solution"})
-[:CONTAINS]->(:Term {name: "Service"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "Resource management and deployment"})
-[:APPLIES_TO]->(:Equipment)
-[:REQUIRES]->(:Environment)
-[:PRODUCES]->(:DeployedResource)

(:UsageContext {name: "Geographic and logical resource organization"})
-[:APPLIES_TO]->(:Location)
-[:REQUIRES]->(:Policy)
-[:PRODUCES]->(:ResourceLayout)

(:UsageContext {name: "Deployment and operational management"})
-[:APPLIES_TO]->(:Environment)
-[:REQUIRES]->(:Policy)
-[:PRODUCES]->(:OperationalState)

(:UsageContext {name: "Equipment customization and optimization"})
-[:APPLIES_TO]->(:Preference)
-[:REQUIRES]->(:Equipment)
-[:PRODUCES]->(:OptimizedConfiguration)

(:UsageContext {name: "Environment compliance and management"})
-[:APPLIES_TO]->(:Policy)
-[:REQUIRES]->(:Standard)
-[:PRODUCES]->(:ComplianceReport)

(:UsageContext {name: "End-user applications and services"})
-[:APPLIES_TO]->(:Solution)
-[:REQUIRES]->(:Equipment)
-[:PRODUCES]->(:UserService)
```

## Code References

```cypher
(:CodeBase {path: "src/environment/equipment.md"})
-[:IMPLEMENTS]->(:Equipment)

(:CodeBase {path: "src/environment/location.md"})
-[:IMPLEMENTS]->(:Location)

(:CodeBase {path: "src/environment/environment.md"})
-[:IMPLEMENTS]->(:Environment)

(:CodeBase {path: "src/environment/preference.md"})
-[:IMPLEMENTS]->(:Preference)

(:CodeBase {path: "src/environment/policy.md"})
-[:IMPLEMENTS]->(:Policy)

(:CodeBase {path: "src/environment/solution.md"})
-[:IMPLEMENTS]->(:Solution)
```

