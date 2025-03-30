#!/bin/bash
#
# vocabulary_manager.sh
#
# A comprehensive script to manage the vocabulary workflow:
#   /notes -> /docs -> /vocabulary -> /code
#
# This script helps maintain consistent domain vocabulary across the project
# by processing the workflow steps and ensuring proper alignment.
#
# Usage: ./scripts/vocabulary_manager.sh [--extract | --align | --validate | --report | --help]
#

# Set default action
ACTION="help"
if [ $# -gt 0 ]; then
    case "$1" in
        --extract) ACTION="extract" ;;
        --align) ACTION="align" ;;
        --validate) ACTION="validate" ;;
        --report) ACTION="report" ;;
        --help) ACTION="help" ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Directories
NOTES_DIR="./notes"
INCOMING_DIR="./notes/incoming"
DOCS_DIR="./docs"
VOCABULARY_DIR="./vocabulary/domains"
TAXONOMIES_DIR="./vocabulary/taxonomies"
ONTOLOGIES_DIR="./vocabulary/ontologies"
KNOWLEDGE_DIR="./vocabulary/knowledge"
TEMP_DIR="/tmp/vocab_manager"

# Ensure temp directory exists
mkdir -p $TEMP_DIR

# Check if all required directories exist and create them if necessary
check_directories() {
    echo -e "${BLUE}Checking required directories...${NC}"
    
    # List of directories to check
    DIRS=("$NOTES_DIR" "$INCOMING_DIR" "$DOCS_DIR" "$VOCABULARY_DIR" "$TAXONOMIES_DIR" "$ONTOLOGIES_DIR" "$KNOWLEDGE_DIR")
    
    for dir in "${DIRS[@]}"; do
        if [ ! -d "$dir" ]; then
            echo -e "${YELLOW}Creating directory: $dir${NC}"
            mkdir -p "$dir"
        else
            echo -e "${GREEN}Directory exists: $dir${NC}"
        fi
    done
}

# Extract terms from notes
extract_terms_from_notes() {
    echo -e "${BLUE}Extracting terms from notes...${NC}"
    
    # Clear previous results
    rm -f $TEMP_DIR/note_terms.txt
    
    # Find all markdown files in notes directory (excluding incoming)
    find "$NOTES_DIR" -type f -name "*.md" -not -path "$INCOMING_DIR/*" -print0 | while IFS= read -r -d '' file; do
        # Extract headings (potential terms)
        grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.*)/\1/' >> $TEMP_DIR/note_terms.txt
        
        # Extract emphasized text (potential terms)
        grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*[^*]+\*\*" | sed -E 's/\*\*(.+)\*\*/\1/' >> $TEMP_DIR/note_terms.txt
    done
    
    # Sort and get unique terms
    if [ -f "$TEMP_DIR/note_terms.txt" ]; then
        sort $TEMP_DIR/note_terms.txt | uniq > $TEMP_DIR/note_terms_unique.txt
        NOTE_TERMS_COUNT=$(wc -l < $TEMP_DIR/note_terms_unique.txt)
        echo -e "${GREEN}Extracted $NOTE_TERMS_COUNT unique terms from notes${NC}"
    else
        echo -e "${YELLOW}No terms extracted from notes${NC}"
    fi
}

# Extract terms from incoming documents
extract_terms_from_incoming() {
    echo -e "${BLUE}Extracting terms from incoming documents...${NC}"
    
    # Check if incoming directory exists
    if [ ! -d "$INCOMING_DIR" ]; then
        echo -e "${YELLOW}Creating incoming directory: $INCOMING_DIR${NC}"
        mkdir -p "$INCOMING_DIR"
    fi
    
    # Clear previous results
    rm -f $TEMP_DIR/incoming_terms.txt
    
    # Count files in incoming directory
    INCOMING_FILES=$(find "$INCOMING_DIR" -type f -name "*.md" | wc -l)
    
    if [ "$INCOMING_FILES" -eq 0 ]; then
        echo -e "${YELLOW}No documents found in the incoming directory${NC}"
        return
    fi
    
    echo -e "${GREEN}Found $INCOMING_FILES documents in incoming directory${NC}"
    
    # Extract terms from incoming documents
    find "$INCOMING_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
        # Extract headings (potential terms)
        grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.+)/\1/' >> $TEMP_DIR/incoming_terms.txt
        
        # Extract emphasized text (potential terms)
        grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*[^*]+\*\*" | sed -E 's/\*\*(.+)\*\*/\1/' >> $TEMP_DIR/incoming_terms.txt
    done
    
    # Sort and get unique terms
    if [ -f "$TEMP_DIR/incoming_terms.txt" ]; then
        sort $TEMP_DIR/incoming_terms.txt | uniq > $TEMP_DIR/incoming_terms_unique.txt
        INCOMING_TERMS_COUNT=$(wc -l < $TEMP_DIR/incoming_terms_unique.txt)
        echo -e "${GREEN}Extracted $INCOMING_TERMS_COUNT unique terms from incoming documents${NC}"
    else
        echo -e "${YELLOW}No terms extracted from incoming documents${NC}"
    fi
}

# Extract terms from docs
extract_terms_from_docs() {
    echo -e "${BLUE}Extracting terms from docs...${NC}"
    
    # Clear previous results
    rm -f $TEMP_DIR/doc_terms.txt
    
    # Find all markdown files in docs directory
    find "$DOCS_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
        # Extract headings (potential terms)
        grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.*)/\1/' >> $TEMP_DIR/doc_terms.txt
        
        # Extract emphasized text (potential terms)
        grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*[^*]+\*\*" | sed -E 's/\*\*(.+)\*\*/\1/' >> $TEMP_DIR/doc_terms.txt
    done
    
    # Sort and get unique terms
    if [ -f "$TEMP_DIR/doc_terms.txt" ]; then
        sort $TEMP_DIR/doc_terms.txt | uniq > $TEMP_DIR/doc_terms_unique.txt
        DOC_TERMS_COUNT=$(wc -l < $TEMP_DIR/doc_terms_unique.txt)
        echo -e "${GREEN}Extracted $DOC_TERMS_COUNT unique terms from docs${NC}"
    else
        echo -e "${YELLOW}No terms extracted from docs${NC}"
    fi
}

# Extract terms from vocabulary
extract_terms_from_vocabulary() {
    echo -e "${BLUE}Extracting terms from vocabulary...${NC}"
    
    # Clear previous results
    rm -f $TEMP_DIR/vocab_terms.txt
    
    # Check if vocabulary directory exists
    if [ ! -d "$VOCABULARY_DIR" ]; then
        echo -e "${YELLOW}Vocabulary directory not found. Creating...${NC}"
        mkdir -p "$VOCABULARY_DIR"
        echo -e "${YELLOW}No vocabulary terms found${NC}"
        return
    fi
    
    # Get all term definitions from Cypher nodes
    grep -r "term:" "$VOCABULARY_DIR" --include="*.md" | sed -E 's/.*term: "(.*)",/\1/' > $TEMP_DIR/vocab_terms.txt
    
    # Sort and get unique terms
    if [ -s "$TEMP_DIR/vocab_terms.txt" ]; then
        sort $TEMP_DIR/vocab_terms.txt | uniq > $TEMP_DIR/vocab_terms_unique.txt
        VOCAB_TERMS_COUNT=$(wc -l < $TEMP_DIR/vocab_terms_unique.txt)
        echo -e "${GREEN}Extracted $VOCAB_TERMS_COUNT unique terms from vocabulary${NC}"
    else
        echo -e "${YELLOW}No vocabulary terms found${NC}"
    fi
}

# Function to suggest new terms from notes/incoming to add to docs
suggest_new_terms_for_docs() {
    echo -e "${BLUE}Suggesting new terms for docs...${NC}"
    
    # Check if we have both note terms and doc terms
    if [ ! -f "$TEMP_DIR/note_terms_unique.txt" ] || [ ! -f "$TEMP_DIR/doc_terms_unique.txt" ]; then
        echo -e "${YELLOW}Missing input files${NC}"
        return
    fi
    
    # Find terms in notes/incoming but not in docs
    cat $TEMP_DIR/note_terms_unique.txt $TEMP_DIR/incoming_terms_unique.txt 2>/dev/null | sort | uniq > $TEMP_DIR/all_source_terms.txt
    comm -23 $TEMP_DIR/all_source_terms.txt $TEMP_DIR/doc_terms_unique.txt > $TEMP_DIR/terms_to_add_to_docs.txt
    
    NEW_TERMS_COUNT=$(wc -l < $TEMP_DIR/terms_to_add_to_docs.txt)
    
    if [ "$NEW_TERMS_COUNT" -eq 0 ]; then
        echo -e "${GREEN}No new terms to add to docs${NC}"
    else
        echo -e "${YELLOW}Found $NEW_TERMS_COUNT terms to add to docs:${NC}"
        cat $TEMP_DIR/terms_to_add_to_docs.txt | head -n 10 | while read -r term; do
            echo -e "  - $term"
        done
        
        if [ "$NEW_TERMS_COUNT" -gt 10 ]; then
            echo -e "${YELLOW}  ... and $(($NEW_TERMS_COUNT - 10)) more${NC}"
        fi
    fi
}

# Function to suggest new terms from docs to add to vocabulary
suggest_new_terms_for_vocabulary() {
    echo -e "${BLUE}Suggesting new terms for vocabulary...${NC}"
    
    # Check if we have both doc terms and vocabulary terms
    if [ ! -f "$TEMP_DIR/doc_terms_unique.txt" ]; then
        echo -e "${YELLOW}Missing doc terms file${NC}"
        return
    fi
    
    # Create empty vocab terms file if it doesn't exist
    if [ ! -f "$TEMP_DIR/vocab_terms_unique.txt" ]; then
        touch $TEMP_DIR/vocab_terms_unique.txt
    fi
    
    # Find terms in docs but not in vocabulary
    comm -23 $TEMP_DIR/doc_terms_unique.txt $TEMP_DIR/vocab_terms_unique.txt > $TEMP_DIR/terms_to_add_to_vocab.txt
    
    NEW_VOCAB_TERMS_COUNT=$(wc -l < $TEMP_DIR/terms_to_add_to_vocab.txt)
    
    if [ "$NEW_VOCAB_TERMS_COUNT" -eq 0 ]; then
        echo -e "${GREEN}No new terms to add to vocabulary${NC}"
    else
        echo -e "${YELLOW}Found $NEW_VOCAB_TERMS_COUNT terms to add to vocabulary:${NC}"
        cat $TEMP_DIR/terms_to_add_to_vocab.txt | head -n 10 | while read -r term; do
            echo -e "  - $term"
        done
        
        if [ "$NEW_VOCAB_TERMS_COUNT" -gt 10 ]; then
            echo -e "${YELLOW}  ... and $(($NEW_VOCAB_TERMS_COUNT - 10)) more${NC}"
        fi
    fi
}

# Function to validate vocabulary structure
validate_vocabulary() {
    echo -e "${BLUE}Validating vocabulary structure...${NC}"
    
    # Check if vocabulary directories exist
    for dir in "$VOCABULARY_DIR" "$TAXONOMIES_DIR" "$ONTOLOGIES_DIR" "$KNOWLEDGE_DIR"; do
        if [ ! -d "$dir" ]; then
            echo -e "${RED}Directory does not exist: $dir${NC}"
            return 1
        fi
    fi
    
    # Check for domain vocabulary files
    DOMAIN_FILES=$(find "$VOCABULARY_DIR" -type f -name "*.md" | wc -l)
    if [ "$DOMAIN_FILES" -eq 0 ]; then
        echo -e "${RED}No domain vocabulary files found${NC}"
    else
        echo -e "${GREEN}Found $DOMAIN_FILES domain vocabulary files${NC}"
    fi
    
    # Validate Cypher format in vocabulary files
    CYPHER_NODE_COUNT=$(grep -r "(:.*:.*{" "$VOCABULARY_DIR" --include="*.md" | wc -l)
    CYPHER_REL_COUNT=$(grep -r "-\[:" "$VOCABULARY_DIR" --include="*.md" | wc -l)
    
    echo -e "${GREEN}Found $CYPHER_NODE_COUNT node definitions and $CYPHER_REL_COUNT relationship definitions${NC}"
    
    # Check for required properties
    MISSING_TERM_COUNT=$(grep -r "(:.*:.*{" "$VOCABULARY_DIR" --include="*.md" | grep -v "term:" | wc -l)
    MISSING_DOMAIN_COUNT=$(grep -r "(:.*:.*{" "$VOCABULARY_DIR" --include="*.md" | grep -v "domain:" | wc -l)
    
    if [ "$MISSING_TERM_COUNT" -gt 0 ]; then
        echo -e "${RED}Found $MISSING_TERM_COUNT node definitions missing 'term' property${NC}"
    else
        echo -e "${GREEN}All node definitions include 'term' property${NC}"
    fi
    
    if [ "$MISSING_DOMAIN_COUNT" -gt 0 ]; then
        echo -e "${RED}Found $MISSING_DOMAIN_COUNT node definitions missing 'domain' property${NC}"
    else
        echo -e "${GREEN}All node definitions include 'domain' property${NC}"
    fi
    
    # Check taxonomy files
    TAXONOMY_FILES=$(find "$TAXONOMIES_DIR" -type f -name "*.md" | wc -l)
    echo -e "${GREEN}Found $TAXONOMY_FILES taxonomy files${NC}"
    
    # Check ontology files
    ONTOLOGY_FILES=$(find "$ONTOLOGIES_DIR" -type f -name "*.md" | wc -l)
    echo -e "${GREEN}Found $ONTOLOGY_FILES ontology files${NC}"
}

# Function to generate alignment report
generate_report() {
    echo -e "${BLUE}Generating vocabulary alignment report...${NC}"
    
    REPORT_FILE="$DOCS_DIR/vocabulary_alignment_report.md"
    
    # Make sure all terms are extracted
    extract_terms_from_notes
    extract_terms_from_incoming
    extract_terms_from_docs
    extract_terms_from_vocabulary
    
    # Calculate metrics
    NOTES_TERMS_COUNT=$(wc -l < $TEMP_DIR/note_terms_unique.txt 2>/dev/null || echo 0)
    INCOMING_TERMS_COUNT=$(wc -l < $TEMP_DIR/incoming_terms_unique.txt 2>/dev/null || echo 0)
    DOCS_TERMS_COUNT=$(wc -l < $TEMP_DIR/doc_terms_unique.txt 2>/dev/null || echo 0)
    VOCAB_TERMS_COUNT=$(wc -l < $TEMP_DIR/vocab_terms_unique.txt 2>/dev/null || echo 0)
    
    # Terms in notes/incoming but not in docs
    if [ -f "$TEMP_DIR/note_terms_unique.txt" ] && [ -f "$TEMP_DIR/incoming_terms_unique.txt" ] && [ -f "$TEMP_DIR/doc_terms_unique.txt" ]; then
        cat $TEMP_DIR/note_terms_unique.txt $TEMP_DIR/incoming_terms_unique.txt | sort | uniq > $TEMP_DIR/all_source_terms.txt
        comm -23 $TEMP_DIR/all_source_terms.txt $TEMP_DIR/doc_terms_unique.txt > $TEMP_DIR/terms_to_add_to_docs.txt
        TERMS_TO_ADD_TO_DOCS=$(wc -l < $TEMP_DIR/terms_to_add_to_docs.txt)
    else
        TERMS_TO_ADD_TO_DOCS=0
    fi
    
    # Terms in docs but not in vocabulary
    if [ -f "$TEMP_DIR/doc_terms_unique.txt" ] && [ -f "$TEMP_DIR/vocab_terms_unique.txt" ]; then
        comm -23 $TEMP_DIR/doc_terms_unique.txt $TEMP_DIR/vocab_terms_unique.txt > $TEMP_DIR/terms_to_add_to_vocab.txt
        TERMS_TO_ADD_TO_VOCAB=$(wc -l < $TEMP_DIR/terms_to_add_to_vocab.txt)
    else
        TERMS_TO_ADD_TO_VOCAB=0
    fi
    
    # Calculate coverage percentages
    if [ "$DOCS_TERMS_COUNT" -gt 0 ] && [ "$NOTES_TERMS_COUNT" -gt 0 ]; then
        NOTES_TO_DOCS_FLOW=$(( (NOTES_TERMS_COUNT - TERMS_TO_ADD_TO_DOCS) * 100 / NOTES_TERMS_COUNT ))
    else
        NOTES_TO_DOCS_FLOW=0
    fi
    
    if [ "$VOCAB_TERMS_COUNT" -gt 0 ] && [ "$DOCS_TERMS_COUNT" -gt 0 ]; then
        DOCS_TO_VOCAB_FLOW=$(( (DOCS_TERMS_COUNT - TERMS_TO_ADD_TO_VOCAB) * 100 / DOCS_TERMS_COUNT ))
    else
        DOCS_TO_VOCAB_FLOW=0
    fi
    
    # Generate report markdown
    cat > "$REPORT_FILE" << EOF
# Vocabulary Alignment Report

This report analyzes how well the project follows the workflow:
\`/notes -> /docs -> /vocabulary -> /code\`

## Summary

- **Notes Terms**: $NOTES_TERMS_COUNT terms found in notes
- **Incoming Terms**: $INCOMING_TERMS_COUNT terms found in incoming documents
- **Docs Terms**: $DOCS_TERMS_COUNT terms found in documentation
- **Vocabulary Terms**: $VOCAB_TERMS_COUNT terms defined in vocabulary

## Workflow Metrics

- **Notes -> Docs Flow**: $NOTES_TO_DOCS_FLOW% coverage
  - $TERMS_TO_ADD_TO_DOCS terms from notes/incoming not yet in docs
- **Docs -> Vocabulary Flow**: $DOCS_TO_VOCAB_FLOW% coverage
  - $TERMS_TO_ADD_TO_VOCAB terms from docs not yet in vocabulary

## Recommendations

EOF
    
    # Add recommendations based on analysis
    if [ "$TERMS_TO_ADD_TO_DOCS" -gt 0 ]; then
        echo "### Terms to Add to Documentation" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        cat $TEMP_DIR/terms_to_add_to_docs.txt | head -n 20 | while read -r term; do
            echo "- \`$term\`" >> "$REPORT_FILE"
        done
        
        if [ "$TERMS_TO_ADD_TO_DOCS" -gt 20 ]; then
            echo "- ... and $(($TERMS_TO_ADD_TO_DOCS - 20)) more" >> "$REPORT_FILE"
        fi
        echo "" >> "$REPORT_FILE"
    fi
    
    if [ "$TERMS_TO_ADD_TO_VOCAB" -gt 0 ]; then
        echo "### Terms to Add to Vocabulary" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        cat $TEMP_DIR/terms_to_add_to_vocab.txt | head -n 20 | while read -r term; do
            echo "- \`$term\`" >> "$REPORT_FILE"
        done
        
        if [ "$TERMS_TO_ADD_TO_VOCAB" -gt 20 ]; then
            echo "- ... and $(($TERMS_TO_ADD_TO_VOCAB - 20)) more" >> "$REPORT_FILE"
        fi
        echo "" >> "$REPORT_FILE"
    fi
    
    echo -e "${GREEN}Report generated: $REPORT_FILE${NC}"
}

# Process incoming documents
process_incoming_documents() {
    echo -e "${BLUE}Processing incoming documents...${NC}"
    
    # Check if incoming directory exists and has files
    if [ ! -d "$INCOMING_DIR" ]; then
        echo -e "${YELLOW}Incoming directory not found${NC}"
        return
    fi
    
    INCOMING_FILES=$(find "$INCOMING_DIR" -type f -name "*.md" | wc -l)
    
    if [ "$INCOMING_FILES" -eq 0 ]; then
        echo -e "${YELLOW}No documents found in the incoming directory${NC}"
        return
    fi
    
    echo -e "${GREEN}Found $INCOMING_FILES documents in incoming directory${NC}"
    echo -e "${YELLOW}Processing incoming documents and extracting claims...${NC}"
    
    # Process each incoming document
    find "$INCOMING_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
        filename=$(basename "$file")
        echo -e "${CYAN}Processing: $filename${NC}"
        
        # Create a claims directory if it doesn't exist
        if [ ! -d "$NOTES_DIR/claims" ]; then
            mkdir -p "$NOTES_DIR/claims"
        fi
        
        # Extract claims from the document
        grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.+)/\1/' > "$TEMP_DIR/claims.txt"
        
        # Create a claim file
        CLAIM_FILE="$NOTES_DIR/claims/$(date +%Y-%m-%d)-$(echo $filename | sed 's/\.md/-claims.md/')"
        
        cat > "$CLAIM_FILE" << EOF
# Claims from $filename

_Extracted on $(date)_

## Source Document
\`$file\`

## Extracted Claims

EOF
        
        # Add claims to the file
        if [ -s "$TEMP_DIR/claims.txt" ]; then
            cat "$TEMP_DIR/claims.txt" | while read -r claim; do
                echo "- $claim" >> "$CLAIM_FILE"
            done
        else
            echo "No claims found in this document." >> "$CLAIM_FILE"
        fi
        
        echo -e "${GREEN}Created claims file: $CLAIM_FILE${NC}"
    done
    
    echo -e "${YELLOW}Do you want to move processed documents out of the incoming directory? (y/n)${NC}"
    read -r response
    
    if [[ "$response" =~ ^[Yy]$ ]]; then
        # Create an archive directory
        ARCHIVE_DIR="$NOTES_DIR/processed"
        mkdir -p "$ARCHIVE_DIR"
        
        # Move files
        find "$INCOMING_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
            filename=$(basename "$file")
            mv "$file" "$ARCHIVE_DIR/$filename"
            echo -e "${GREEN}Moved $filename to $ARCHIVE_DIR${NC}"
        done
    fi
}

# Main function that processes the chosen action
main() {
    case "$ACTION" in
        extract)
            check_directories
            extract_terms_from_notes
            extract_terms_from_incoming
            extract_terms_from_docs
            extract_terms_from_vocabulary
            ;;
        align)
            check_directories
            extract_terms_from_notes
            extract_terms_from_incoming
            extract_terms_from_docs
            extract_terms_from_vocabulary
            suggest_new_terms_for_docs
            suggest_new_terms_for_vocabulary
            process_incoming_documents
            ;;
        validate)
            check_directories
            validate_vocabulary
            ;;
        report)
            check_directories
            generate_report
            ;;
        help|*)
            echo -e "${BLUE}Vocabulary Manager${NC}"
            echo ""
            echo "Usage: ./scripts/vocabulary_manager.sh [OPTION]"
            echo ""
            echo "Options:"
            echo "  --extract    Extract terms from notes, docs, and vocabulary"
            echo "  --align      Suggest terms to add to docs and vocabulary"
            echo "  --validate   Validate vocabulary structure and format"
            echo "  --report     Generate vocabulary alignment report"
            echo "  --help       Display this help message"
            ;;
    esac
}

# Run the main function
main 