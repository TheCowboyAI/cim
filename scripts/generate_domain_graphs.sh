#!/bin/bash
set -e

# Configuration
VOCAB_DIR="vocabulary/domains"
OUTPUT_DIR="docs/knowledge_graphs"

echo "Generating Domain-Specific Knowledge Graphs..."

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Process each domain file
for file in "$VOCAB_DIR"/*.md; do
    domain=$(basename "$file" .md)
    output_file="$OUTPUT_DIR/${domain}_graph.md"
    echo "Processing $domain domain..."
    
    # Start the output file with header
    cat > "$output_file" << EOL
# ${domain^} Domain Knowledge Graph

This document visualizes the relationships between terms in the ${domain^} domain.

## Term Relationships

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

    # Extract all terms from this domain
    domain_terms=()
    while read -r line; do
        if [[ $line =~ ^"### Term: "(.+)$ ]]; then
            term="${BASH_REMATCH[1]}"
            domain_terms+=("$term")
        fi
    done < "$file"

    # Extract terms and their types
    for term in "${domain_terms[@]}"; do
        # Find the term in the file
        line_num=$(grep -n "### Term: $term" "$file" | cut -d: -f1)
        if [ -z "$line_num" ]; then continue; fi
        
        # Find the type for this term (skip 2 lines after the term)
        type=$(sed -n "$((line_num+2))p" "$file" | grep -o '**Type**: .*' | sed 's/**Type**: //')
        
        # Write node definition
        echo "    ${term//[^a-zA-Z0-9]/_}[\"$term\"]" >> "$output_file"
        
        # Add class based on type
        case "$type" in
            "Entity") echo "    class ${term//[^a-zA-Z0-9]/_} entity" >> "$output_file" ;;
            "Value Object") echo "    class ${term//[^a-zA-Z0-9]/_} valueObject" >> "$output_file" ;;
            "Aggregate") echo "    class ${term//[^a-zA-Z0-9]/_} aggregate" >> "$output_file" ;;
            "Service") echo "    class ${term//[^a-zA-Z0-9]/_} service" >> "$output_file" ;;
            "Platform") echo "    class ${term//[^a-zA-Z0-9]/_} platform" >> "$output_file" ;;
            "Tool") echo "    class ${term//[^a-zA-Z0-9]/_} tool" >> "$output_file" ;;
            "Protocol") echo "    class ${term//[^a-zA-Z0-9]/_} protocol" >> "$output_file" ;;
            "Framework") echo "    class ${term//[^a-zA-Z0-9]/_} framework" >> "$output_file" ;;
            *) echo "    class ${term//[^a-zA-Z0-9]/_} default" >> "$output_file" ;;
        esac
    done

    # Add terms from other domains that are referenced in relationships
    referenced_terms=()
    while read -r line; do
        if [[ $line =~ ^"### Term: "(.+)$ ]]; then
            current_term="${BASH_REMATCH[1]}"
        elif [[ $line =~ ^\*\ ([^:]+):\ (.+)$ ]]; then
            rel_type="${BASH_REMATCH[1]}"
            rel_targets="${BASH_REMATCH[2]}"
            
            # Add targets to referenced terms if they're not in domain_terms
            IFS=',' read -ra TARGETS <<< "$rel_targets"
            for target in "${TARGETS[@]}"; do
                target=$(echo "$target" | xargs)  # Trim whitespace
                if [[ ! " ${domain_terms[@]} " =~ " ${target} " ]]; then
                    referenced_terms+=("$target")
                fi
            done
        fi
    done < "$file"

    # Add referenced terms as external nodes
    if [ ${#referenced_terms[@]} -gt 0 ]; then
        echo -e "\n    %% External referenced terms" >> "$output_file"
        for term in "${referenced_terms[@]}"; do
            echo "    ${term//[^a-zA-Z0-9]/_}[\"$term (external)\"]" >> "$output_file"
            echo "    class ${term//[^a-zA-Z0-9]/_} default" >> "$output_file"
        done
    fi

    # Process relationships
    echo -e "\n    %% Relationships" >> "$output_file"
    while read -r line; do
        if [[ $line =~ ^"### Term: "(.+)$ ]]; then
            current_term="${BASH_REMATCH[1]}"
        elif [[ $line =~ ^\*\ ([^:]+):\ (.+)$ ]] && [[ " ${domain_terms[@]} " =~ " ${current_term} " ]]; then
            rel_type="${BASH_REMATCH[1]}"
            rel_targets="${BASH_REMATCH[2]}"
            
            # Handle comma-separated targets
            IFS=',' read -ra TARGETS <<< "$rel_targets"
            for target in "${TARGETS[@]}"; do
                target=$(echo "$target" | xargs)  # Trim whitespace
                # Add the relationship to the diagram
                echo "    ${current_term//[^a-zA-Z0-9]/_} -->|$rel_type| ${target//[^a-zA-Z0-9]/_}" >> "$output_file"
            done
        fi
    done < "$file"

    # Close the mermaid diagram
    echo -e "\`\`\`\n" >> "$output_file"

    # Add legend
    cat >> "$output_file" << EOL
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

## Term Definitions

EOL

    # Add term definitions
    for term in "${domain_terms[@]}"; do
        # Find the term in the file
        term_section=$(sed -n "/### Term: $term/,/### Term:/p" "$file" | sed '$d')
        if [ -z "$term_section" ]; then continue; fi
        
        # Extract definition
        definition=$(echo "$term_section" | grep -o '**Definition**: .*' | sed 's/**Definition**: //')
        
        # Write term definition
        echo "- **$term**: $definition" >> "$output_file"
    done

    echo "Generated $output_file"
done

# Create an index file for the domain graphs
index_file="$OUTPUT_DIR/index.md"
cat > "$index_file" << EOL
# CIM Domain Knowledge Graphs

This directory contains domain-specific knowledge graphs visualizing the relationships between terms in each domain.

## Available Domain Graphs

EOL

for file in "$VOCAB_DIR"/*.md; do
    domain=$(basename "$file" .md)
    echo "- [${domain^} Domain](${domain}_graph.md)" >> "$index_file"
done

echo "Knowledge graph index created: $index_file" 