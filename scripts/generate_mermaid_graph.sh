#!/bin/bash
set -e

# Configuration
VOCAB_DIR="vocabulary/domains"
OUTPUT_FILE="docs/knowledge_graph.md"

echo "Generating Knowledge Graph from Vocabulary..."

# Start the output file with header
cat > "$OUTPUT_FILE" << EOL
# CIM Knowledge Graph

This document visualizes the relationships between key terms in our vocabulary.

## Domain Relationships

\`\`\`mermaid
graph TD
    %% Define styles
    classDef entity fill:#d1e7f9,stroke:#0366d6
    classDef valueObject fill:#d1f9d9,stroke:#03d666
    classDef aggregate fill:#f9d1e7,stroke:#d60366
    classDef service fill:#f9f9d1,stroke:#d6d603
    classDef platform fill:#d1f9f9,stroke:#03d6d6
    classDef tool fill:#e7d1f9,stroke:#6603d6
    classDef protocol fill:#f9e7d1,stroke:#d66603
    classDef framework fill:#e7e7f9,stroke:#6666d6
    classDef default fill:#f9f9f9,stroke:#666666

EOL

# Process each domain file
for file in "$VOCAB_DIR"/*.md; do
    domain=$(basename "$file" .md)
    echo "Processing $domain domain..."
    
    # Extract terms and their type
    grep -n "### Term:" "$file" | while read -r line; do
        line_num=$(echo "$line" | cut -d: -f1)
        term=$(echo "$line" | sed 's/.*Term: //')
        
        # Find the type for this term (safely)
        type_line=$(sed -n "$((line_num+2))p" "$file" || echo "")
        type=$(echo "$type_line" | grep -o "Type.*" | sed 's/Type\*\*: //' || echo "Unknown")
        
        # Clean term for node ID (remove special chars)
        safe_term=$(echo "$term" | tr -cd '[:alnum:]_' | tr '[:upper:]' '[:lower:]')
        
        # Write node definition with both ID and label
        echo "    ${safe_term}[\"$term\"]" >> "$OUTPUT_FILE"
        
        # Add class based on type
        case "$type" in
            *"Entity"*) echo "    class ${safe_term} entity" >> "$OUTPUT_FILE" ;;
            *"Value Object"*) echo "    class ${safe_term} valueObject" >> "$OUTPUT_FILE" ;;
            *"Aggregate"*) echo "    class ${safe_term} aggregate" >> "$OUTPUT_FILE" ;;
            *"Service"*) echo "    class ${safe_term} service" >> "$OUTPUT_FILE" ;;
            *"Platform"*) echo "    class ${safe_term} platform" >> "$OUTPUT_FILE" ;;
            *"Tool"*) echo "    class ${safe_term} tool" >> "$OUTPUT_FILE" ;;
            *"Protocol"*) echo "    class ${safe_term} protocol" >> "$OUTPUT_FILE" ;;
            *"Framework"*) echo "    class ${safe_term} framework" >> "$OUTPUT_FILE" ;;
            *) echo "    class ${safe_term} default" >> "$OUTPUT_FILE" ;;
        esac
    done
done

# Process relationships
echo -e "\n    %% Relationships" >> "$OUTPUT_FILE"

# Create a mapping of terms to their safe versions for lookup
declare -A term_map
for file in "$VOCAB_DIR"/*.md; do
    while read -r line; do
        if [[ "$line" == "### Term:"* ]]; then
            term=$(echo "$line" | sed 's/### Term: //')
            safe_term=$(echo "$term" | tr -cd '[:alnum:]_' | tr '[:upper:]' '[:lower:]')
            term_map["$term"]="$safe_term"
        fi
    done < "$file"
done

# Now process relationships
for file in "$VOCAB_DIR"/*.md; do
    domain=$(basename "$file" .md)
    
    # Find term sections and extract relationships
    current_term=""
    current_safe_term=""
    while IFS= read -r line; do
        # Check if this is a term definition
        if [[ "$line" == "### Term:"* ]]; then
            current_term=$(echo "$line" | sed 's/### Term: //')
            current_safe_term=$(echo "$current_term" | tr -cd '[:alnum:]_' | tr '[:upper:]' '[:lower:]')
        # Check if this is a relationship line
        elif [[ "$line" == "  * "* ]] && [ -n "$current_term" ]; then
            # Extract relationship type and target
            rel_parts=$(echo "$line" | cut -d':' -f1-2)
            rel_type=$(echo "$rel_parts" | sed 's/  \* //' | sed 's/:.*//')
            rel_targets=$(echo "$rel_parts" | sed "s/.*$rel_type: //")
            
            # Handle comma-separated targets
            IFS=',' read -ra TARGETS <<< "$rel_targets"
            for target in "${TARGETS[@]}"; do
                target=$(echo "$target" | xargs)  # Trim whitespace
                # Look up the safe version of the target term
                target_safe="${term_map[$target]}"
                if [ -n "$target_safe" ]; then
                    # Add the relationship to the diagram if target exists in our term map
                    echo "    $current_safe_term -->|$rel_type| $target_safe" >> "$OUTPUT_FILE"
                fi
            done
        fi
    done < "$file"
done

# Close the mermaid diagram
echo -e "\`\`\`\n" >> "$OUTPUT_FILE"

# Add legend
cat >> "$OUTPUT_FILE" << EOL
## Legend

- Node shapes represent term types:
  - Entity: Blue
  - Value Object: Green
  - Aggregate: Pink
  - Service: Yellow
  - Platform: Cyan
  - Tool: Purple
  - Protocol: Orange
  - Framework: Light Blue

- Edge labels indicate relationship types:
  - Contains: Containment relationship
  - Uses: Usage dependency
  - Implements: Implementation relationship
  - Part-Of: Composition relationship
  - Provides: Capability provision
  - Defines: Definition relationship
  - Manages: Management relationship
  - Identifies: Identification relationship

## Domain-Specific Views

For better readability, you can view domain-specific graphs in the `/docs/knowledge_graphs/` directory.
EOL

echo "Knowledge graph generated: $OUTPUT_FILE"
echo "You can view this file in any Markdown viewer that supports Mermaid diagrams." 