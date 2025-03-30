# Domain Configuration Vocabulary

## Domain Objects

### Nodes

```cypher
(:Domain:Aggregate {
  domain: "DomainConfiguration",
  term: "Domain",
  definition: "A bounded context representing a specific business area with its own ubiquitous language, rules, and processes.",
  taxonomy: "Core Concepts",
  usage_context: "Business modeling and system organization",
  code_reference: "cim/src/domain"
})

(:Subdomain:Entity {
  domain: "DomainConfiguration",
  term: "Subdomain",
  definition: "A distinct part of the overall domain, categorized as Core, Supporting, or Generic based on business value.",
  taxonomy: "Domain Organization",
  usage_context: "Domain partitioning and organization",
  code_reference: "cim/src/domain/subdomain"
})

(:BoundedContext:Aggregate {
  domain: "DomainConfiguration",
  term: "BoundedContext",
  definition: "A logical boundary that separates different parts of the system where terms and concepts may have different meanings.",
  taxonomy: "Domain Organization",
  usage_context: "Domain model organization and context separation",
  code_reference: "cim/src/domain/context"
})

(:DomainModel:Aggregate {
  domain: "DomainConfiguration",
  term: "DomainModel",
  definition: "A conceptual model representing the key concepts, relationships, and behaviors within a domain.",
  taxonomy: "Domain Organization",
  usage_context: "Domain representation and business logic implementation",
  code_reference: "cim/src/domain/model"
})

(:NixFlake:Configuration {
  domain: "DomainConfiguration",
  term: "NixFlake",
  definition: "A standardized format for packaging Nix expressions with dependencies and metadata for reproducible builds.",
  taxonomy: "System Configuration",
  usage_context: "System configuration and deployment",
  code_reference: "cim/src/config/flake"
})

(:Configuration:ValueObject {
  domain: "DomainConfiguration",
  term: "Configuration",
  definition: "A collection of settings and parameters that define how a system component or service should operate.",
  taxonomy: "System Management",
  usage_context: "System setup and management",
  code_reference: "cim/src/config"
})

(:DomainEvent:ValueObject {
  domain: "DomainConfiguration",
  term: "DomainEvent",
  definition: "A record of something significant that happened within the domain, stored immutably in the event store.",
  taxonomy: "Communication",
  usage_context: "Event sourcing and state management",
  code_reference: "cim/src/domain/event"
})

(:DomainCommand:ValueObject {
  domain: "DomainConfiguration",
  term: "DomainCommand",
  definition: "A request to perform an action that will change the state of the domain.",
  taxonomy: "Communication",
  usage_context: "State modification and business operations",
  code_reference: "cim/src/domain/command"
})

(:DomainQuery:ValueObject {
  domain: "DomainConfiguration",
  term: "DomainQuery",
  definition: "A request to retrieve information from the domain without modifying state.",
  taxonomy: "Communication",
  usage_context: "State retrieval and data access",
  code_reference: "cim/src/domain/query"
})

(:DomainModeling:Process {
  domain: "DomainConfiguration",
  term: "DomainModeling",
  definition: "The process of identifying, analyzing, and structuring domain concepts and their relationships to create effective domain models.",
  taxonomy: "Domain Organization",
  usage_context: "Domain analysis and design",
  code_reference: "cim/src/domain/modeling"
})

(:ConfigurationManagement:Service {
  domain: "DomainConfiguration",
  term: "ConfigurationManagement",
  definition: "A system service responsible for managing, validating, and applying configuration changes across the system.",
  taxonomy: "System Management",
  usage_context: "System configuration and maintenance",
  code_reference: "cim/src/config/management"
})

(:DomainEventProcessing:Service {
  domain: "DomainConfiguration",
  term: "DomainEventProcessing",
  definition: "A service responsible for processing and managing the lifecycle of domain events, commands, and queries.",
  taxonomy: "Communication",
  usage_context: "Domain event handling and processing",
  code_reference: "cim/src/domain/processing"
})
```

## Relationships

```cypher
// Domain relationships
(:Domain)-[:CONTAINS {type: "structure"}]->(:Entities)
(:Domain)-[:CONTAINS {type: "structure"}]->(:Values)
(:Domain)-[:CONTAINS {type: "structure"}]->(:Policies)
(:Domain)-[:DEFINES {type: "language"}]->(:UbiquitousLanguage)
(:Domain)-[:IMPLEMENTS {type: "logic"}]->(:BusinessLogic)

// Subdomain relationships
(:Subdomain)-[:PART_OF {type: "structure"}]->(:Domain)
(:Subdomain)-[:CONTAINS {type: "logic"}]->(:DomainLogic)
(:Subdomain)-[:IMPLEMENTS {type: "rules"}]->(:BusinessRules)

// BoundedContext relationships
(:BoundedContext)-[:CONTAINS {type: "model"}]->(:DomainModel)
(:BoundedContext)-[:DEFINES {type: "boundary"}]->(:ContextBoundaries)
(:BoundedContext)-[:IMPLEMENTS {type: "separation"}]->(:DomainSeparation)

// DomainModel relationships
(:DomainModel)-[:CONTAINS {type: "structure"}]->(:Entities)
(:DomainModel)-[:CONTAINS {type: "structure"}]->(:ValueObjects)
(:DomainModel)-[:PART_OF {type: "context"}]->(:BoundedContext)
(:DomainModel)-[:IMPLEMENTS {type: "rules"}]->(:BusinessRules)

// NixFlake relationships
(:NixFlake)-[:CONTAINS {type: "package"}]->(:Dependencies)
(:NixFlake)-[:CONTAINS {type: "package"}]->(:Outputs)
(:NixFlake)-[:IMPLEMENTS {type: "management"}]->(:ConfigurationManagement)
(:NixFlake)-[:USED_BY {type: "system"}]->(:CIM)

// Configuration relationships
(:Configuration)-[:PART_OF {type: "structure"}]->(:Domain)
(:Configuration)-[:DEFINES {type: "behavior"}]->(:SystemBehavior)
(:Configuration)-[:USED_BY {type: "system"}]->(:Components)

// DomainEvent relationships
(:DomainEvent)-[:PART_OF {type: "storage"}]->(:EventStore)
(:DomainEvent)-[:RECORDS {type: "change"}]->(:StateChanges)
(:DomainEvent)-[:USED_BY {type: "model"}]->(:DomainModel)

// DomainCommand relationships
(:DomainCommand)-[:PRODUCES {type: "event"}]->(:DomainEvent)
(:DomainCommand)-[:CHANGES {type: "state"}]->(:DomainState)
(:DomainCommand)-[:USED_BY {type: "model"}]->(:DomainModel)

// DomainQuery relationships
(:DomainQuery)-[:RETRIEVES {type: "state"}]->(:DomainState)
(:DomainQuery)-[:USED_BY {type: "model"}]->(:DomainModel)
(:DomainQuery)-[:PRODUCES {type: "view"}]->(:Projections)

// DomainModeling relationships
(:DomainModeling)-[:PRODUCES {type: "model"}]->(:DomainModel)
(:DomainModeling)-[:USES {type: "context"}]->(:BoundedContext)
(:DomainModeling)-[:IMPLEMENTS {type: "requirements"}]->(:BusinessRequirements)

// ConfigurationManagement relationships
(:ConfigurationManagement)-[:MANAGES {type: "config"}]->(:Configuration)
(:ConfigurationManagement)-[:USES {type: "package"}]->(:NixFlake)
(:ConfigurationManagement)-[:IMPLEMENTS {type: "setup"}]->(:SystemSetup)

// DomainEventProcessing relationships
(:DomainEventProcessing)-[:PROCESSES {type: "event"}]->(:DomainEvent)
(:DomainEventProcessing)-[:PROCESSES {type: "command"}]->(:DomainCommand)
(:DomainEventProcessing)-[:PROCESSES {type: "query"}]->(:DomainQuery)
(:DomainEventProcessing)-[:IMPLEMENTS {type: "pipeline"}]->(:EventProcessingPipeline)
(:DomainEventProcessing)-[:USES {type: "storage"}]->(:EventStore)
```

## Taxonomies

### Domain Organization Taxonomy

```cypher
(:Taxonomy {name: "DomainOrganizationTaxonomy"})
-[:CONTAINS]->(:Category {name: "CoreConcepts"})
-[:CONTAINS]->(:Operation {name: "DomainModeling"})
-[:CONTAINS]->(:Operation {name: "BoundedContextMapping"})
-[:CONTAINS]->(:Operation {name: "SubdomainDefinition"})

(:Category {name: "EventProcessing"})
-[:CONTAINS]->(:Operation {name: "EventHandling"})
-[:CONTAINS]->(:Operation {name: "CommandProcessing"})
-[:CONTAINS]->(:Operation {name: "QueryHandling"})
-[:CONTAINS]->(:Operation {name: "ProjectionBuilding"})

(:Category {name: "ConfigurationManagement"})
-[:CONTAINS]->(:Operation {name: "SystemConfigurationDefinition"})
-[:CONTAINS]->(:Operation {name: "ConfigurationValidation"})
-[:CONTAINS]->(:Operation {name: "ConfigurationDeployment"})
-[:CONTAINS]->(:Operation {name: "ConfigurationVersioning"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "DomainModelingContext"})
-[:APPLIES_TO]->(:DomainModel)
-[:REQUIRES]->(:BoundedContext)
-[:PRODUCES]->(:StructuredDomain)

(:UsageContext {name: "EventSourcedArchitecture"})
-[:APPLIES_TO]->(:DomainEvent)
-[:REQUIRES]->(:EventStore)
-[:PRODUCES]->(:ConsistentState)

(:UsageContext {name: "SystemConfiguration"})
-[:APPLIES_TO]->(:Configuration)
-[:REQUIRES]->(:NixFlake)
-[:PRODUCES]->(:ConfiguredSystem)

(:UsageContext {name: "CommandQuerySeparation"})
-[:APPLIES_TO]->(:DomainCommand)
-[:APPLIES_TO]->(:DomainQuery)
-[:REQUIRES]->(:DomainEventProcessing)
-[:PRODUCES]->(:RespectfulArchitecture)
```

## Code References

```cypher
(:CodeBase {path: "cim/src/domain"})
-[:IMPLEMENTS]->(:Domain)

(:CodeBase {path: "cim/src/domain/subdomain"})
-[:IMPLEMENTS]->(:Subdomain)

(:CodeBase {path: "cim/src/domain/context"})
-[:IMPLEMENTS]->(:BoundedContext)

(:CodeBase {path: "cim/src/domain/model"})
-[:IMPLEMENTS]->(:DomainModel)

(:CodeBase {path: "cim/src/config/flake"})
-[:IMPLEMENTS]->(:NixFlake)

(:CodeBase {path: "cim/src/config"})
-[:IMPLEMENTS]->(:Configuration)

(:CodeBase {path: "cim/src/domain/event"})
-[:IMPLEMENTS]->(:DomainEvent)

(:CodeBase {path: "cim/src/domain/command"})
-[:IMPLEMENTS]->(:DomainCommand)

(:CodeBase {path: "cim/src/domain/query"})
-[:IMPLEMENTS]->(:DomainQuery)

(:CodeBase {path: "cim/src/domain/modeling"})
-[:IMPLEMENTS]->(:DomainModeling)

(:CodeBase {path: "cim/src/config/management"})
-[:IMPLEMENTS]->(:ConfigurationManagement)

(:CodeBase {path: "cim/src/domain/processing"})
-[:IMPLEMENTS]->(:DomainEventProcessing)
``` 