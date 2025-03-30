# Security_domain Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Example:Aggregate {
  domain: "Security",
  term: "Example",
  definition: "Definition goes here",
  taxonomy: "ExampleTaxonomy",
  usage_context: "ExampleContext",
  code_reference: "path/to/example.md"
})

// Add more nodes here
```

## Relationships

```cypher
// Example relationships
(:Example)-[:RELATES_TO {type: "relationship"}]->(:AnotherExample)
(:Example)-[:CONTAINS {type: "composition"}]->(:Component)

// Add more relationships here
```

## Taxonomies

### Example Processing

```cypher
(:Taxonomy {name: "ExampleProcessing"})
-[:CONTAINS]->(:Category {name: "ExampleCategory"})
-[:CONTAINS]->(:Operation {name: "ExampleOperation"})
-[:CONTAINS]->(:Operation {name: "AnotherOperation"})

(:Category {name: "AnotherCategory"})
-[:CONTAINS]->(:Operation {name: "ThirdOperation"})
-[:CONTAINS]->(:Operation {name: "FourthOperation"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "ExampleContext"})
-[:APPLIES_TO]->(:Example)
-[:REQUIRES]->(:Component)
-[:PRODUCES]->(:Output)

// Add more usage contexts here
```

## Code References

```cypher
(:CodeBase {path: "notes/security_domain/readme.md"})
-[:IMPLEMENTS]->(:Example)

// Add more code references here
```

