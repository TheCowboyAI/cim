#!/bin/bash
#
# standardize_vocabulary.sh
#
# Script to check if vocabulary files follow the rules defined in vocabulary.mdc
#
# Usage: ./standardize_vocabulary.sh [path/to/check]
#

# Default to vocabulary directory if no path specified
CHECK_PATH=${1:-"./vocabulary"}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Vocabulary Standardization Checker ===${NC}"
echo -e "${BLUE}Checking files in: ${CHECK_PATH}${NC}"
echo ""

# Required sections according to vocabulary.mdc
REQUIRED_SECTIONS=("Domain Objects" "Relationships" "Taxonomies" "Usage Contexts" "Code References")

# Required term properties
REQUIRED_PROPERTIES=("domain" "term" "taxonomy" "definition" "usage_context" "code_reference")

# Track statistics
total_files=0
compliant_files=0
non_compliant_files=0

# Function to check if file has Cypher-formatted terms
check_cypher_format() {
    local file=$1
    if grep -q "(:.*:.*{" "$file"; then
        return 0 # Found Cypher format
    else
        return 1 # No Cypher format found
    fi
}

# Function to check missing properties in a file
check_missing_properties() {
    local file=$1
    local missing_props=()
    
    if grep -q "(:.*:.*{" "$file"; then
        for prop in "${REQUIRED_PROPERTIES[@]}"; do
            if ! grep -q "$prop:" "$file"; then
                missing_props+=("$prop")
            fi
        done
    fi
    
    echo "${missing_props[@]}"
}

# Function to check if file has all required sections
check_required_sections() {
    local file=$1
    local missing_sections=()
    
    for section in "${REQUIRED_SECTIONS[@]}"; do
        if ! grep -q "^## $section$" "$file"; then
            missing_sections+=("$section")
        fi
    done
    
    echo "${missing_sections[@]}"
}

# Main function to check a file
check_file() {
    local file=$1
    local issues=0
    
    echo -e "${BLUE}Checking: ${file}${NC}"
    
    # Check if file exists
    if [ ! -f "$file" ]; then
        echo -e "${RED}  File does not exist${NC}"
        return 1
    fi
    
    # Check if file has Cypher-formatted terms
    if ! check_cypher_format "$file"; then
        echo -e "${YELLOW}  No Cypher-formatted terms found${NC}"
        echo -e "${YELLOW}  Expected format: (:TermName:Type { domain: \"Domain\", ... })${NC}"
        ((issues++))
    fi
    
    # Check if file has all required sections
    local missing_sections=($(check_required_sections "$file"))
    if [ ${#missing_sections[@]} -ne 0 ]; then
        echo -e "${YELLOW}  Missing sections:${NC}"
        for section in "${missing_sections[@]}"; do
            echo -e "    - $section"
        done
        ((issues++))
    fi
    
    # Check if any term is missing required properties
    if grep -q "(:.*:.*{" "$file"; then
        local missing_props=($(check_missing_properties "$file"))
        if [ ${#missing_props[@]} -ne 0 ]; then
            echo -e "${YELLOW}  Missing properties in terms:${NC}"
            for prop in "${missing_props[@]}"; do
                echo -e "    - $prop"
            done
            ((issues++))
        fi
    fi
    
    # Report compliance status
    if [ $issues -eq 0 ]; then
        echo -e "${GREEN}  File is compliant with vocabulary.mdc rules${NC}"
        return 0
    else
        echo -e "${YELLOW}  Found $issues issues to fix${NC}"
        echo -e "${YELLOW}  See template file: vocabulary/domains/knowledge_domain.md${NC}"
        return 1
    fi
}

# Process all Markdown files in the directory
process_directory() {
    local dir=$1
    local md_files=()
    
    # Find all Markdown files
    while IFS= read -r -d '' file; do
        md_files+=("$file")
    done < <(find "$dir" -type f -name "*.md" -print0)
    
    # Check each file
    for file in "${md_files[@]}"; do
        ((total_files++))
        if check_file "$file"; then
            ((compliant_files++))
        else
            ((non_compliant_files++))
        fi
        echo ""
    done
}

# Start processing
process_directory "$CHECK_PATH"

# Print summary
echo -e "${BLUE}=== Summary ===${NC}"
echo -e "Total files checked: $total_files"
echo -e "${GREEN}Compliant files: $compliant_files${NC}"
echo -e "${YELLOW}Non-compliant files: $non_compliant_files${NC}"

exit $non_compliant_files 