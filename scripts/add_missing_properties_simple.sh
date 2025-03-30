#!/bin/bash
#
# add_missing_properties_simple.sh
#
# Simple script to add missing properties to Cypher nodes in vocabulary files.
# This script focuses on adding taxonomy, usage_context, and code_reference properties.
#
# Usage: ./add_missing_properties_simple.sh path/to/file.md [domain]
#

# Check if file is provided
if [ -z "$1" ]; then
    echo "Usage: $0 path/to/file.md [domain]"
    exit 1
fi

FILE=$1
# Extract domain from filename if not provided as second argument
if [ -z "$2" ]; then
    # Remove path and extension
    FILENAME=$(basename "$FILE" .md)
    # Handle special cases with _domain suffix
    if [[ "$FILENAME" == *_domain ]]; then
        DOMAIN=${FILENAME%_domain}
        DOMAIN=${DOMAIN^}  # Capitalize first letter
    else
        DOMAIN=${FILENAME^}  # Capitalize first letter
    fi
else
    DOMAIN=$2
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Adding Missing Properties to $FILE ===${NC}"
echo -e "${BLUE}Using domain: $DOMAIN${NC}"

# Check if file exists
if [ ! -f "$FILE" ]; then
    echo -e "${RED}File does not exist: $FILE${NC}"
    exit 1
fi

# Create a backup of the file
BACKUP="${FILE}.bak"
cp "$FILE" "$BACKUP"
echo -e "${BLUE}Created backup at: $BACKUP${NC}"

# Add taxonomy property if missing
if ! grep -q "taxonomy:" "$FILE"; then
    echo -e "${YELLOW}Adding 'taxonomy' property to all nodes${NC}"
    # Find all definition properties and add taxonomy after them
    sed -i '/definition:/a\\  taxonomy: "'"$DOMAIN"' Taxonomy",' "$FILE"
fi

# Add usage_context property if missing
if ! grep -q "usage_context:" "$FILE"; then
    echo -e "${YELLOW}Adding 'usage_context' property to all nodes${NC}"
    # Find all taxonomy properties (or definition if we just added taxonomy) and add usage_context after
    if grep -q "taxonomy:" "$FILE"; then
        sed -i '/taxonomy:/a\\  usage_context: "'"$DOMAIN"' related operations and processes",' "$FILE"
    else
        sed -i '/definition:/a\\  usage_context: "'"$DOMAIN"' related operations and processes",' "$FILE"
    fi
fi

# Add code_reference property if missing
if ! grep -q "code_reference:" "$FILE"; then
    echo -e "${YELLOW}Adding 'code_reference' property to all nodes${NC}"
    # Add before the closing brace of each node
    sed -i '/})/i\\  code_reference: "TBD",' "$FILE"
fi

# Add term property if missing
if ! grep -q "term:" "$FILE"; then
    echo -e "${YELLOW}Adding 'term' property to all nodes${NC}"
    # Extract node names and add term property after domain
    grep -o "(:.*:.*{" "$FILE" | while read -r node; do
        # Extract node name
        node_name=$(echo "$node" | sed -E 's/\(:([^:]+):.*/\1/')
        # Add term property after domain
        sed -i '/domain:/a\\  term: "'"$node_name"'",' "$FILE"
    done
fi

echo -e "${GREEN}Added missing properties to all nodes${NC}"
echo -e "${BLUE}Verify the changes with: diff $BACKUP $FILE${NC}"

# Check if the file is now compliant
echo -e "${BLUE}Verifying file...${NC}"
./scripts/standardize_vocabulary.sh "$FILE"

result=$?
if [ $result -eq 0 ]; then
    echo -e "${GREEN}The file is now compliant with vocabulary.mdc rules!${NC}"
else
    echo -e "${YELLOW}The file still has some issues. Please check the above messages and fix manually.${NC}"
    echo -e "${YELLOW}You may need to check for inconsistencies or make manual edits.${NC}"
fi

echo -e "${BLUE}Remember to update the standardization checklist!${NC}"
exit $result 