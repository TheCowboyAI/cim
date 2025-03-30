#!/bin/bash
#
# add_all_properties.sh
#
# Script to add all missing properties to Cypher nodes in vocabulary files.
# This helps with standardizing medium complexity files that already have Cypher format
# but are missing multiple required properties.
#
# Usage: ./add_all_properties.sh path/to/file.md [domain]
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

# Create a temporary file for the modified content
TMPFILE="${FILE}.tmp"

# Process the file
{
    # Read file line by line
    while IFS= read -r line; do
        # Write the original line
        echo "$line"
        
        # If this line starts a node definition
        if [[ "$line" =~ ^\(:([a-zA-Z0-9_]+): ]]; then
            # Extract node name
            node_name="${BASH_REMATCH[1]}"
            echo -e "${BLUE}Processing node: $node_name${NC}"
            
            # Check if next line has domain property
            read -r next_line
            echo "$next_line"
            
            # If the domain property exists, check for term property
            read -r next_line
            echo "$next_line"
            
            # If the term property is missing, add it
            if [[ ! "$next_line" =~ term: ]]; then
                echo -e "${YELLOW}Adding 'term' property to $node_name${NC}"
                echo "  term: \"$node_name\","
            fi
            
            # Continue checking for the other required properties
            properties_to_check=("taxonomy" "usage_context" "code_reference")
            
            # Read next lines until we find the closing brace
            while IFS= read -r next_line; do
                echo "$next_line"
                
                # If we reached the closing brace
                if [[ "$next_line" =~ \}\) ]]; then
                    # Before the closing brace, add any missing properties
                    for prop in "${properties_to_check[@]}"; do
                        if ! grep -q "$prop:" <(echo "$prop_text"); then
                            echo -e "${YELLOW}Adding '$prop' property to $node_name${NC}"
                            if [ "$prop" == "taxonomy" ]; then
                                echo "  taxonomy: \"${DOMAIN} Taxonomy\","
                            elif [ "$prop" == "usage_context" ]; then
                                echo "  usage_context: \"${DOMAIN} related operations and processes\","
                            elif [ "$prop" == "code_reference" ]; then
                                echo "  code_reference: \"TBD\","
                            fi
                        fi
                    done
                    break
                fi
                
                # Collect property text for checking
                prop_text+="$next_line"
            done
        fi
    done
} < "$FILE" > "$TMPFILE"

# Replace the original file
mv "$TMPFILE" "$FILE"

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
fi

echo -e "${BLUE}Remember to update the standardization checklist!${NC}"
exit $result 