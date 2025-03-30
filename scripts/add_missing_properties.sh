#!/bin/bash
#
# add_missing_properties.sh
#
# Script to add missing properties to Cypher nodes in vocabulary files.
# This helps with standardizing medium complexity files that already have Cypher format
# but are missing required properties.
#
# Usage: ./add_missing_properties.sh path/to/file.md [domain]
#

# Check if file is provided
if [ -z "$1" ]; then
    echo "Usage: $0 path/to/file.md [domain]"
    exit 1
fi

FILE=$1
DOMAIN=${2:-$(basename "$FILE" .md)}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Adding Missing Properties to $FILE ===${NC}"

# Check if file exists
if [ ! -f "$FILE" ]; then
    echo -e "${RED}File does not exist: $FILE${NC}"
    exit 1
fi

# Create a backup of the file
BACKUP="${FILE}.bak"
cp "$FILE" "$BACKUP"
echo -e "${BLUE}Created backup at: $BACKUP${NC}"

# Function to add missing properties to Cypher nodes
add_missing_properties() {
    local file=$1
    local domain=$2
    local tmpfile="$file.tmp"
    
    # Process each Cypher node in the file
    grep -oP '\(:[^}]+\})' "$file" | while read -r node; do
        # Extract the node name and type
        node_name=$(echo "$node" | grep -oP ':[a-zA-Z0-9_]+' | head -1 | sed 's/://')
        node_type=$(echo "$node" | grep -oP ':[a-zA-Z0-9_]+' | tail -1 | sed 's/://')
        
        # Check if the node has the required properties
        if ! echo "$node" | grep -q "term:"; then
            echo -e "${YELLOW}Adding 'term' property to $node_name${NC}"
            sed -i "s/\($node_name:[^{]*{[^}]*\)}/\1, term: \"$node_name\"}/g" "$file"
        fi
        
        if ! echo "$node" | grep -q "domain:"; then
            echo -e "${YELLOW}Adding 'domain' property to $node_name${NC}"
            sed -i "s/\($node_name:[^{]*{[^}]*\)}/\1, domain: \"$domain\"}/g" "$file"
        fi
        
        if ! echo "$node" | grep -q "taxonomy:"; then
            echo -e "${YELLOW}Adding 'taxonomy' property to $node_name${NC}"
            taxonomy="${domain^} Taxonomy"
            sed -i "s/\($node_name:[^{]*{[^}]*\)}/\1, taxonomy: \"$taxonomy\"}/g" "$file"
        fi
        
        if ! echo "$node" | grep -q "usage_context:"; then
            echo -e "${YELLOW}Adding 'usage_context' property to $node_name${NC}"
            sed -i "s/\($node_name:[^{]*{[^}]*\)}/\1, usage_context: \"${domain^} context\"}/g" "$file"
        fi
        
        if ! echo "$node" | grep -q "code_reference:"; then
            echo -e "${YELLOW}Adding 'code_reference' property to $node_name${NC}"
            sed -i "s/\($node_name:[^{]*{[^}]*\)}/\1, code_reference: \"TBD\"}/g" "$file"
        fi
    done
    
    echo -e "${GREEN}Added missing properties to all nodes${NC}"
}

# Add missing properties to the file
add_missing_properties "$FILE" "$DOMAIN"

# Check if the file is now compliant
echo -e "${BLUE}Verifying file...${NC}"
./scripts/standardize_vocabulary.sh "$FILE"

result=$?
if [ $result -eq 0 ]; then
    echo -e "${GREEN}The file is now compliant with vocabulary.mdc rules!${NC}"
else
    echo -e "${YELLOW}The file still has some issues. Please check the above messages and fix manually.${NC}"
fi

echo -e "${BLUE}Remember to update the standardization checklist!${NC}"
exit $result 