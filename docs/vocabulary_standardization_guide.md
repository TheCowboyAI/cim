# Vocabulary Standardization Guide

This guide provides detailed instructions for standardizing vocabulary files according to the rules in `.cursor/rules/vocabulary.mdc`.

## Overview of Standardization Requirements

All vocabulary files must:

1. Have the required **sections**:
   - Domain Objects
   - Relationships
   - Taxonomies
   - Usage Contexts
   - Code References

2. Use the **Cypher format** for defining terms:
   ```cypher
   (:[TermName]:[TypeName] {
     domain: "[Domain]",
     term: "[TermName]",
     taxonomy: "[Taxonomy Name]",
     definition: "[Definition]",
     usage_context: "[Usage Context]",
     code_reference: "[Code Reference]"
   })
   ```

3. Include all **required properties** for each term:
   - domain
   - term
   - taxonomy
   - definition
   - usage_context
   - code_reference

4. Use proper **relationship definitions** in Cypher format:
   ```cypher
   (:[TermName])-[:RELATIONSHIP_TYPE {type: "relationship_subtype"}]->(:[RelatedTerm])
   ```

## The Standardization Process

### 1. Preparation

Before standardizing a file:
- Review the reference files: `vocabulary/domains/knowledge_domain.md` and `vocabulary/domains/template.md`
- Make a backup copy of the file you're about to standardize
- Run `./scripts/standardize_vocabulary_enhanced.sh path/to/file.md` to identify issues

### 2. Step-by-Step Standardization

#### For Files with Cypher Format (Missing Properties)

1. Open the file in your editor
2. For each term node, ensure it has all required properties:
   ```cypher
   (:[TermName]:[TypeName] {
     domain: "[Domain]",  // Add if missing
     term: "[TermName]",  // Add if missing
     taxonomy: "[Taxonomy Name]",  // Add if missing
     definition: "[Definition]",  // Should exist
     usage_context: "[Usage Context]",  // Add if missing
     code_reference: "[Code Reference]"  // Add if missing ("TBD" if unknown)
   })
   ```
3. Save the file and verify with the standardization script

#### For Files Missing Sections

1. Add any missing sections with the correct headings:
   ```markdown
   ## Domain Objects
   ## Relationships
   ## Taxonomies
   ## Usage Contexts
   ## Code References
   ```
2. For each section, add appropriate content using the Cypher format
3. Use the template as a guide for the structure of each section
4. Save the file and verify with the standardization script

#### For Files Without Cypher Format

1. Identify all existing terms in the current format (often bullet points)
2. For each term, create a proper Cypher node definition:
   ```cypher
   (:[TermName]:[TypeName] {
     domain: "[Domain]",
     term: "[TermName]",
     taxonomy: "[Taxonomy Name]",
     definition: "[Definition from existing text]",
     usage_context: "[Derive from context or use best judgment]",
     code_reference: "[Existing reference or 'TBD']"
   })
   ```
3. Create proper relationships between the terms
4. Create taxonomy structures based on existing categories
5. Create usage contexts based on how the terms are used
6. Create code references for any implementation details
7. Save the file and verify with the standardization script

### 3. Common Transformations

#### From Bullet Points to Cypher Nodes

**Before:**
```markdown
- **TermName**: A definition of what the term means.
```

**After:**
```cypher
(:TermName:Entity {
  domain: "Domain",
  term: "TermName",
  taxonomy: "Domain Taxonomy",
  definition: "A definition of what the term means.",
  usage_context: "Derived from context or best judgment",
  code_reference: "TBD"
})
```

#### From Text Relationships to Cypher Relationships

**Before:**
```markdown
TermA relates to TermB through X.
```

**After:**
```cypher
(:TermA)-[:RELATES_TO {type: "X"}]->(:TermB)
```

#### From Simple Lists to Taxonomies

**Before:**
```markdown
Types of X:
- TypeA
- TypeB
- TypeC
```

**After:**
```cypher
(:Taxonomy {name: "X Taxonomy"})
-[:CONTAINS]->(:Category {name: "X Types"})

(:Category {name: "X Types"})
-[:CONTAINS]->(:Term {name: "TypeA"})
-[:CONTAINS]->(:Term {name: "TypeB"})
-[:CONTAINS]->(:Term {name: "TypeC"})
```

### 4. Verification

After standardizing a file:

1. Run the validation script:
   ```bash
   ./scripts/standardize_vocabulary.sh path/to/standardized/file.md
   ```

2. Address any remaining issues identified by the script

3. Update the standardization checklist:
   - Mark the file as complete
   - Add an entry in the log
   - Update the progress summary

### 5. Common Issues and Solutions

| Issue | Solution |
|-------|----------|
| Missing required property | Add the property with appropriate value; use "TBD" if unknown |
| Incorrect Cypher syntax | Reference the template for correct syntax |
| Missing relationship direction | Ensure relationships include source->target direction |
| Missing section | Add the section with appropriate content |
| Non-descriptive taxonomy | Enhance taxonomies to reflect hierarchical relationships |

## Reference Types for Domain Objects

When standardizing terms, use these common type designations:

- **Entity**: A core business object with a distinct identity
- **Aggregate**: A cluster of entities and value objects
- **Value Object**: An immutable object representing a measurable quantity or descriptor
- **Service**: A stateless operation or process
- **Process**: A sequence of related activities
- **Event**: Something that has happened
- **Command**: An instruction to do something
- **Query**: A request for information
- **Policy**: A rule or guideline
- **Repository**: A mechanism for accessing domain objects

## References

- Vocabulary Rules: `.cursor/rules/vocabulary.mdc`
- Template File: `vocabulary/domains/template.md`
- Reference Implementation: `vocabulary/domains/knowledge_domain.md`
- Standardization Tools: `scripts/standardize_vocabulary.sh` and `scripts/standardize_vocabulary_enhanced.sh`
- Tracking: `docs/standardization_checklist.md` 