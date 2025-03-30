#!/bin/bash

# This script helps convert non-Cypher formatted vocabulary files to Cypher format
# It provides a structure and templates to assist with the conversion process

# Check if input file is provided
if [ $# -lt 1 ]; then
  echo "Usage: $0 <input_file> [domain]"
  echo "Example: $0 vocabulary/domains/cim.md Business"
  exit 1
fi

INPUT_FILE=$1
DOMAIN=${2:-"KnowledgeDomain"}  # Default domain if not provided
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
(:Example:Aggregate {
  domain: \"$DOMAIN\",
  term: \"Example\",
  definition: \"Definition goes here\",
  taxonomy: \"ExampleTaxonomy\",
  usage_context: \"ExampleContext\",
  code_reference: \"path/to/example.md\"
})

// Add more nodes here
\`\`\`"
      ;;
      
    "relationships")
      echo "## Relationships

\`\`\`cypher
// Example relationships
(:Example)-[:RELATES_TO {type: \"relationship\"}]->(:AnotherExample)
(:Example)-[:CONTAINS {type: \"composition\"}]->(:Component)

// Add more relationships here
\`\`\`"
      ;;
      
    "taxonomies")
      echo "## Taxonomies

### Example Processing

\`\`\`cypher
(:Taxonomy {name: \"ExampleProcessing\"})
-[:CONTAINS]->(:Category {name: \"ExampleCategory\"})
-[:CONTAINS]->(:Operation {name: \"ExampleOperation\"})
-[:CONTAINS]->(:Operation {name: \"AnotherOperation\"})

(:Category {name: \"AnotherCategory\"})
-[:CONTAINS]->(:Operation {name: \"ThirdOperation\"})
-[:CONTAINS]->(:Operation {name: \"FourthOperation\"})
\`\`\`"
      ;;
      
    "usage_contexts")
      echo "## Usage Contexts

\`\`\`cypher
(:UsageContext {name: \"ExampleContext\"})
-[:APPLIES_TO]->(:Example)
-[:REQUIRES]->(:Component)
-[:PRODUCES]->(:Output)

// Add more usage contexts here
\`\`\`"
      ;;
      
    "code_references")
      echo "## Code References

\`\`\`cypher
(:CodeBase {path: \"notes/$FILENAME/readme.md\"})
-[:IMPLEMENTS]->(:Example)

// Add more code references here
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
echo "=== Template Structure for Each Node ==="
echo "(:NodeName:AggregateType {
  domain: \"$DOMAIN\",
  term: \"NodeName\",
  definition: \"Definition goes here\",
  taxonomy: \"RelevantTaxonomy\",
  usage_context: \"RelevantContext\",
  code_reference: \"path/to/implementation.md\"
})"

exit 0 