#!/bin/bash

# This script helps convert non-Cypher formatted vocabulary files to Cypher format
# It provides a structure and templates to assist with the conversion process
# according to vocabulary.mdc rules

# Check if input file is provided
if [ $# -lt 1 ]; then
  echo "Usage: $0 <input_file> [domain]"
  echo "Example: $0 vocabulary/domains/cim.md Technical"
  exit 1
fi

INPUT_FILE=$1
DOMAIN=${2:-"Technical"}  # Default domain if not provided
OUTPUT_FILE="${INPUT_FILE%.md}_cypher.md"  # Create a new file with _cypher suffix
TEMP_FILE="${INPUT_FILE%.md}_temp.md"

# Check if input file exists
if [ ! -f "$INPUT_FILE" ]; then
  echo "Error: Input file '$INPUT_FILE' not found"
  exit 1
fi

# Create a backup of the original file
cp "$INPUT_FILE" "${INPUT_FILE}.bak"
echo "Created backup at: ${INPUT_FILE}.bak"

# Extract file name without path and extension
FILENAME=$(basename "$INPUT_FILE" .md)

# Function to display section template
display_section_template() {
  local section=$1
  
  case "$section" in
    "nodes")
      echo "## Domain Objects

### Nodes

\`\`\`cypher
(:Example:Entity {
  domain: \"$DOMAIN\",
  term: \"Example\",
  taxonomy: \"${DOMAIN} Taxonomy\",
  definition: \"Definition goes here\",
  usage_context: \"Where/how this term is used in the system\",
  code_reference: \"TBD\"
})

(:AnotherExample:Aggregate {
  domain: \"$DOMAIN\",
  term: \"AnotherExample\",
  taxonomy: \"${DOMAIN} Taxonomy\",
  definition: \"Definition goes here\",
  usage_context: \"Where/how this term is used in the system\",
  code_reference: \"TBD\"
})

// Add more terms using these types:
// Domain Objects: Entity, Aggregate, ValueObject, Service, Policy, Command, Query, Event
// Technical Concepts: Pattern, Architecture, Protocol, Algorithm
// Business Concepts: Process, Rule, Workflow, Policy
// Cross-Cutting Terms: Security, Configuration, Monitoring, Event
\`\`\`"
      ;;
      
    "relationships")
      echo "## Relationships

\`\`\`cypher
// Hierarchical relationships
(:Example)-[:CONTAINS {type: \"component\"}]->(:AnotherExample)
(:Example)-[:PART_OF {type: \"structure\"}]->(:LargerConcept)
(:Example)-[:IS_A {type: \"classification\"}]->(:ParentType)
(:Example)-[:EXTENDS {type: \"enhancement\"}]->(:BaseType)

// Functional relationships
(:Service)-[:MANAGES {type: \"control\"}]->(:Example)
(:Service)-[:PROCESSES {type: \"operation\"}]->(:AnotherExample)
(:Service)-[:VALIDATES {type: \"quality\"}]->(:Example)
(:Service)-[:CONFIGURES {type: \"setup\"}]->(:AnotherExample)

// Temporal relationships
(:FirstStep)-[:PRECEDES {type: \"sequence\"}]->(:Example)
(:Example)-[:FOLLOWS {type: \"sequence\"}]->(:FirstStep)
(:Event)-[:TRIGGERS {type: \"cause\"}]->(:Example)
(:Example)-[:DEPENDS_ON {type: \"requirement\"}]->(:AnotherExample)

// Add more relationships as appropriate for your domain
\`\`\`"
      ;;
      
    "taxonomies")
      echo "## Taxonomies

### ${DOMAIN} Taxonomy

\`\`\`cypher
(:Taxonomy {name: \"${DOMAIN} Taxonomy\"})
-[:CONTAINS]->(:Category {name: \"${DOMAIN} Components\"})
-[:CONTAINS]->(:Category {name: \"${DOMAIN} Operations\"})

(:Category {name: \"${DOMAIN} Components\"})
-[:CONTAINS]->(:Term {name: \"Example\"})
-[:CONTAINS]->(:Term {name: \"AnotherExample\"})

(:Category {name: \"${DOMAIN} Operations\"})
-[:CONTAINS]->(:Term {name: \"ExampleOperation\"})
-[:CONTAINS]->(:Term {name: \"AnotherOperation\"})
\`\`\`

### ${DOMAIN} Classification

\`\`\`cypher
(:Taxonomy {name: \"${DOMAIN} Classification\"})
-[:CONTAINS]->(:Category {name: \"${DOMAIN} Types\"})
-[:CONTAINS]->(:Category {name: \"${DOMAIN} Categories\"})

(:Category {name: \"${DOMAIN} Types\"})
-[:CONTAINS]->(:Term {name: \"ExampleType\"})
-[:CONTAINS]->(:Term {name: \"AnotherType\"})

(:Category {name: \"${DOMAIN} Categories\"})
-[:CONTAINS]->(:Term {name: \"ExampleCategory\"})
-[:CONTAINS]->(:Term {name: \"AnotherCategory\"})
\`\`\`

// Consider including these taxonomies if relevant:
// - Storage Taxonomy (Storage Operations, Location Management, etc.)
// - Media Taxonomy (Media Types, Metadata, etc.)
// - UI Taxonomy (Windows, Dialogs, Themes, etc.)
// - Configuration Taxonomy (Settings, Preferences, etc.)
"
      ;;
      
    "usage_contexts")
      echo "## Usage Contexts

\`\`\`cypher
(:UsageContext {name: \"${DOMAIN}Creation\"})
-[:APPLIES_TO]->(:Example)
-[:REQUIRES]->(:AnotherExample)
-[:PRODUCES]->(:OutputConcept)

(:UsageContext {name: \"${DOMAIN}Management\"})
-[:APPLIES_TO]->(:Service)
-[:REQUIRES]->(:Example)
-[:PRODUCES]->(:AnotherExample)

(:UsageContext {name: \"${DOMAIN}Processing\"})
-[:APPLIES_TO]->(:Process)
-[:REQUIRES]->(:InputConcept)
-[:PRODUCES]->(:ResultConcept)

// Add more usage contexts that describe how terms are used in different scenarios
\`\`\`"
      ;;
      
    "code_references")
      echo "## Code References

\`\`\`cypher
(:CodeBase {path: \"cim/src/${FILENAME,,}/example\"})
-[:IMPLEMENTS]->(:Example)

(:CodeBase {path: \"cim/src/${FILENAME,,}/another_example\"})
-[:IMPLEMENTS]->(:AnotherExample)

(:CodeBase {path: \"cim/docs/${FILENAME,,}/readme.md\"})
-[:DOCUMENTS]->(:Example)

// Add actual code references using paths like:
// - cim/src/domain/concept
// - cim/docs/specifications/concept.md 
// - Use TBD for the path if not yet implemented
\`\`\`"
      ;;
  esac
}

# Create a new file with Cypher template structure
echo "# ${FILENAME^} Domain Vocabulary" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Add Cypher templates for all sections
for section in "nodes" "relationships" "taxonomies" "usage_contexts" "code_references"; do
  display_section_template "$section" >> "$OUTPUT_FILE"
  echo "" >> "$OUTPUT_FILE"
done

echo "Created template file: $OUTPUT_FILE"
echo ""
echo "=== Next Steps ==="
echo "1. Open the original file ($INPUT_FILE) and the template file ($OUTPUT_FILE)"
echo "2. Convert the bullet points or descriptions in the original file to Cypher format"
echo "3. Replace the example templates in $OUTPUT_FILE with actual content"
echo "4. Ensure all nodes have the required properties (domain, term, definition, taxonomy, usage_context, code_reference)"
echo "5. After completing the conversion, verify with: ./scripts/standardize_vocabulary.sh $OUTPUT_FILE"
echo ""
echo "When finished, you can replace the original file with the converted one:"
echo "mv $OUTPUT_FILE $INPUT_FILE"

# Extract terms from the original file to assist with conversion
echo "=== Extracting potential terms from original file ==="
echo "The following might be key terms to convert to nodes:"

# Look for bullet points, headings, or emphasized text that might be terms
grep -E '^\s*[*-] |^#{2,4} [A-Z]|_[A-Za-z ]+_|\*\*[A-Za-z ]+\*\*' "$INPUT_FILE" | 
  sed 's/^\s*[*-] //g; s/^#{2,4} //g; s/_//g; s/\*\*//g' | 
  sort | uniq

echo ""
echo "=== Term Category Guidelines ==="
echo "Domain Objects: Entity, Aggregate, ValueObject, Service, Policy, Command, Query, Event"
echo "Technical Concepts: Pattern, Architecture, Protocol, Algorithm"
echo "Business Concepts: Process, Rule, Workflow, Policy"
echo "Cross-Cutting Terms: Security, Configuration, Monitoring, Event"
echo ""
echo "=== Relationship Type Guidelines ==="
echo "Hierarchical: IS_A, PART_OF, CONTAINS, EXTENDS"
echo "Functional: MANAGES, PROCESSES, VALIDATES, CONFIGURES"
echo "Temporal: PRECEDES, FOLLOWS, TRIGGERS, DEPENDS_ON"
echo ""
echo "=== Note on Required Properties ==="
echo "Each term must have these properties:"
echo "domain: The domain the term belongs to (e.g., \"$DOMAIN\")"
echo "term: The name of the term (identical to the node name)"
echo "taxonomy: The taxonomy it belongs to (e.g., \"${DOMAIN} Taxonomy\")"
echo "definition: Clear description of what the term means"
echo "usage_context: Where and how the term is used"
echo "code_reference: Path to implementation (use \"TBD\" if unknown)"

exit 0 