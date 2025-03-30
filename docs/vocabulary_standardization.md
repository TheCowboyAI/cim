# Vocabulary Standardization Guide

This guide explains how to standardize vocabulary files according to the rules defined in the `vocabulary.mdc` file.

## Overview

The CIM project uses a standardized vocabulary format for maintaining consistent domain terms, concepts, and relationships. This format is defined in the `.cursor/rules/vocabulary.mdc` file and enforced through the standardization script.

## File Structure

Each domain vocabulary file should follow this structure:

```markdown
# <Domain> Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:<TermName>:<Type> {
  domain: "<DomainName>",
  term: "<TermName>",
  taxonomy: "<TaxonomyName>",
  definition: "<Clear, concise definition>",
  usage_context: "<Where/how this term is used>",
  code_reference: "<Link to implementing code (if applicable)>"
})

// Additional nodes...
```

## Relationships

```cypher
// <TermName> relationships
(:<TermName>)-[:<RELATIONSHIP_TYPE> {type: "<subtype>"}]->(:<RelatedTerm>)
// Additional relationships...
```

## Taxonomies

### <Taxonomy Name>

```cypher
(:Taxonomy {name: "<TaxonomyName>"})
-[:CONTAINS]->(:Category {name: "<CategoryName>"})
-[:CONTAINS]->(:Operation {name: "<OperationName>"})
// Additional taxonomy items...
```

## Usage Contexts

```cypher
(:UsageContext {name: "<ContextName>"})
-[:APPLIES_TO]->(:<Term>)
-[:REQUIRES]->(:<Term>)
-[:PRODUCES]->(:<Term>)
// Additional contexts...
```

## Code References

```cypher
(:CodeBase {path: "<path/to/code>"})
-[:IMPLEMENTS]->(:<Term>)
// Additional references...
```
```

## Required Sections

Each vocabulary file must include these sections:

1. **Domain Objects**: Terms and their properties
2. **Relationships**: Connections between terms
3. **Taxonomies**: Categories and hierarchies
4. **Usage Contexts**: Where and how terms are used
5. **Code References**: Links to implementing code

## Term Properties

Each term must have these properties:

1. **domain**: The domain the term belongs to
2. **term**: The name of the term
3. **taxonomy**: The taxonomy the term belongs to
4. **definition**: A clear, concise definition
5. **usage_context**: Where and how the term is used
6. **code_reference**: A link to implementing code (or "TBD" if not yet implemented)

## Relationship Types

Use these standard relationship types:

### Hierarchical
- IS_A
- PART_OF
- CONTAINS
- EXTENDS

### Functional
- MANAGES
- PROCESSES
- VALIDATES
- CONFIGURES

### Temporal
- PRECEDES
- FOLLOWS
- TRIGGERS
- DEPENDS_ON

## Standardization Tools

### Template File

Use the template file as a starting point for new vocabulary files:

```bash
cp vocabulary/domains/template.md vocabulary/domains/new_domain.md
```

### Standardization Script

Use the standardization script to check if your vocabulary files comply with the rules:

```bash
./scripts/standardize_vocabulary.sh
```

To check a specific directory:

```bash
./scripts/standardize_vocabulary.sh vocabulary/domains
```

## Step-by-Step Standardization Process

1. **Identify Non-Compliant Files**:
   - Run the standardization script to identify files that need updating:
   ```bash
   ./scripts/standardize_vocabulary.sh
   ```

2. **Update Files**:
   - Start with the simplest files (those with fewest issues)
   - Use the template file as a reference
   - Convert all terms to Cypher format
   - Ensure all sections are present
   - Add all required properties to terms

3. **Check Compliance**:
   - Run the standardization script again to verify your changes

4. **Update Related Files**:
   - Ensure consistency across related domains
   - Update files that reference the terms you've changed

5. **Documentation**:
   - After standardizing, update any documentation that references the vocabulary

## Example: Converting a Term

### Before:
```markdown
### Term: Research
- **Category**: Domain Object
- **Type**: Aggregate
- **Taxonomy**: Knowledge Taxonomy
- **Definition**: Systematic investigation to establish facts and reach conclusions
- **Relationships**:
  * Contains: Evidence, Methods, Findings
  * Validates: Claims
  * Produces: Knowledge
- **Usage Context**: Foundation for knowledge creation and validation
- **Code Reference**: TBD
```

### After:
```markdown
## Domain Objects

### Nodes

```cypher
(:Research:Aggregate {
  domain: "Knowledge",
  term: "Research",
  taxonomy: "Knowledge Taxonomy",
  definition: "Systematic investigation to establish facts and reach conclusions",
  usage_context: "Foundation for knowledge creation and validation",
  code_reference: "TBD"
})
```

## Relationships

```cypher
// Research relationships
(:Research)-[:CONTAINS {type: "component"}]->(:Evidence)
(:Research)-[:CONTAINS {type: "component"}]->(:Method)
(:Research)-[:CONTAINS {type: "component"}]->(:Finding)
(:Research)-[:VALIDATES {type: "process"}]->(:Claim)
(:Research)-[:PRODUCES {type: "output"}]->(:Knowledge)
```
```

## References

- [Vocabulary Management Rule](.cursor/rules/vocabulary.mdc)
- [Knowledge Domain Template](vocabulary/domains/knowledge_domain.md)
- [Domain Template](vocabulary/domains/template.md) 