# Architecture Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Entity:ValueObject {
  domain: "Architecture",
  term: "Entity",
  taxonomy: "ECS Components",
  definition: "A unique identifier that represents a distinct object or concept in the ECS architecture, serving as a container for components.",
  usage_context: "Data representation in ECS",
  code_reference: "cim/src/ecs/entity"
})

(:Component:ValueObject {
  domain: "Architecture",
  term: "Component",
  taxonomy: "ECS Components",
  definition: "A data structure that defines properties or attributes that can be attached to entities in the ECS architecture.",
  usage_context: "Property definition in ECS",
  code_reference: "cim/src/ecs/component"
})

(:System:Service {
  domain: "Architecture",
  term: "System",
  taxonomy: "ECS Components",
  definition: "A logic unit in the ECS architecture that processes entities with specific component combinations to implement behavior.",
  usage_context: "Logic implementation in ECS",
  code_reference: "cim/src/ecs/system"
})
```

## Relationships

```cypher
// Entity relationships
(:Entity)-[:CONTAINS {type: "structure"}]->(:Component)
(:Entity)-[:PART_OF {type: "architecture"}]->(:ECS)
(:System)-[:PROCESSES {type: "data"}]->(:Entity)

// Component relationships
(:Component)-[:ATTACHED_TO {type: "container"}]->(:Entity)
(:Component)-[:PART_OF {type: "architecture"}]->(:ECS)
(:System)-[:PROCESSES {type: "data"}]->(:Component)

// System relationships
(:System)-[:PROCESSES {type: "structure"}]->(:Entity)
(:System)-[:PROCESSES {type: "data"}]->(:Component)
(:System)-[:PART_OF {type: "architecture"}]->(:ECS)
(:System)-[:IMPLEMENTS {type: "functionality"}]->(:BusinessLogic)
```

## Taxonomies

### ECS Architecture Taxonomy

```cypher
(:Taxonomy {name: "ECS Architecture"})
-[:CONTAINS]->(:Category {name: "ECS Components"})
-[:CONTAINS]->(:Category {name: "ECS Patterns"})

(:Category {name: "ECS Components"})
-[:CONTAINS]->(:Term {name: "Entity"})
-[:CONTAINS]->(:Term {name: "Component"})
-[:CONTAINS]->(:Term {name: "System"})

(:Category {name: "ECS Patterns"})
-[:CONTAINS]->(:Term {name: "ComponentBased"})
-[:CONTAINS]->(:Term {name: "EntityManagement"})
-[:CONTAINS]->(:Term {name: "SystemProcessing"})
```

### Architecture Patterns

```cypher
(:Taxonomy {name: "Architecture Patterns"})
-[:CONTAINS]->(:Category {name: "Data Patterns"})
-[:CONTAINS]->(:Category {name: "Processing Patterns"})

(:Category {name: "Data Patterns"})
-[:CONTAINS]->(:Term {name: "Entity"})
-[:CONTAINS]->(:Term {name: "Component"})

(:Category {name: "Processing Patterns"})
-[:CONTAINS]->(:Term {name: "System"})
-[:CONTAINS]->(:Term {name: "BusinessLogic"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "DataRepresentation"})
-[:APPLIES_TO]->(:Entity)
-[:REQUIRES]->(:Component)
-[:PRODUCES]->(:DataModel)

(:UsageContext {name: "PropertyDefinition"})
-[:APPLIES_TO]->(:Component)
-[:REQUIRES]->(:DataSchema)
-[:PRODUCES]->(:EntityAttribute)

(:UsageContext {name: "LogicImplementation"})
-[:APPLIES_TO]->(:System)
-[:REQUIRES]->(:Entity)
-[:REQUIRES]->(:Component)
-[:PRODUCES]->(:BusinessLogic)
```

## Code References

```cypher
(:CodeBase {path: "cim/src/ecs/entity"})
-[:IMPLEMENTS]->(:Entity)

(:CodeBase {path: "cim/src/ecs/component"})
-[:IMPLEMENTS]->(:Component)

(:CodeBase {path: "cim/src/ecs/system"})
-[:IMPLEMENTS]->(:System)
``` 