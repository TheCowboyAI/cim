# Agent Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Agent:Service {
  domain: "Technical",
  term: "Agent",
  definition: "An autonomous entity capable of performing tasks within the CIM",
  taxonomy: "Processing Rules",
  usage_context: "Automated processing and decision making",
  code_reference: "src/agent/agent.md"
})

(:Behavior:ValueObject {
  domain: "Technical",
  term: "Behavior",
  definition: "Defined patterns of action and response for agents",
  taxonomy: "Processing Rules",
  usage_context: "Defining how agents interact with the system",
  code_reference: "src/agent/behavior.md"
})

(:AITool:Service {
  domain: "Technical",
  term: "AITool",
  definition: "Specialized capabilities provided to agents through AI models",
  taxonomy: "Processing Rules",
  usage_context: "Tool-augmented agent operations",
  code_reference: "src/agent/tools.md"
})

(:Policy:Entity {
  domain: "Technical",
  term: "Policy",
  definition: "Constraints and guidelines that govern agent behavior",
  taxonomy: "Processing Rules",
  usage_context: "Agent governance and control",
  code_reference: "src/agent/policy.md"
})

(:Model:Service {
  domain: "Technical",
  term: "Model",
  definition: "AI model that provides intelligence capabilities to agents",
  taxonomy: "Processing Rules",
  usage_context: "Intelligence augmentation for agents",
  code_reference: "src/agent/model.md"
})

(:InformationEntity:Entity {
  domain: "Technical",
  term: "InformationEntity",
  definition: "Units of information processed by agents",
  taxonomy: "Information Taxonomy",
  usage_context: "Data processing within agent workflows",
  code_reference: "src/information/entity.md"
})
```

## Relationships

```cypher
// Agent relationships
(:Agent)-[:USES {type: "utilization"}]->(:AITool)
(:Agent)-[:PROCESSES {type: "operation"}]->(:InformationEntity)
(:Agent)-[:HAS {type: "composition"}]->(:Behavior)

// Behavior relationships
(:Behavior)-[:CONFIGURES {type: "configuration"}]->(:Agent)
(:Behavior)-[:FOLLOWS {type: "compliance"}]->(:Policy)
(:Behavior)-[:USES {type: "utilization"}]->(:Model)

// AITool relationships
(:AITool)-[:INTEGRATES_WITH {type: "integration"}]->(:Model)
(:AITool)-[:ENHANCES {type: "augmentation"}]->(:Agent)

// Policy relationships
(:Policy)-[:CONSTRAINS {type: "governance"}]->(:Behavior)
(:Policy)-[:ENSURES {type: "governance"}]->(:Compliance)

// Model relationships
(:Model)-[:PROVIDES {type: "capability"}]->(:Intelligence)
(:Model)-[:USED_BY {type: "utilization"}]->(:AITool)
```

## Taxonomies

### Agent Processing Taxonomy

```cypher
(:Taxonomy {name: "Processing Rules"})
-[:CONTAINS]->(:Category {name: "Agent Components"})
-[:CONTAINS]->(:Term {name: "Agent"})
-[:CONTAINS]->(:Term {name: "Behavior"})
-[:CONTAINS]->(:Term {name: "AITool"})

(:Taxonomy {name: "Processing Rules"})
-[:CONTAINS]->(:Category {name: "Agent Governance"})
-[:CONTAINS]->(:Term {name: "Policy"})
-[:CONTAINS]->(:Term {name: "Compliance"})

(:Taxonomy {name: "Processing Rules"})
-[:CONTAINS]->(:Category {name: "Intelligence Components"})
-[:CONTAINS]->(:Term {name: "Model"})
-[:CONTAINS]->(:Term {name: "Intelligence"})

(:Taxonomy {name: "Information Taxonomy"})
-[:CONTAINS]->(:Category {name: "Information Types"})
-[:CONTAINS]->(:Term {name: "InformationEntity"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "Automated processing and decision making"})
-[:APPLIES_TO]->(:Agent)
-[:REQUIRES]->(:AITool)
-[:PRODUCES]->(:Decision)

(:UsageContext {name: "Defining how agents interact with the system"})
-[:APPLIES_TO]->(:Behavior)
-[:REQUIRES]->(:Policy)
-[:PRODUCES]->(:Interaction)

(:UsageContext {name: "Tool-augmented agent operations"})
-[:APPLIES_TO]->(:AITool)
-[:REQUIRES]->(:Model)
-[:PRODUCES]->(:AugmentedCapability)

(:UsageContext {name: "Agent governance and control"})
-[:APPLIES_TO]->(:Policy)
-[:REQUIRES]->(:Governance)
-[:PRODUCES]->(:Compliance)

(:UsageContext {name: "Intelligence augmentation for agents"})
-[:APPLIES_TO]->(:Model)
-[:REQUIRES]->(:Training)
-[:PRODUCES]->(:Intelligence)

(:UsageContext {name: "Data processing within agent workflows"})
-[:APPLIES_TO]->(:InformationEntity)
-[:REQUIRES]->(:Agent)
-[:PRODUCES]->(:ProcessedInformation)
```

## Code References

```cypher
(:CodeBase {path: "src/agent/agent.md"})
-[:IMPLEMENTS]->(:Agent)

(:CodeBase {path: "src/agent/behavior.md"})
-[:IMPLEMENTS]->(:Behavior)

(:CodeBase {path: "src/agent/tools.md"})
-[:IMPLEMENTS]->(:AITool)

(:CodeBase {path: "src/agent/policy.md"})
-[:IMPLEMENTS]->(:Policy)

(:CodeBase {path: "src/agent/model.md"})
-[:IMPLEMENTS]->(:Model)

(:CodeBase {path: "src/information/entity.md"})
-[:IMPLEMENTS]->(:InformationEntity)
```

