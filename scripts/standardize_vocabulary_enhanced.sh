#!/bin/bash
#
# standardize_vocabulary_enhanced.sh
#
# Enhanced script to both identify and help automate standardization of vocabulary files
# according to vocabulary.mdc rules.
#
# Usage: ./standardize_vocabulary_enhanced.sh [path/to/check]
#

# Default to vocabulary directory if no path specified
CHECK_PATH=${1:-"./vocabulary"}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Enhanced Vocabulary Standardization Checker ===${NC}"
echo -e "${BLUE}Checking files in: ${CHECK_PATH}${NC}"
echo ""

# Required sections according to vocabulary.mdc
REQUIRED_SECTIONS=("Domain Objects" "Relationships" "Taxonomies" "Usage Contexts" "Code References")

# Required term properties
REQUIRED_PROPERTIES=("domain" "term" "taxonomy" "definition" "usage_context" "code_reference")

# Valid term categories and types from vocabulary.mdc
VALID_CATEGORIES=(
  "DomainObject" "Entity" "Aggregate" "ValueObject" "Policy" "Command" "Query" "Event" "Service"
  "TechnicalConcept" "Pattern" "Architecture" "Protocol" "Algorithm"
  "BusinessConcept" "Process" "Rule" "Workflow" "Policy"
  "CrossCuttingTerm" "Security" "Configuration" "Monitoring" "Event"
)

# Valid relationship types from vocabulary.mdc
VALID_HIERARCHICAL_RELS=("IS_A" "PART_OF" "CONTAINS" "EXTENDS")
VALID_FUNCTIONAL_RELS=("MANAGES" "PROCESSES" "VALIDATES" "CONFIGURES")
VALID_TEMPORAL_RELS=("PRECEDES" "FOLLOWS" "TRIGGERS" "DEPENDS_ON")
VALID_RELATIONSHIPS=(
  "${VALID_HIERARCHICAL_RELS[@]}"
  "${VALID_FUNCTIONAL_RELS[@]}"
  "${VALID_TEMPORAL_RELS[@]}"
  "APPLIES_TO" "REQUIRES" "PRODUCES" "IMPLEMENTS" "USES" "REFERENCES" "SUPPORTS" "ENABLES"
  "BASED_ON" "RELATED_TO" "CONNECTS" "STORES" "RETRIEVES" "CREATES" "UPDATES" "DELETES"
  "AFFECTED_BY" "AFFECTS" "CONFIGURED_BY" "CONFIGURES" "DEFINES" "ENFORCED_BY" "EXTENDED_BY" 
  "FOLLOWS" "IMPLEMENTS" "MANAGES" "OPTIMIZES" "ORGANIZES" "PROVIDES" "RECORDED_IN"
  "TRIGGERED_BY" "TRIGGERS" "USED_BY" "VALIDATED_BY" "VALIDATES"
)

# Primary taxonomies from vocabulary.mdc
PRIMARY_TAXONOMIES=("Storage Taxonomy" "Media Taxonomy" "UI Taxonomy" "Configuration Taxonomy")

# Special term types that should be ignored during validation
SPECIAL_TYPES=("Taxonomy" "Category" "Term" "UsageContext" "CodeBase")

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

# Function to check term types
check_term_types() {
    local file=$1
    local invalid_types=()
    
    # Extract node types from the file - looking for patterns like :Term:Type
    # Only pull the secondary type (after second colon and before curly brace or space)
    types=$(grep -o ":[A-Za-z0-9_]\+:[A-Za-z0-9_]\+" "$file" | cut -d':' -f3 | sort | uniq)
    
    for type in $types; do
        # Skip special types that should be exempt from validation
        if [[ " ${SPECIAL_TYPES[@]} " =~ " ${type} " ]]; then
            continue
        fi
        
        valid=false
        for valid_type in "${VALID_CATEGORIES[@]}"; do
            if [[ "$type" == "$valid_type" ]]; then
                valid=true
                break
            fi
        done
        
        if [ "$valid" = false ]; then
            invalid_types+=("$type")
        fi
    done
    
    echo "${invalid_types[@]}"
}

# Function to check relationship types
check_relationship_types() {
    local file=$1
    local invalid_rels=()
    
    # Extract relationship types from the file
    # Look for patterns like -[:RELATIONSHIP_TYPE 
    rels=$(grep -o "-\[[^]]*\]" "$file" | grep -o ":[A-Z_]\+" | sed 's/^://' | sort | uniq)
    
    for rel in $rels; do
        valid=false
        for valid_rel in "${VALID_RELATIONSHIPS[@]}"; do
            if [[ "$rel" == "$valid_rel" ]]; then
                valid=true
                break
            fi
        done
        
        if [ "$valid" = false ]; then
            invalid_rels+=("$rel")
        fi
    done
    
    echo "${invalid_rels[@]}"
}

# Function to check taxonomy names
check_taxonomies() {
    local file=$1
    local missing_primary=()
    
    # Extract taxonomy names from the file
    taxonomies=$(grep -o "name: \"[^\"]*\"" "$file" | grep "Taxonomy" | cut -d'"' -f2 | sort | uniq)
    
    # Check for primary taxonomies
    for primary in "${PRIMARY_TAXONOMIES[@]}"; do
        found=false
        for tax in $taxonomies; do
            if [[ "$tax" == *"$primary"* ]]; then
                found=true
                break
            fi
        done
        
        if [ "$found" = false ] && grep -q "storage\|media\|ui\|configuration" "$file"; then
            missing_primary+=("$primary")
        fi
    done
    
    echo "${missing_primary[@]}"
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
        
        # Check term types
        local invalid_types=($(check_term_types "$file"))
        if [ ${#invalid_types[@]} -ne 0 ]; then
            echo -e "${YELLOW}  Invalid term types found:${NC}"
            for type in "${invalid_types[@]}"; do
                echo -e "    - $type"
            done
            echo -e "${CYAN}  Valid categories include: Domain Objects (Entity, Aggregate, ValueObject), Technical Concepts (Pattern, Architecture), etc.${NC}"
            ((issues++))
        fi
        
        # Check relationship types
        local invalid_rels=($(check_relationship_types "$file"))
        if [ ${#invalid_rels[@]} -ne 0 ]; then
            echo -e "${YELLOW}  Invalid relationship types found:${NC}"
            for rel in "${invalid_rels[@]}"; do
                echo -e "    - $rel"
            done
            echo -e "${CYAN}  Valid relationships include: CONTAINS, PART_OF, MANAGES, TRIGGERS, etc.${NC}"
            ((issues++))
        fi
        
        # Check taxonomies (if relevant to this file)
        if grep -q "taxonomy.*Taxonomy" "$file"; then
            local missing_primary=($(check_taxonomies "$file"))
            if [ ${#missing_primary[@]} -ne 0 ]; then
                echo -e "${YELLOW}  Consider adding primary taxonomies:${NC}"
                for tax in "${missing_primary[@]}"; do
                    echo -e "    - $tax"
                done
                # This is advisory, not an error
                echo -e "${CYAN}  Primary taxonomies help organize terms consistently across domains${NC}"
            fi
        fi
    fi
    
    # Report compliance status
    if [ $issues -eq 0 ]; then
        echo -e "${GREEN}  File is fully compliant with vocabulary.mdc rules${NC}"
        return 0
    else
        echo -e "${YELLOW}  Found $issues issues to fix${NC}"
        echo -e "${YELLOW}  See template file: vocabulary/domains/knowledge_domain.md${NC}"
        return 1
    fi
}

# Function to suggest fixes for a file
suggest_fixes() {
    local file=$1
    
    echo -e "${CYAN}=== Suggested Fixes for ${file} ===${NC}"
    
    # Missing sections
    local missing_sections=($(check_required_sections "$file"))
    if [ ${#missing_sections[@]} -ne 0 ]; then
        echo -e "${CYAN}To add missing sections:${NC}"
        for section in "${missing_sections[@]}"; do
            echo -e "  Add '## ${section}' section with appropriate content"
        done
    fi
    
    # Missing properties
    if grep -q "(:.*:.*{" "$file"; then
        local missing_props=($(check_missing_properties "$file"))
        if [ ${#missing_props[@]} -ne 0 ]; then
            echo -e "${CYAN}To add missing properties:${NC}"
            echo -e "  Run: ./scripts/add_missing_properties.sh \"$file\""
            echo -e "  Or manually add these properties to each term: ${missing_props[*]}"
        fi
        
        # Invalid term types
        local invalid_types=($(check_term_types "$file"))
        if [ ${#invalid_types[@]} -ne 0 ]; then
            echo -e "${CYAN}To fix invalid term types:${NC}"
            echo -e "  Replace these types with valid categories from vocabulary.mdc:"
            for type in "${invalid_types[@]}"; do
                echo -e "    - $type -> Choose from: Entity, Aggregate, ValueObject, Pattern, etc."
            done
        fi
        
        # Invalid relationship types
        local invalid_rels=($(check_relationship_types "$file"))
        if [ ${#invalid_rels[@]} -ne 0 ]; then
            echo -e "${CYAN}To fix invalid relationship types:${NC}"
            echo -e "  Replace these relationships with valid types from vocabulary.mdc:"
            for rel in "${invalid_rels[@]}"; do
                echo -e "    - $rel -> Choose from: CONTAINS, PART_OF, MANAGES, TRIGGERS, etc."
            done
        fi
    fi
    
    echo -e "${CYAN}After making fixes, run this script again to verify${NC}"
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
            suggest_fixes "$file"
        fi
        echo ""
    done
}

# Check if it's a single file or directory
if [ -f "$CHECK_PATH" ]; then
    total_files=1
    if check_file "$CHECK_PATH"; then
        compliant_files=1
        non_compliant_files=0
    else
        compliant_files=0
        non_compliant_files=1
        suggest_fixes "$CHECK_PATH"
    fi
else
    # Start processing directory
    process_directory "$CHECK_PATH"
fi

# Print summary
echo -e "${BLUE}=== Summary ===${NC}"
echo -e "Total files checked: $total_files"
echo -e "${GREEN}Compliant files: $compliant_files${NC}"
echo -e "${YELLOW}Non-compliant files: $non_compliant_files${NC}"

exit $non_compliant_files 