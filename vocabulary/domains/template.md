# [Domain Name] Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:[TermName]:[TypeName] {
  domain: "[Domain]",
  term: "[TermName]",
  taxonomy: "[Taxonomy Name]",
  definition: "[Clear definition of what this term represents]",
  usage_context: "[Where/how this term is used]",
  code_reference: "[Path to related code or docs]"
})

(:[AnotherTerm]:[TypeName] {
  domain: "[Domain]",
  term: "[AnotherTerm]",
  taxonomy: "[Taxonomy Name]",
  definition: "[Clear definition of what this term represents]",
  usage_context: "[Where/how this term is used]",
  code_reference: "[Path to related code or docs]"
})

// Add more terms as needed
```

## Relationships

```cypher
// Primary relationships
(:[TermName])-[:RELATIONSHIP_TYPE {type: "relationship_subtype"}]->(:[RelatedTerm])
(:[TermName])-[:ANOTHER_RELATIONSHIP {type: "relationship_subtype"}]->(:[AnotherTerm])

// Additional relationships between terms
(:[AnotherTerm])-[:RELATES_TO {type: "relationship_subtype"}]->(:[ThirdTerm])

// Relationships can use these common types:
// Hierarchical: IS_A, PART_OF, CONTAINS, EXTENDS
// Functional: MANAGES, PROCESSES, VALIDATES, CONFIGURES
// Temporal: PRECEDES, FOLLOWS, TRIGGERS, DEPENDS_ON
// Feel free to define domain-specific relationships as appropriate
```

## Taxonomies

### [Primary Taxonomy Name]

```cypher
(:Taxonomy {name: "[Primary Taxonomy Name]"})
-[:CONTAINS]->(:Category {name: "[Category1]"})
-[:CONTAINS]->(:Category {name: "[Category2]"})

(:Category {name: "[Category1]"})
-[:CONTAINS]->(:Term {name: "[TermName]"})
-[:CONTAINS]->(:Term {name: "[AnotherTerm]"})

(:Category {name: "[Category2]"})
-[:CONTAINS]->(:Term {name: "[ThirdTerm]"})
```

### [Secondary Taxonomy Name]

```cypher
(:Taxonomy {name: "[Secondary Taxonomy Name]"})
-[:CONTAINS]->(:Category {name: "[CategoryA]"})
-[:CONTAINS]->(:Operation {name: "[OperationX]"})
-[:CONTAINS]->(:Operation {name: "[OperationY]"})

// Add more taxonomy structures as needed
```

## Usage Contexts

```cypher
(:UsageContext {name: "[ContextName1]"})
-[:APPLIES_TO]->(:TermName)
-[:REQUIRES]->(:RelatedTerm)
-[:PRODUCES]->(:OutputTerm)

(:UsageContext {name: "[ContextName2]"})
-[:APPLIES_TO]->(:AnotherTerm)
-[:REQUIRES]->(:InputTerm)
-[:PRODUCES]->(:ResultTerm)

// Add more usage contexts as needed
```

## Code References

```cypher
(:CodeBase {path: "[Path/to/code1]"})
-[:IMPLEMENTS]->(:TermName)

(:CodeBase {path: "[Path/to/code2]"})
-[:IMPLEMENTS]->(:AnotherTerm)

(:CodeBase {path: "[Path/to/docs]"})
-[:DOCUMENTS]->(:ThirdTerm)

// Add more code references as needed
``` 