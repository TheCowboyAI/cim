#!/bin/bash
#
# workflow_analyzer.sh
#
# Script to analyze and validate the workflow described in vocabulary.mdc:
#   /notes -> /docs -> /vocabulary -> /code
#
# This script checks for proper flow of knowledge and terminology across these directories
# and identifies any gaps or inconsistencies in the knowledge flow.
#
# Usage: ./scripts/workflow_analyzer.sh [--detailed]
#

# Check for detailed output flag
DETAILED=false
if [ "$1" = "--detailed" ]; then
    DETAILED=true
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Define directories to check
NOTES_DIR="./notes"
DOCS_DIR="./docs"
VOCAB_DIR="./vocabulary/domains"
CODE_DIR="./cim/src"

# Output file for the report
REPORT_FILE="docs/workflow_analysis_report.md"

echo -e "${BLUE}=== Vocabulary Workflow Analyzer ===${NC}"
echo -e "${BLUE}Analyzing knowledge flow: /notes -> /docs -> /vocabulary -> /code${NC}"
echo ""

# Create backup of existing report if it exists
if [ -f "$REPORT_FILE" ]; then
    cp "$REPORT_FILE" "${REPORT_FILE}.bak"
    echo -e "${BLUE}Created backup of existing report at: ${REPORT_FILE}.bak${NC}"
fi

# Check if directories exist
for dir in "$NOTES_DIR" "$DOCS_DIR" "$VOCAB_DIR" "$CODE_DIR"; do
    if [ ! -d "$dir" ]; then
        echo -e "${RED}Directory does not exist: $dir${NC}"
        echo -e "${YELLOW}Please ensure all required directories exist before running this script.${NC}"
        exit 1
    fi
done

# Function to extract terms from notes
extract_terms_from_notes() {
    echo -e "${BLUE}Extracting terms from notes...${NC}"
    
    # Find all Markdown files in notes directory
    find "$NOTES_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
        # Extract headings (potential terms)
        grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.*)/\1/' >> /tmp/note_terms.txt
        
        # Extract emphasized text (potential terms)
        grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*.*?\*\*" | sed -E 's/\*\*(.*?)\*\*/\1/g' >> /tmp/note_terms.txt
    done
    
    # Sort and count unique terms
    sort /tmp/note_terms.txt | uniq -c | sort -nr > /tmp/note_terms_count.txt
    
    # Get count of unique terms
    NOTE_TERMS_COUNT=$(wc -l < /tmp/note_terms_count.txt)
    echo -e "${GREEN}Extracted $NOTE_TERMS_COUNT unique terms from notes.${NC}"
}

# Function to extract terms from docs
extract_terms_from_docs() {
    echo -e "${BLUE}Extracting terms from docs...${NC}"
    
    # Find all Markdown files in docs directory
    find "$DOCS_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
        # Extract headings (potential terms)
        grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.*)/\1/' >> /tmp/doc_terms.txt
        
        # Extract emphasized text (potential terms)
        grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*.*?\*\*" | sed -E 's/\*\*(.*?)\*\*/\1/g' >> /tmp/doc_terms.txt
    done
    
    # Sort and count unique terms
    sort /tmp/doc_terms.txt | uniq -c | sort -nr > /tmp/doc_terms_count.txt
    
    # Get count of unique terms
    DOC_TERMS_COUNT=$(wc -l < /tmp/doc_terms_count.txt)
    echo -e "${GREEN}Extracted $DOC_TERMS_COUNT unique terms from docs.${NC}"
}

# Function to extract terms from vocabulary
extract_terms_from_vocabulary() {
    echo -e "${BLUE}Extracting terms from vocabulary...${NC}"
    
    # Get all term definitions from Cypher nodes
    grep -r "term:" "$VOCAB_DIR" --include="*.md" | sed -E 's/.*term: "(.*)",/\1/' > /tmp/vocab_terms.txt
    
    # Sort and count unique terms
    sort /tmp/vocab_terms.txt | uniq -c | sort -nr > /tmp/vocab_terms_count.txt
    
    # Get count of unique terms
    VOCAB_TERMS_COUNT=$(wc -l < /tmp/vocab_terms_count.txt)
    echo -e "${GREEN}Extracted $VOCAB_TERMS_COUNT unique terms from vocabulary.${NC}"
    
    # Extract domains
    grep -r "domain:" "$VOCAB_DIR" --include="*.md" | sed -E 's/.*domain: "(.*)",/\1/' | sort | uniq > /tmp/vocab_domains.txt
    
    # Extract term categories
    grep -r ":[A-Za-z0-9_]\+:[A-Za-z0-9_]\+" "$VOCAB_DIR" --include="*.md" | sed -E 's/.*:([A-Za-z0-9_]+):.*/\1/' | sort | uniq > /tmp/vocab_types.txt
}

# Function to extract terms from code
extract_terms_from_code() {
    echo -e "${BLUE}Extracting terms from code...${NC}"
    
    # Find all Rust files
    find "$CODE_DIR" -type f -name "*.rs" -print0 | while IFS= read -r -d '' file; do
        # Extract struct/enum/trait names
        grep -E "^(struct|enum|trait)\s+[A-Za-z0-9_]+" "$file" | sed -E 's/^(struct|enum|trait)\s+([A-Za-z0-9_]+).*/\2/' >> /tmp/code_terms.txt
        
        # Extract function names
        grep -E "^(pub\s+)?(async\s+)?fn\s+[A-Za-z0-9_]+" "$file" | sed -E 's/^(pub\s+)?(async\s+)?fn\s+([A-Za-z0-9_]+).*/\3/' >> /tmp/code_terms.txt
        
        # Extract module names
        grep -E "^(pub\s+)?mod\s+[A-Za-z0-9_]+" "$file" | sed -E 's/^(pub\s+)?mod\s+([A-Za-z0-9_]+).*/\2/' >> /tmp/code_terms.txt
    done
    
    # Sort and count unique terms
    sort /tmp/code_terms.txt | uniq -c | sort -nr > /tmp/code_terms_count.txt
    
    # Get count of unique terms
    CODE_TERMS_COUNT=$(wc -l < /tmp/code_terms_count.txt)
    echo -e "${GREEN}Extracted $CODE_TERMS_COUNT unique terms from code.${NC}"
}

# Function to analyze flow from notes to docs
analyze_notes_to_docs() {
    echo -e "${BLUE}Analyzing flow: notes -> docs...${NC}"
    
    # Compare terms in notes vs. docs
    grep -f /tmp/note_terms.txt /tmp/doc_terms.txt | sort | uniq > /tmp/notes_to_docs_common.txt
    NOTES_TO_DOCS_COMMON_COUNT=$(wc -l < /tmp/notes_to_docs_common.txt)
    NOTE_TERMS_COUNT=$(wc -l < /tmp/note_terms_count.txt)
    
    # Calculate coverage percentage
    if [ "$NOTE_TERMS_COUNT" -gt 0 ]; then
        NOTES_TO_DOCS_COVERAGE=$((NOTES_TO_DOCS_COMMON_COUNT * 100 / NOTE_TERMS_COUNT))
    else
        NOTES_TO_DOCS_COVERAGE=0
    fi
    
    echo -e "${GREEN}Notes -> Docs Coverage: $NOTES_TO_DOCS_COVERAGE%${NC}"
    echo -e "${GREEN}$NOTES_TO_DOCS_COMMON_COUNT of $NOTE_TERMS_COUNT note terms appear in docs${NC}"
    
    # Find terms in notes but not in docs
    grep -v -f /tmp/doc_terms.txt /tmp/note_terms.txt | sort | uniq > /tmp/notes_not_in_docs.txt
    NOTES_NOT_IN_DOCS_COUNT=$(wc -l < /tmp/notes_not_in_docs.txt)
    
    echo -e "${YELLOW}$NOTES_NOT_IN_DOCS_COUNT terms from notes are not in docs${NC}"
}

# Function to analyze flow from docs to vocabulary
analyze_docs_to_vocabulary() {
    echo -e "${BLUE}Analyzing flow: docs -> vocabulary...${NC}"
    
    # Compare terms in docs vs. vocabulary
    grep -f /tmp/doc_terms.txt /tmp/vocab_terms.txt | sort | uniq > /tmp/docs_to_vocab_common.txt
    DOCS_TO_VOCAB_COMMON_COUNT=$(wc -l < /tmp/docs_to_vocab_common.txt)
    DOC_TERMS_COUNT=$(wc -l < /tmp/doc_terms_count.txt)
    
    # Calculate coverage percentage
    if [ "$DOC_TERMS_COUNT" -gt 0 ]; then
        DOCS_TO_VOCAB_COVERAGE=$((DOCS_TO_VOCAB_COMMON_COUNT * 100 / DOC_TERMS_COUNT))
    else
        DOCS_TO_VOCAB_COVERAGE=0
    fi
    
    echo -e "${GREEN}Docs -> Vocabulary Coverage: $DOCS_TO_VOCAB_COVERAGE%${NC}"
    echo -e "${GREEN}$DOCS_TO_VOCAB_COMMON_COUNT of $DOC_TERMS_COUNT doc terms appear in vocabulary${NC}"
    
    # Find terms in docs but not in vocabulary
    grep -v -f /tmp/vocab_terms.txt /tmp/doc_terms.txt | sort | uniq > /tmp/docs_not_in_vocab.txt
    DOCS_NOT_IN_VOCAB_COUNT=$(wc -l < /tmp/docs_not_in_vocab.txt)
    
    echo -e "${YELLOW}$DOCS_NOT_IN_VOCAB_COUNT terms from docs are not in vocabulary${NC}"
}

# Function to analyze flow from vocabulary to code
analyze_vocabulary_to_code() {
    echo -e "${BLUE}Analyzing flow: vocabulary -> code...${NC}"
    
    # Compare terms in vocabulary vs. code
    grep -f /tmp/vocab_terms.txt /tmp/code_terms.txt | sort | uniq > /tmp/vocab_to_code_common.txt
    VOCAB_TO_CODE_COMMON_COUNT=$(wc -l < /tmp/vocab_to_code_common.txt)
    VOCAB_TERMS_COUNT=$(wc -l < /tmp/vocab_terms_count.txt)
    
    # Calculate coverage percentage
    if [ "$VOCAB_TERMS_COUNT" -gt 0 ]; then
        VOCAB_TO_CODE_COVERAGE=$((VOCAB_TO_CODE_COMMON_COUNT * 100 / VOCAB_TERMS_COUNT))
    else
        VOCAB_TO_CODE_COVERAGE=0
    fi
    
    echo -e "${GREEN}Vocabulary -> Code Coverage: $VOCAB_TO_CODE_COVERAGE%${NC}"
    echo -e "${GREEN}$VOCAB_TO_CODE_COMMON_COUNT of $VOCAB_TERMS_COUNT vocabulary terms appear in code${NC}"
    
    # Find terms in vocabulary but not in code
    grep -v -f /tmp/code_terms.txt /tmp/vocab_terms.txt | sort | uniq > /tmp/vocab_not_in_code.txt
    VOCAB_NOT_IN_CODE_COUNT=$(wc -l < /tmp/vocab_not_in_code.txt)
    
    echo -e "${YELLOW}$VOCAB_NOT_IN_CODE_COUNT terms from vocabulary are not in code${NC}"
    
    # Check code references in vocabulary
    CODE_REFS_COUNT=$(grep -r "code_reference:" "$VOCAB_DIR" --include="*.md" | grep -v "TBD" | wc -l)
    TBD_REFS_COUNT=$(grep -r "code_reference:" "$VOCAB_DIR" --include="*.md" | grep "TBD" | wc -l)
    TOTAL_REFS=$((CODE_REFS_COUNT + TBD_REFS_COUNT))
    
    # Calculate code reference coverage
    if [ "$TOTAL_REFS" -gt 0 ]; then
        CODE_REF_COVERAGE=$((CODE_REFS_COUNT * 100 / TOTAL_REFS))
    else
        CODE_REF_COVERAGE=0
    fi
    
    echo -e "${GREEN}Code Reference Coverage: $CODE_REF_COVERAGE%${NC}"
    echo -e "${GREEN}$CODE_REFS_COUNT of $TOTAL_REFS terms have code references${NC}"
}

# Function to analyze overall workflow
analyze_overall_workflow() {
    echo -e "${BLUE}Analyzing overall workflow...${NC}"
    
    # Calculate overall flow metrics
    NOTE_TERMS_COUNT=$(wc -l < /tmp/note_terms_count.txt)
    DOC_TERMS_COUNT=$(wc -l < /tmp/doc_terms_count.txt)
    VOCAB_TERMS_COUNT=$(wc -l < /tmp/vocab_terms_count.txt)
    CODE_TERMS_COUNT=$(wc -l < /tmp/code_terms_count.txt)
    
    # Calculate overall flow scores
    NOTES_TO_DOCS_COMMON_COUNT=$(wc -l < /tmp/notes_to_docs_common.txt)
    DOCS_TO_VOCAB_COMMON_COUNT=$(wc -l < /tmp/docs_to_vocab_common.txt)
    VOCAB_TO_CODE_COMMON_COUNT=$(wc -l < /tmp/vocab_to_code_common.txt)
    
    # Calculate percentages
    if [ "$NOTE_TERMS_COUNT" -gt 0 ]; then
        NOTES_TO_DOCS_COVERAGE=$((NOTES_TO_DOCS_COMMON_COUNT * 100 / NOTE_TERMS_COUNT))
    else
        NOTES_TO_DOCS_COVERAGE=0
    fi
    
    if [ "$DOC_TERMS_COUNT" -gt 0 ]; then
        DOCS_TO_VOCAB_COVERAGE=$((DOCS_TO_VOCAB_COMMON_COUNT * 100 / DOC_TERMS_COUNT))
    else
        DOCS_TO_VOCAB_COVERAGE=0
    fi
    
    if [ "$VOCAB_TERMS_COUNT" -gt 0 ]; then
        VOCAB_TO_CODE_COVERAGE=$((VOCAB_TO_CODE_COMMON_COUNT * 100 / VOCAB_TERMS_COUNT))
    else
        VOCAB_TO_CODE_COVERAGE=0
    fi
    
    # Calculate overall workflow score (average of the three percentages)
    OVERALL_WORKFLOW_SCORE=$(( (NOTES_TO_DOCS_COVERAGE + DOCS_TO_VOCAB_COVERAGE + VOCAB_TO_CODE_COVERAGE) / 3 ))
    
    echo -e "${GREEN}Overall Workflow Score: $OVERALL_WORKFLOW_SCORE%${NC}"
    echo -e "${GREEN}Notes -> Docs: $NOTES_TO_DOCS_COVERAGE%${NC}"
    echo -e "${GREEN}Docs -> Vocabulary: $DOCS_TO_VOCAB_COVERAGE%${NC}"
    echo -e "${GREEN}Vocabulary -> Code: $VOCAB_TO_CODE_COVERAGE%${NC}"
}

# Function to generate the analysis report
generate_report() {
    echo -e "${BLUE}Generating workflow analysis report...${NC}"
    
    # Calculate metrics for the report
    NOTE_TERMS_COUNT=$(wc -l < /tmp/note_terms_count.txt)
    DOC_TERMS_COUNT=$(wc -l < /tmp/doc_terms_count.txt)
    VOCAB_TERMS_COUNT=$(wc -l < /tmp/vocab_terms_count.txt)
    CODE_TERMS_COUNT=$(wc -l < /tmp/code_terms_count.txt)
    
    NOTES_TO_DOCS_COMMON_COUNT=$(wc -l < /tmp/notes_to_docs_common.txt)
    DOCS_TO_VOCAB_COMMON_COUNT=$(wc -l < /tmp/docs_to_vocab_common.txt)
    VOCAB_TO_CODE_COMMON_COUNT=$(wc -l < /tmp/vocab_to_code_common.txt)
    
    if [ "$NOTE_TERMS_COUNT" -gt 0 ]; then
        NOTES_TO_DOCS_COVERAGE=$((NOTES_TO_DOCS_COMMON_COUNT * 100 / NOTE_TERMS_COUNT))
    else
        NOTES_TO_DOCS_COVERAGE=0
    fi
    
    if [ "$DOC_TERMS_COUNT" -gt 0 ]; then
        DOCS_TO_VOCAB_COVERAGE=$((DOCS_TO_VOCAB_COMMON_COUNT * 100 / DOC_TERMS_COUNT))
    else
        DOCS_TO_VOCAB_COVERAGE=0
    fi
    
    if [ "$VOCAB_TERMS_COUNT" -gt 0 ]; then
        VOCAB_TO_CODE_COVERAGE=$((VOCAB_TO_CODE_COMMON_COUNT * 100 / VOCAB_TERMS_COUNT))
    else
        VOCAB_TO_CODE_COVERAGE=0
    fi
    
    OVERALL_WORKFLOW_SCORE=$(( (NOTES_TO_DOCS_COVERAGE + DOCS_TO_VOCAB_COVERAGE + VOCAB_TO_CODE_COVERAGE) / 3 ))
    
    CODE_REFS_COUNT=$(grep -r "code_reference:" "$VOCAB_DIR" --include="*.md" | grep -v "TBD" | wc -l)
    TBD_REFS_COUNT=$(grep -r "code_reference:" "$VOCAB_DIR" --include="*.md" | grep "TBD" | wc -l)
    TOTAL_REFS=$((CODE_REFS_COUNT + TBD_REFS_COUNT))
    
    if [ "$TOTAL_REFS" -gt 0 ]; then
        CODE_REF_COVERAGE=$((CODE_REFS_COUNT * 100 / TOTAL_REFS))
    else
        CODE_REF_COVERAGE=0
    fi
    
    # Create the report
    cat > "$REPORT_FILE" << EOF
# Vocabulary Workflow Analysis Report

![Generated-$(date +%Y-%m-%d)](https://img.shields.io/badge/Generated-$(date +%Y-%m-%d)-blue)
![Overall Score-${OVERALL_WORKFLOW_SCORE}%](https://img.shields.io/badge/Overall%20Score-${OVERALL_WORKFLOW_SCORE}%25-${OVERALL_WORKFLOW_SCORE -ge 70 && echo "green" || echo "orange"})
![Notes to Docs-${NOTES_TO_DOCS_COVERAGE}%](https://img.shields.io/badge/Notes%20to%20Docs-${NOTES_TO_DOCS_COVERAGE}%25-${NOTES_TO_DOCS_COVERAGE -ge 70 && echo "green" || echo "orange"})
![Docs to Vocab-${DOCS_TO_VOCAB_COVERAGE}%](https://img.shields.io/badge/Docs%20to%20Vocab-${DOCS_TO_VOCAB_COVERAGE}%25-${DOCS_TO_VOCAB_COVERAGE -ge 70 && echo "green" || echo "orange"})
![Vocab to Code-${VOCAB_TO_CODE_COVERAGE}%](https://img.shields.io/badge/Vocab%20to%20Code-${VOCAB_TO_CODE_COVERAGE}%25-${VOCAB_TO_CODE_COVERAGE -ge 70 && echo "green" || echo "orange"})

This report analyzes how well the project follows the workflow defined in \`vocabulary.mdc\`:

\`\`\`
/notes are analyzed to create /docs
/docs are analyzed to create /vocabulary
/vocabulary is used to create /code
\`\`\`

## Executive Summary

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Overall Workflow Score | ${OVERALL_WORKFLOW_SCORE}% | 80% | ${OVERALL_WORKFLOW_SCORE -ge 80 && echo "✅" || echo "⚠️"} |
| Notes to Docs Flow | ${NOTES_TO_DOCS_COVERAGE}% | 75% | ${NOTES_TO_DOCS_COVERAGE -ge 75 && echo "✅" || echo "⚠️"} |
| Docs to Vocabulary Flow | ${DOCS_TO_VOCAB_COVERAGE}% | 80% | ${DOCS_TO_VOCAB_COVERAGE -ge 80 && echo "✅" || echo "⚠️"} |
| Vocabulary to Code Flow | ${VOCAB_TO_CODE_COVERAGE}% | 70% | ${VOCAB_TO_CODE_COVERAGE -ge 70 && echo "✅" || echo "⚠️"} |
| Code Reference Coverage | ${CODE_REF_COVERAGE}% | 90% | ${CODE_REF_COVERAGE -ge 90 && echo "✅" || echo "⚠️"} |

## Term Statistics

| Location | Unique Terms | Flow Coverage |
|----------|--------------|--------------|
| Notes | ${NOTE_TERMS_COUNT} | - |
| Docs | ${DOC_TERMS_COUNT} | ${NOTES_TO_DOCS_COVERAGE}% from Notes |
| Vocabulary | ${VOCAB_TERMS_COUNT} | ${DOCS_TO_VOCAB_COVERAGE}% from Docs |
| Code | ${CODE_TERMS_COUNT} | ${VOCAB_TO_CODE_COVERAGE}% from Vocabulary |

## Flow Visualization

\`\`\`
Notes [${NOTE_TERMS_COUNT}] --${NOTES_TO_DOCS_COVERAGE}%--> Docs [${DOC_TERMS_COUNT}] --${DOCS_TO_VOCAB_COVERAGE}%--> Vocabulary [${VOCAB_TERMS_COUNT}] --${VOCAB_TO_CODE_COVERAGE}%--> Code [${CODE_TERMS_COUNT}]
\`\`\`

## Workflow Metrics

### Notes to Docs Flow

- ${NOTES_TO_DOCS_COMMON_COUNT} terms from notes appear in docs (${NOTES_TO_DOCS_COVERAGE}% coverage)
- $(wc -l < /tmp/notes_not_in_docs.txt) terms from notes are missing in docs

### Docs to Vocabulary Flow

- ${DOCS_TO_VOCAB_COMMON_COUNT} terms from docs appear in vocabulary (${DOCS_TO_VOCAB_COVERAGE}% coverage)
- $(wc -l < /tmp/docs_not_in_vocab.txt) terms from docs are missing in vocabulary

### Vocabulary to Code Flow

- ${VOCAB_TO_CODE_COMMON_COUNT} terms from vocabulary appear in code (${VOCAB_TO_CODE_COVERAGE}% coverage)
- $(wc -l < /tmp/vocab_not_in_code.txt) terms from vocabulary are missing in code
- ${CODE_REFS_COUNT} of ${TOTAL_REFS} terms have code references (${CODE_REF_COVERAGE}% coverage)

## Domain Coverage

EOF

    # Add vocabulary domains to the report
    echo "### Vocabulary Domains" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "| Domain | Term Count |" >> "$REPORT_FILE"
    echo "|--------|------------|" >> "$REPORT_FILE"
    
    while IFS= read -r domain; do
        term_count=$(grep -r "domain: \"$domain\"" "$VOCAB_DIR" --include="*.md" | wc -l)
        echo "| $domain | $term_count |" >> "$REPORT_FILE"
    done < /tmp/vocab_domains.txt
    
    echo "" >> "$REPORT_FILE"

    # Add vocabulary term types to the report
    echo "### Term Types" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "| Type | Count | Percentage |" >> "$REPORT_FILE"
    echo "|------|-------|------------|" >> "$REPORT_FILE"
    
    while IFS= read -r type; do
        type_count=$(grep -r ":$type" "$VOCAB_DIR" --include="*.md" | wc -l)
        percentage=$((type_count * 100 / VOCAB_TERMS_COUNT))
        echo "| $type | $type_count | $percentage% |" >> "$REPORT_FILE"
    done < /tmp/vocab_types.txt
    
    echo "" >> "$REPORT_FILE"

    # If detailed output requested, add lists of missing terms
    if [ "$DETAILED" = true ]; then
        echo "## Missing Terms" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        
        echo "### Top Notes Terms Missing from Docs" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "| Term | Occurrences in Notes |" >> "$REPORT_FILE"
        echo "|------|---------------------|" >> "$REPORT_FILE"
        
        grep -f /tmp/notes_not_in_docs.txt /tmp/note_terms_count.txt | head -10 | while read -r count term; do
            echo "| $term | $count |" >> "$REPORT_FILE"
        done
        
        echo "" >> "$REPORT_FILE"
        
        echo "### Top Docs Terms Missing from Vocabulary" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "| Term | Occurrences in Docs |" >> "$REPORT_FILE"
        echo "|------|---------------------|" >> "$REPORT_FILE"
        
        grep -f /tmp/docs_not_in_vocab.txt /tmp/doc_terms_count.txt | head -10 | while read -r count term; do
            echo "| $term | $count |" >> "$REPORT_FILE"
        done
        
        echo "" >> "$REPORT_FILE"
        
        echo "### Top Vocabulary Terms Missing from Code" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "| Term | Code Reference Status |" >> "$REPORT_FILE"
        echo "|------|----------------------|" >> "$REPORT_FILE"
        
        grep -f /tmp/vocab_not_in_code.txt /tmp/vocab_terms.txt | head -10 | while read -r term; do
            code_ref=$(grep -r "term: \"$term\"" "$VOCAB_DIR" --include="*.md" -A 5 | grep "code_reference:" | head -1 | sed -E 's/.*code_reference: "(.*)",/\1/')
            echo "| $term | $code_ref |" >> "$REPORT_FILE"
        done
    fi
    
    # Add recommendations
    echo "## Recommendations" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    
    # Notes to Docs recommendations
    if [ "$NOTES_TO_DOCS_COVERAGE" -lt 75 ]; then
        echo "### Improve Notes to Docs Flow" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "1. Review important terms in notes that are missing from docs (see Missing Terms section)" >> "$REPORT_FILE"
        echo "2. Ensure key concepts from research notes are documented properly" >> "$REPORT_FILE"
        echo "3. Consider creating documentation templates that prompt for key terms from notes" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
    fi
    
    # Docs to Vocabulary recommendations
    if [ "$DOCS_TO_VOCAB_COVERAGE" -lt 80 ]; then
        echo "### Improve Docs to Vocabulary Flow" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "1. Extract key terms from documentation into the vocabulary" >> "$REPORT_FILE"
        echo "2. Review high-frequency terms in docs that are missing from vocabulary" >> "$REPORT_FILE"
        echo "3. Consider creating a script to suggest vocabulary terms from documentation" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
    fi
    
    # Vocabulary to Code recommendations
    if [ "$VOCAB_TO_CODE_COVERAGE" -lt 70 ]; then
        echo "### Improve Vocabulary to Code Flow" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
        echo "1. Ensure code uses the same terminology as defined in vocabulary" >> "$REPORT_FILE"
        echo "2. Complete code references for vocabulary terms (currently at ${CODE_REF_COVERAGE}%)" >> "$REPORT_FILE"
        echo "3. Consider creating code generators or templates that use vocabulary terms" >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
    fi
    
    # Final closing
    echo "---" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "*This report was automatically generated on $(date) by the workflow_analyzer.sh script.*" >> "$REPORT_FILE"
    
    echo -e "${GREEN}Report generated at: $REPORT_FILE${NC}"
}

# Run the analysis
extract_terms_from_notes
extract_terms_from_docs
extract_terms_from_vocabulary
extract_terms_from_code

analyze_notes_to_docs
analyze_docs_to_vocabulary
analyze_vocabulary_to_code
analyze_overall_workflow

generate_report

# Clean up temporary files
rm -f /tmp/note_terms.txt /tmp/doc_terms.txt /tmp/vocab_terms.txt /tmp/code_terms.txt
rm -f /tmp/note_terms_count.txt /tmp/doc_terms_count.txt /tmp/vocab_terms_count.txt /tmp/code_terms_count.txt
rm -f /tmp/notes_to_docs_common.txt /tmp/docs_to_vocab_common.txt /tmp/vocab_to_code_common.txt
rm -f /tmp/notes_not_in_docs.txt /tmp/docs_not_in_vocab.txt /tmp/vocab_not_in_code.txt
rm -f /tmp/vocab_domains.txt /tmp/vocab_types.txt

echo -e "${GREEN}Workflow analysis complete!${NC}"
echo -e "${BLUE}To see detailed results, check the report at: $REPORT_FILE${NC}"

exit 0 