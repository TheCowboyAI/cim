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

echo -e "${BLUE}=== Enhanced Vocabulary Standardization Tool ===${NC}"
echo -e "${BLUE}Checking files in: ${CHECK_PATH}${NC}"
echo ""

# Required sections according to vocabulary.mdc
REQUIRED_SECTIONS=("Domain Objects" "Relationships" "Taxonomies" "Usage Contexts" "Code References")

# Required term properties
REQUIRED_PROPERTIES=("domain" "term" "taxonomy" "definition" "usage_context" "code_reference")

# Relationship types
HIERARCHICAL_TYPES=("IS_A" "PART_OF" "CONTAINS" "EXTENDS")
FUNCTIONAL_TYPES=("MANAGES" "PROCESSES" "VALIDATES" "CONFIGURES")
TEMPORAL_TYPES=("PRECEDES" "FOLLOWS" "TRIGGERS" "DEPENDS_ON")

# Track statistics
total_files=0
compliant_files=0
non_compliant_files=0
cypher_format_files=0
non_cypher_format_files=0
missing_properties_files=0
missing_sections_files=0

# Storage for categorized files
declare -A domain_files
declare -A complexity_files
declare -A issues_by_file

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

# Function to extract domain name from file path
extract_domain() {
    local file=$1
    local domain
    
    # Try to extract from the path structure
    if [[ $file == *"/domains/"* ]]; then
        domain=$(echo "$file" | grep -o "/domains/[^/]*" | sed 's|/domains/||')
        domain=${domain%.md}
    elif [[ $file == *"/taxonomies/"* ]]; then
        domain=$(echo "$file" | grep -o "/taxonomies/[^/]*" | sed 's|/taxonomies/||')
        domain=${domain%.md}
        domain=${domain%_processing}
    elif [[ $file == *"/ontologies/"* ]]; then
        domain=$(echo "$file" | grep -o "/ontologies/[^/]*" | sed 's|/ontologies/||')
        domain=${domain%.md}
    else
        # Default fallback
        domain=$(basename "$file" .md)
    fi
    
    echo "$domain"
}

# Function to determine file complexity
determine_complexity() {
    local file=$1
    local issues=0
    local complexity="low"
    
    # Check for Cypher format
    if ! check_cypher_format "$file"; then
        ((issues+=3)) # Higher weight for this issue
    fi
    
    # Check missing sections
    local missing_sections=($(check_required_sections "$file"))
    issues=$((issues + ${#missing_sections[@]}))
    
    # Check missing properties
    local missing_props=($(check_missing_properties "$file"))
    issues=$((issues + ${#missing_props[@]}))
    
    # Determine complexity based on issues
    if [ $issues -ge 5 ]; then
        complexity="high"
    elif [ $issues -ge 2 ]; then
        complexity="medium"
    fi
    
    echo "$complexity"
}

# Main function to check a file
check_file() {
    local file=$1
    local issues=0
    local domain=$(extract_domain "$file")
    local complexity
    local missing_props=()
    local missing_sects=()
    
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
        ((non_cypher_format_files++))
    else
        ((cypher_format_files++))
    fi
    
    # Check if file has all required sections
    missing_sects=($(check_required_sections "$file"))
    if [ ${#missing_sects[@]} -ne 0 ]; then
        echo -e "${YELLOW}  Missing sections:${NC}"
        for section in "${missing_sects[@]}"; do
            echo -e "    - $section"
        done
        ((issues++))
        ((missing_sections_files++))
    fi
    
    # Check if any term is missing required properties
    if grep -q "(:.*:.*{" "$file"; then
        missing_props=($(check_missing_properties "$file"))
        if [ ${#missing_props[@]} -ne 0 ]; then
            echo -e "${YELLOW}  Missing properties in terms:${NC}"
            for prop in "${missing_props[@]}"; do
                echo -e "    - $prop"
            done
            ((issues++))
            ((missing_properties_files++))
        fi
    fi
    
    # Determine complexity
    complexity=$(determine_complexity "$file")
    
    # Store file in appropriate categories
    domain_files["$domain"]+=" $file"
    complexity_files["$complexity"]+=" $file"
    
    # Store issues by file
    issues_by_file["$file"]="format:$(check_cypher_format "$file" && echo "yes" || echo "no") missing_props:${missing_props[*]} missing_sections:${missing_sects[*]} complexity:$complexity domain:$domain"
    
    # Report compliance status
    if [ $issues -eq 0 ]; then
        echo -e "${GREEN}  File is compliant with vocabulary.mdc rules${NC}"
        return 0
    else
        echo -e "${YELLOW}  Found $issues issues to fix (complexity: $complexity)${NC}"
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
            echo -e "${YELLOW}  See template file: vocabulary/domains/knowledge_domain.md${NC}"
        fi
        echo ""
    done
}

# Generate standardization plan
generate_plan() {
    echo -e "${BLUE}=== Standardization Plan ===${NC}"
    
    # Plan by complexity
    echo -e "${CYAN}Files by Complexity:${NC}"
    echo -e "${GREEN}Low Complexity (Start with these):${NC}"
    for file in ${complexity_files["low"]}; do
        echo -e "  - $file"
    done
    echo -e "${YELLOW}Medium Complexity:${NC}"
    for file in ${complexity_files["medium"]}; do
        echo -e "  - $file"
    done
    echo -e "${RED}High Complexity:${NC}"
    for file in ${complexity_files["high"]}; do
        echo -e "  - $file"
    done
    
    # Plan by domain
    echo -e "\n${CYAN}Files by Domain:${NC}"
    for domain in "${!domain_files[@]}"; do
        echo -e "${CYAN}Domain: $domain${NC}"
        for file in ${domain_files["$domain"]}; do
            local issues="${issues_by_file["$file"]}"
            local complexity=$(echo "$issues" | grep -o "complexity:[^ ]*" | cut -d: -f2)
            case "$complexity" in
                "low") color=$GREEN ;;
                "medium") color=$YELLOW ;;
                "high") color=$RED ;;
                *) color=$NC ;;
            esac
            echo -e "  - ${color}$file${NC} (${issues})"
        done
    done
    
    # Generate task checklist
    echo -e "\n${CYAN}Task Checklist (in recommended order):${NC}"
    
    # First, files that are already in Cypher format but just missing properties
    if [ $cypher_format_files -gt 0 ] && [ $missing_properties_files -gt 0 ]; then
        echo -e "\n${GREEN}Step 1: Fix files with Cypher format but missing properties${NC}"
        for file in ${md_files[@]}; do
            issues="${issues_by_file["$file"]}"
            if [[ "$issues" == *"format:yes"* ]] && [[ "$issues" == *"missing_props:"*[a-z]* ]]; then
                domain=$(echo "$issues" | grep -o "domain:[^ ]*" | cut -d: -f2)
                missing=$(echo "$issues" | grep -o "missing_props:[^ ]*" | cut -d: -f2-)
                echo -e "  [ ] $file (Domain: $domain, Missing: $missing)"
            fi
        done
    fi
    
    # Then, files missing sections but have some Cypher format
    if [ $missing_sections_files -gt 0 ]; then
        echo -e "\n${YELLOW}Step 2: Fix files missing required sections${NC}"
        for file in ${md_files[@]}; do
            issues="${issues_by_file["$file"]}"
            if [[ "$issues" == *"format:yes"* ]] && [[ "$issues" == *"missing_sections:"*[A-Z]* ]]; then
                domain=$(echo "$issues" | grep -o "domain:[^ ]*" | cut -d: -f2)
                missing=$(echo "$issues" | grep -o "missing_sections:[^ ]*" | cut -d: -f2-)
                echo -e "  [ ] $file (Domain: $domain, Missing sections: $missing)"
            fi
        done
    fi
    
    # Finally, files without Cypher format
    if [ $non_cypher_format_files -gt 0 ]; then
        echo -e "\n${RED}Step 3: Fix files without Cypher format${NC}"
        for file in ${md_files[@]}; do
            issues="${issues_by_file["$file"]}"
            if [[ "$issues" == *"format:no"* ]]; then
                domain=$(echo "$issues" | grep -o "domain:[^ ]*" | cut -d: -f2)
                echo -e "  [ ] $file (Domain: $domain)"
            fi
        done
    fi
    
    # Generate recommended commands
    echo -e "\n${CYAN}Recommended Commands:${NC}"
    echo -e "1. Verify file after standardization:"
    echo -e "   ${YELLOW}./scripts/standardize_vocabulary.sh path/to/file.md${NC}"
    echo -e "2. Check progress on all files:"
    echo -e "   ${YELLOW}./scripts/standardize_vocabulary_enhanced.sh${NC}"
    echo -e "3. Copy template file:"
    echo -e "   ${YELLOW}cp vocabulary/domains/template.md vocabulary/domains/new_domain.md${NC}"
    echo -e "4. Check domain-specific files:"
    echo -e "   ${YELLOW}./scripts/standardize_vocabulary.sh vocabulary/domains/specific_domain*.md${NC}"
}

# Start processing
process_directory "$CHECK_PATH"

# Print summary
echo -e "${BLUE}=== Summary ===${NC}"
echo -e "Total files checked: $total_files"
echo -e "${GREEN}Compliant files: $compliant_files${NC}"
echo -e "${YELLOW}Non-compliant files: $non_compliant_files${NC}"
echo -e "\nIssue breakdown:"
echo -e "  Files without Cypher format: $non_cypher_format_files"
echo -e "  Files missing properties: $missing_properties_files"
echo -e "  Files missing sections: $missing_sections_files"
echo ""
echo -e "${BLUE}References:${NC}"
echo -e "- Template: vocabulary/domains/knowledge_domain.md"
echo -e "- Rules: .cursor/rules/vocabulary.mdc"
echo ""

if [ $non_compliant_files -eq 0 ]; then
    echo -e "${GREEN}All files are compliant with vocabulary.mdc rules!${NC}"
else
    # Generate standardization plan
    generate_plan
fi

exit $non_compliant_files 