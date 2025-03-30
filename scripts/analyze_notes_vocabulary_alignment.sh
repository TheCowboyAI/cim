#!/bin/bash
#
# analyze_notes_vocabulary_alignment.sh
#
# Script to analyze how well vocabulary terms reflect content in the /notes directory
# and align with the workflow defined in vocabulary.mdc:
#   /notes -> /docs -> /vocabulary -> /code
#
# Usage: ./scripts/analyze_notes_vocabulary_alignment.sh
#

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Directories and files
VOCABULARY_DIR="./vocabulary/domains"
NOTES_DIR="./notes"
DOCS_DIR="./docs"
INCOMING_DIR="./notes/incoming"
OUTPUT_FILE="docs/notes_vocabulary_alignment.md"
TEMP_DIR="/tmp/vocab_analysis"

# Create temp directory if it doesn't exist
mkdir -p $TEMP_DIR

echo -e "${BLUE}=== Notes-Vocabulary Alignment Analysis ===${NC}"
echo -e "${BLUE}Analyzing alignment between notes content and vocabulary terms...${NC}"

# Clean up any existing temp files
rm -f $TEMP_DIR/*.txt

# Check if directories exist
for dir in "$VOCABULARY_DIR" "$NOTES_DIR" "$DOCS_DIR"; do
    if [ ! -d "$dir" ]; then
        echo -e "${RED}Directory does not exist: $dir${NC}"
        exit 1
    fi
done

# Check for incoming directory and create if missing
if [ ! -d "$INCOMING_DIR" ]; then
    echo -e "${YELLOW}Creating incoming directory for documents ready for inspection...${NC}"
    mkdir -p "$INCOMING_DIR"
    echo -e "${GREEN}Created $INCOMING_DIR${NC}"
fi

# Step 1: Extract vocabulary terms, domains, and categories
echo -e "${BLUE}1. Extracting vocabulary terms...${NC}"

# Get all terms from vocabulary files
grep -r "term:" "$VOCABULARY_DIR" --include="*.md" | sed -E 's/.*term: "([^"]+)".*/\1/' > $TEMP_DIR/vocab_terms.txt
echo -e "${GREEN}  Extracted $(wc -l < $TEMP_DIR/vocab_terms.txt) vocabulary terms${NC}"

# Get domains used in vocabulary
grep -r "domain:" "$VOCABULARY_DIR" --include="*.md" | sed -E 's/.*domain: "([^"]+)".*/\1/' | sort | uniq > $TEMP_DIR/vocab_domains.txt
DOMAIN_COUNT=$(wc -l < $TEMP_DIR/vocab_domains.txt)
echo -e "${GREEN}  Extracted $DOMAIN_COUNT vocabulary domains${NC}"

# Extract term categories (from node type declarations)
grep -r ":" "$VOCABULARY_DIR" --include="*.md" | grep -oE ':[A-Za-z0-9_]+:[A-Za-z0-9_]+' | cut -d':' -f3 | sort | uniq > $TEMP_DIR/term_types.txt
echo -e "${GREEN}  Extracted $(wc -l < $TEMP_DIR/term_types.txt) term types/categories${NC}"

# Count term categories according to vocabulary.mdc
echo -e "${BLUE}2. Analyzing term categories...${NC}"

# Domain Objects
DOMAIN_OBJECTS_COUNT=$(grep -E "(Entity|Aggregate|ValueObject|Policy|Command|Query|Event|Service)" $TEMP_DIR/term_types.txt | wc -l)

# Technical Concepts
TECHNICAL_CONCEPTS_COUNT=$(grep -E "(Pattern|Architecture|Protocol|Algorithm)" $TEMP_DIR/term_types.txt | wc -l)  

# Business Concepts
BUSINESS_CONCEPTS_COUNT=$(grep -E "(Process|Rule|Workflow)" $TEMP_DIR/term_types.txt | wc -l)

# Cross-Cutting Terms
CROSS_CUTTING_COUNT=$(grep -E "(Security|Configuration|Monitoring)" $TEMP_DIR/term_types.txt | wc -l)

echo -e "${GREEN}  Domain Objects: $DOMAIN_OBJECTS_COUNT${NC}"
echo -e "${GREEN}  Technical Concepts: $TECHNICAL_CONCEPTS_COUNT${NC}"
echo -e "${GREEN}  Business Concepts: $BUSINESS_CONCEPTS_COUNT${NC}"
echo -e "${GREEN}  Cross-Cutting Terms: $CROSS_CUTTING_COUNT${NC}"

# Extract relationship types
echo -e "${BLUE}3. Analyzing relationship types...${NC}"

# Extract relationship types
grep -r "-\[:" "$VOCABULARY_DIR" --include="*.md" | grep -oE "\-\[:[A-Z_]+" | sed 's/-\[://g' | sort | uniq > $TEMP_DIR/relationship_types.txt
echo -e "${GREEN}  Extracted $(wc -l < $TEMP_DIR/relationship_types.txt) relationship types${NC}"

# Count relationship types according to vocabulary.mdc
# Hierarchical relationships
HIERARCHICAL_REL_COUNT=$(grep -E "(IS_A|PART_OF|CONTAINS|EXTENDS)" $TEMP_DIR/relationship_types.txt | wc -l)

# Functional relationships
FUNCTIONAL_REL_COUNT=$(grep -E "(MANAGES|PROCESSES|VALIDATES|CONFIGURES)" $TEMP_DIR/relationship_types.txt | wc -l)

# Temporal relationships
TEMPORAL_REL_COUNT=$(grep -E "(PRECEDES|FOLLOWS|TRIGGERS|DEPENDS_ON)" $TEMP_DIR/relationship_types.txt | wc -l)

# Other relationships
OTHER_REL_COUNT=$(grep -vE "(IS_A|PART_OF|CONTAINS|EXTENDS|MANAGES|PROCESSES|VALIDATES|CONFIGURES|PRECEDES|FOLLOWS|TRIGGERS|DEPENDS_ON)" $TEMP_DIR/relationship_types.txt | wc -l)

echo -e "${GREEN}  Hierarchical Relationships: $HIERARCHICAL_REL_COUNT${NC}"
echo -e "${GREEN}  Functional Relationships: $FUNCTIONAL_REL_COUNT${NC}"
echo -e "${GREEN}  Temporal Relationships: $TEMPORAL_REL_COUNT${NC}"
echo -e "${GREEN}  Other Relationships: $OTHER_REL_COUNT${NC}"

# Extract primary taxonomies
echo -e "${BLUE}4. Analyzing primary taxonomies...${NC}"

# Get all taxonomy names
grep -r "Taxonomy" "$VOCABULARY_DIR" --include="*.md" | grep "name:" | grep -oE "name: \"[^\"]+\"" | sed 's/name: "//g' | sed 's/"//g' | sort | uniq > $TEMP_DIR/taxonomies.txt
echo -e "${GREEN}  Extracted $(wc -l < $TEMP_DIR/taxonomies.txt) taxonomies${NC}"

# Check for primary taxonomies from vocabulary.mdc
STORAGE_TAXONOMY=$(grep -c "Storage Taxonomy" $TEMP_DIR/taxonomies.txt)
MEDIA_TAXONOMY=$(grep -c "Media Taxonomy" $TEMP_DIR/taxonomies.txt)
UI_TAXONOMY=$(grep -c "UI Taxonomy" $TEMP_DIR/taxonomies.txt)
CONFIG_TAXONOMY=$(grep -c "Configuration Taxonomy" $TEMP_DIR/taxonomies.txt)

echo -e "${GREEN}  Storage Taxonomy: $STORAGE_TAXONOMY${NC}"
echo -e "${GREEN}  Media Taxonomy: $MEDIA_TAXONOMY${NC}"
echo -e "${GREEN}  UI Taxonomy: $UI_TAXONOMY${NC}"
echo -e "${GREEN}  Configuration Taxonomy: $CONFIG_TAXONOMY${NC}"

# Step 2: Analyze Notes directory structure and content
echo -e "${BLUE}5. Analyzing notes directory structure...${NC}"

# Get list of note directories (potential domains)
find "$NOTES_DIR" -type d -not -path "$NOTES_DIR/.git*" | sort > $TEMP_DIR/notes_dirs.txt
NOTE_DIRS_COUNT=$(wc -l < $TEMP_DIR/notes_dirs.txt)
echo -e "${GREEN}  Found $NOTE_DIRS_COUNT directories in notes${NC}"

# Find potential terms from notes by extracting headings and emphasized text
echo -e "${BLUE}6. Extracting potential terms from notes...${NC}"

find "$NOTES_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
    # Extract headings (potential terms)
    grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.+)/\1/' >> $TEMP_DIR/notes_headings.txt
    
    # Extract emphasized text (potential terms)
    grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*[^*]+\*\*" | sed -E 's/\*\*(.+)\*\*/\1/' >> $TEMP_DIR/notes_emphasized.txt
done

# Combine and get unique terms
cat $TEMP_DIR/notes_headings.txt $TEMP_DIR/notes_emphasized.txt | sort | uniq > $TEMP_DIR/notes_terms.txt
NOTES_TERMS_COUNT=$(wc -l < $TEMP_DIR/notes_terms.txt)
echo -e "${GREEN}  Extracted $NOTES_TERMS_COUNT potential terms from notes${NC}"

# Step 3: Analyze Docs directory structure and content
echo -e "${BLUE}7. Analyzing docs directory structure...${NC}"

# Get list of docs directories
find "$DOCS_DIR" -type d -not -path "$DOCS_DIR/.git*" | sort > $TEMP_DIR/docs_dirs.txt
DOCS_DIRS_COUNT=$(wc -l < $TEMP_DIR/docs_dirs.txt)
echo -e "${GREEN}  Found $DOCS_DIRS_COUNT directories in docs${NC}"

# Find potential terms from docs
echo -e "${BLUE}8. Extracting potential terms from docs...${NC}"

find "$DOCS_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
    # Extract headings (potential terms)
    grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.+)/\1/' >> $TEMP_DIR/docs_headings.txt
    
    # Extract emphasized text (potential terms)
    grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*[^*]+\*\*" | sed -E 's/\*\*(.+)\*\*/\1/' >> $TEMP_DIR/docs_emphasized.txt
done

# Combine and get unique terms
cat $TEMP_DIR/docs_headings.txt $TEMP_DIR/docs_emphasized.txt | sort | uniq > $TEMP_DIR/docs_terms.txt
DOCS_TERMS_COUNT=$(wc -l < $TEMP_DIR/docs_terms.txt)
echo -e "${GREEN}  Extracted $DOCS_TERMS_COUNT potential terms from docs${NC}"

# Step 4: Calculate alignment metrics
echo -e "${BLUE}9. Calculating alignment metrics...${NC}"

# Term alignment between notes and vocabulary
comm -12 <(sort $TEMP_DIR/notes_terms.txt) <(sort $TEMP_DIR/vocab_terms.txt) > $TEMP_DIR/common_notes_vocab.txt
COMMON_NOTES_VOCAB=$(wc -l < $TEMP_DIR/common_notes_vocab.txt)
TERM_ALIGNMENT=$((COMMON_NOTES_VOCAB * 100 / NOTES_TERMS_COUNT))
echo -e "${GREEN}  Term Alignment: $TERM_ALIGNMENT% ($COMMON_NOTES_VOCAB/$NOTES_TERMS_COUNT terms)${NC}"

# Workflow alignment: Terms appearing in notes -> docs -> vocabulary
# First, find common terms between notes and docs
comm -12 <(sort $TEMP_DIR/notes_terms.txt) <(sort $TEMP_DIR/docs_terms.txt) > $TEMP_DIR/common_notes_docs.txt
COMMON_NOTES_DOCS=$(wc -l < $TEMP_DIR/common_notes_docs.txt)
NOTES_TO_DOCS_FLOW=$((COMMON_NOTES_DOCS * 100 / NOTES_TERMS_COUNT))
echo -e "${GREEN}  Notes->Docs Flow: $NOTES_TO_DOCS_FLOW% ($COMMON_NOTES_DOCS/$NOTES_TERMS_COUNT terms)${NC}"

# Then, find common terms between docs and vocabulary
comm -12 <(sort $TEMP_DIR/docs_terms.txt) <(sort $TEMP_DIR/vocab_terms.txt) > $TEMP_DIR/common_docs_vocab.txt
COMMON_DOCS_VOCAB=$(wc -l < $TEMP_DIR/common_docs_vocab.txt)
DOCS_TO_VOCAB_FLOW=$((COMMON_DOCS_VOCAB * 100 / DOCS_TERMS_COUNT))
echo -e "${GREEN}  Docs->Vocabulary Flow: $DOCS_TO_VOCAB_FLOW% ($COMMON_DOCS_VOCAB/$DOCS_TERMS_COUNT terms)${NC}"

# Domain structure alignment
comm -12 <(sort $TEMP_DIR/notes_dirs.txt | xargs -n1 basename) <(sort $TEMP_DIR/vocab_domains.txt) > $TEMP_DIR/common_domains.txt
COMMON_DOMAINS=$(wc -l < $TEMP_DIR/common_domains.txt)
DOMAIN_ALIGNMENT=$((COMMON_DOMAINS * 100 / DOMAIN_COUNT))
echo -e "${GREEN}  Domain Structure Alignment: $DOMAIN_ALIGNMENT% ($COMMON_DOMAINS/$DOMAIN_COUNT domains)${NC}"

# Missing domains (in vocabulary but not reflected in notes)
comm -23 <(sort $TEMP_DIR/vocab_domains.txt) <(sort $TEMP_DIR/notes_dirs.txt | xargs -n1 basename) > $TEMP_DIR/missing_domains.txt
MISSING_DOMAINS=$(wc -l < $TEMP_DIR/missing_domains.txt)
echo -e "${YELLOW}  Missing Domains: $MISSING_DOMAINS${NC}"

# Calculate category distribution score (how balanced are the term categories)
TOTAL_CATEGORIZED=$((DOMAIN_OBJECTS_COUNT + TECHNICAL_CONCEPTS_COUNT + BUSINESS_CONCEPTS_COUNT + CROSS_CUTTING_COUNT))
if [ $TOTAL_CATEGORIZED -gt 0 ]; then
    DOMAIN_OBJECTS_PERCENT=$((DOMAIN_OBJECTS_COUNT * 100 / TOTAL_CATEGORIZED))
    TECHNICAL_CONCEPTS_PERCENT=$((TECHNICAL_CONCEPTS_COUNT * 100 / TOTAL_CATEGORIZED))
    BUSINESS_CONCEPTS_PERCENT=$((BUSINESS_CONCEPTS_COUNT * 100 / TOTAL_CATEGORIZED))
    CROSS_CUTTING_PERCENT=$((CROSS_CUTTING_COUNT * 100 / TOTAL_CATEGORIZED))
    
    # Calculate deviation from ideal balance (25% for each category)
    DEVIATION=$(( (${DOMAIN_OBJECTS_PERCENT}-25)**2 + (${TECHNICAL_CONCEPTS_PERCENT}-25)**2 + (${BUSINESS_CONCEPTS_PERCENT}-25)**2 + (${CROSS_CUTTING_PERCENT}-25)**2 ))
    DEVIATION_SQRT=$(echo "sqrt($DEVIATION)" | bc)
    
    # Convert to a score out of 100 (higher is better)
    CATEGORY_BALANCE=$((100 - DEVIATION_SQRT))
    if [ $CATEGORY_BALANCE -lt 0 ]; then
        CATEGORY_BALANCE=0
    fi
    
    echo -e "${GREEN}  Category Balance Score: $CATEGORY_BALANCE%${NC}"
else
    CATEGORY_BALANCE=0
    echo -e "${YELLOW}  No categorized terms found${NC}"
fi

# Calculate relationship type balance
TOTAL_RELATIONSHIPS=$((HIERARCHICAL_REL_COUNT + FUNCTIONAL_REL_COUNT + TEMPORAL_REL_COUNT + OTHER_REL_COUNT))
if [ $TOTAL_RELATIONSHIPS -gt 0 ]; then
    HIERARCHICAL_PERCENT=$((HIERARCHICAL_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
    FUNCTIONAL_PERCENT=$((FUNCTIONAL_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
    TEMPORAL_PERCENT=$((TEMPORAL_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
    OTHER_PERCENT=$((OTHER_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
    
    # Calculate deviation from ideal balance (33% for hierarchical, functional, and temporal, 0% for other)
    DEVIATION=$(( (${HIERARCHICAL_PERCENT}-33)**2 + (${FUNCTIONAL_PERCENT}-33)**2 + (${TEMPORAL_PERCENT}-33)**2 + ${OTHER_PERCENT}**2 ))
    DEVIATION_SQRT=$(echo "sqrt($DEVIATION)" | bc)
    
    # Convert to a score out of 100 (higher is better)
    RELATIONSHIP_BALANCE=$((100 - DEVIATION_SQRT))
    if [ $RELATIONSHIP_BALANCE -lt 0 ]; then
        RELATIONSHIP_BALANCE=0
    fi
    
    echo -e "${GREEN}  Relationship Type Balance: $RELATIONSHIP_BALANCE%${NC}"
else
    RELATIONSHIP_BALANCE=0
    echo -e "${YELLOW}  No relationships found${NC}"
fi

# Calculate primary taxonomy score
TAXONOMY_COUNT=$((STORAGE_TAXONOMY + MEDIA_TAXONOMY + UI_TAXONOMY + CONFIG_TAXONOMY))
TAXONOMY_SCORE=$((TAXONOMY_COUNT * 100 / 4))
echo -e "${GREEN}  Taxonomy Score: $TAXONOMY_SCORE%${NC}"

# Calculate overall alignment score
OVERALL_ALIGNMENT=$(( (TERM_ALIGNMENT + DOMAIN_ALIGNMENT + NOTES_TO_DOCS_FLOW + DOCS_TO_VOCAB_FLOW + CATEGORY_BALANCE + RELATIONSHIP_BALANCE + TAXONOMY_SCORE) / 7 ))
echo -e "${BLUE}Overall Notes-Vocabulary Alignment: $OVERALL_ALIGNMENT%${NC}"

# Step 5: Generate report
echo -e "${BLUE}10. Generating report...${NC}"

cat > "$OUTPUT_FILE" << EOF
# Notes-Vocabulary Alignment Analysis

![Generated-$(date +%Y-%m-%d)](https://img.shields.io/badge/Generated-$(date +%Y-%m-%d)-blue)
![Overall Alignment-${OVERALL_ALIGNMENT}%](https://img.shields.io/badge/Overall%20Alignment-${OVERALL_ALIGNMENT}%25-${OVERALL_ALIGNMENT -ge 70 && echo "green" || echo "orange"})
![Term Alignment-${TERM_ALIGNMENT}%](https://img.shields.io/badge/Term%20Alignment-${TERM_ALIGNMENT}%25-${TERM_ALIGNMENT -ge 70 && echo "green" || echo "orange"})
![Workflow Alignment-${NOTES_TO_DOCS_FLOW}%](https://img.shields.io/badge/Workflow%20Alignment-${NOTES_TO_DOCS_FLOW}%25-${NOTES_TO_DOCS_FLOW -ge 75 && echo "green" || echo "orange"})

This report analyzes how well the vocabulary terms reflect content in the notes directory and align with the workflow defined in \`vocabulary.mdc\`.

## Executive Summary

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Overall Alignment | ${OVERALL_ALIGNMENT}% | 90% | ${OVERALL_ALIGNMENT -ge 90 && echo "✅" || echo "⚠️"} |
| Term Alignment | ${TERM_ALIGNMENT}% | 85% | ${TERM_ALIGNMENT -ge 85 && echo "✅" || echo "⚠️"} |
| Structure Alignment | ${DOMAIN_ALIGNMENT}% | 95% | ${DOMAIN_ALIGNMENT -ge 95 && echo "✅" || echo "⚠️"} |
| Missing Domains | ${MISSING_DOMAINS} | 0 | ${MISSING_DOMAINS -eq 0 && echo "✅" || echo "⚠️"} |
| Category Balance | ${CATEGORY_BALANCE}% | 80% | ${CATEGORY_BALANCE -ge 80 && echo "✅" || echo "⚠️"} |
| Relationship Type Balance | ${RELATIONSHIP_BALANCE}% | 75% | ${RELATIONSHIP_BALANCE -ge 75 && echo "✅" || echo "⚠️"} |
| Taxonomy Coverage | ${TAXONOMY_SCORE}% | 90% | ${TAXONOMY_SCORE -ge 90 && echo "✅" || echo "⚠️"} |

## Workflow Analysis

| Flow Stage | Coverage | Target | Status |
|------------|----------|--------|--------|
| Notes -> Docs | ${NOTES_TO_DOCS_FLOW}% | 75% | ${NOTES_TO_DOCS_FLOW -ge 75 && echo "✅" || echo "⚠️"} |
| Docs -> Vocabulary | ${DOCS_TO_VOCAB_FLOW}% | 80% | ${DOCS_TO_VOCAB_FLOW -ge 80 && echo "✅" || echo "⚠️"} |

## Term Statistics

- **Notes Terms**: $NOTES_TERMS_COUNT potential terms identified
- **Docs Terms**: $DOCS_TERMS_COUNT potential terms identified
- **Vocabulary Terms**: $(wc -l < $TEMP_DIR/vocab_terms.txt) defined terms
- **Common Notes-Vocabulary**: $COMMON_NOTES_VOCAB terms
- **Common Notes-Docs**: $COMMON_NOTES_DOCS terms
- **Common Docs-Vocabulary**: $COMMON_DOCS_VOCAB terms

## Term Categories (vocabulary.mdc)

| Category | Count | Percentage | Target | Status |
|----------|-------|------------|--------|--------|
| Domain Objects | $DOMAIN_OBJECTS_COUNT | ${DOMAIN_OBJECTS_PERCENT}% | 35-45% | ${DOMAIN_OBJECTS_PERCENT -ge 35 && DOMAIN_OBJECTS_PERCENT -le 45 && echo "✅" || echo "⚠️"} |
| Technical Concepts | $TECHNICAL_CONCEPTS_COUNT | ${TECHNICAL_CONCEPTS_PERCENT}% | 20-30% | ${TECHNICAL_CONCEPTS_PERCENT -ge 20 && TECHNICAL_CONCEPTS_PERCENT -le 30 && echo "✅" || echo "⚠️"} |
| Business Concepts | $BUSINESS_CONCEPTS_COUNT | ${BUSINESS_CONCEPTS_PERCENT}% | 20-30% | ${BUSINESS_CONCEPTS_PERCENT -ge 20 && BUSINESS_CONCEPTS_PERCENT -le 30 && echo "✅" || echo "⚠️"} |
| Cross-Cutting Terms | $CROSS_CUTTING_COUNT | ${CROSS_CUTTING_PERCENT}% | 5-15% | ${CROSS_CUTTING_PERCENT -ge 5 && CROSS_CUTTING_PERCENT -le 15 && echo "✅" || echo "⚠️"} |

## Relationship Types (vocabulary.mdc)

| Type | Count | Percentage | Target | Status |
|------|-------|------------|--------|--------|
| Hierarchical | $HIERARCHICAL_REL_COUNT | ${HIERARCHICAL_PERCENT}% | 30-40% | ${HIERARCHICAL_PERCENT -ge 30 && HIERARCHICAL_PERCENT -le 40 && echo "✅" || echo "⚠️"} |
| Functional | $FUNCTIONAL_REL_COUNT | ${FUNCTIONAL_PERCENT}% | 30-40% | ${FUNCTIONAL_PERCENT -ge 30 && FUNCTIONAL_PERCENT -le 40 && echo "✅" || echo "⚠️"} |
| Temporal | $TEMPORAL_REL_COUNT | ${TEMPORAL_PERCENT}% | 20-30% | ${TEMPORAL_PERCENT -ge 20 && TEMPORAL_PERCENT -le 30 && echo "✅" || echo "⚠️"} |
| Other | $OTHER_REL_COUNT | ${OTHER_PERCENT}% | 0-10% | ${OTHER_PERCENT -ge 0 && OTHER_PERCENT -le 10 && echo "✅" || echo "⚠️"} |

## Primary Taxonomies (vocabulary.mdc)

| Taxonomy | Present | Status |
|----------|---------|--------|
| Storage Taxonomy | ${STORAGE_TAXONOMY -gt 0 && echo "Yes" || echo "No"} | ${STORAGE_TAXONOMY -gt 0 && echo "✅" || echo "⚠️"} |
| Media Taxonomy | ${MEDIA_TAXONOMY -gt 0 && echo "Yes" || echo "No"} | ${MEDIA_TAXONOMY -gt 0 && echo "✅" || echo "⚠️"} |
| UI Taxonomy | ${UI_TAXONOMY -gt 0 && echo "Yes" || echo "No"} | ${UI_TAXONOMY -gt 0 && echo "✅" || echo "⚠️"} |
| Configuration Taxonomy | ${CONFIG_TAXONOMY -gt 0 && echo "Yes" || echo "No"} | ${CONFIG_TAXONOMY -gt 0 && echo "✅" || echo "⚠️"} |

## Domain Coverage

| Domain | In Vocabulary | In Notes | Status |
|--------|---------------|---------|--------|
EOF

# Add domain coverage details to the report
while IFS= read -r domain; do
    in_notes=$(grep -c "$domain" $TEMP_DIR/notes_dirs.txt)
    if [ $in_notes -gt 0 ]; then
        status="✅"
    else
        status="⚠️"
    fi
    echo "| $domain | Yes | ${in_notes -gt 0 && echo "Yes" || echo "No"} | $status |" >> "$OUTPUT_FILE"
done < $TEMP_DIR/vocab_domains.txt

# Add missing domains section
cat >> "$OUTPUT_FILE" << EOF

## Missing Domains

These domains are defined in vocabulary but not reflected in the notes directory structure:

EOF

if [ $MISSING_DOMAINS -eq 0 ]; then
    echo "*No missing domains found*" >> "$OUTPUT_FILE"
else
    while IFS= read -r domain; do
        echo "- $domain" >> "$OUTPUT_FILE"
    done < $TEMP_DIR/missing_domains.txt
fi

# Add workflow recommendations
cat >> "$OUTPUT_FILE" << EOF

## Recommendations

### Workflow Improvements

1. **Notes -> Docs Flow (${NOTES_TO_DOCS_FLOW}%)**
   ${NOTES_TO_DOCS_FLOW -ge 75 && echo "✅ Good coverage of notes terms in documentation." || echo "⚠️ Improve documentation to better cover terms from notes."}
   ${NOTES_TO_DOCS_FLOW -lt 75 && echo "- Consider creating documentation for key concepts found in notes but missing in docs." || echo ""}

2. **Docs -> Vocabulary Flow (${DOCS_TO_VOCAB_FLOW}%)**
   ${DOCS_TO_VOCAB_FLOW -ge 80 && echo "✅ Good coverage of documented terms in vocabulary." || echo "⚠️ Improve vocabulary to better cover terms from documentation."}
   ${DOCS_TO_VOCAB_FLOW -lt 80 && echo "- Extract important terms from documentation into the vocabulary." || echo ""}

### Term Category Improvements

1. **Domain Objects (${DOMAIN_OBJECTS_PERCENT}%)**
   ${DOMAIN_OBJECTS_PERCENT -lt 35 && echo "⚠️ Add more Domain Objects to the vocabulary." || echo ""}
   ${DOMAIN_OBJECTS_PERCENT -gt 45 && echo "⚠️ Consider reducing the proportion of Domain Objects." || echo ""}
   ${DOMAIN_OBJECTS_PERCENT -ge 35 && DOMAIN_OBJECTS_PERCENT -le 45 && echo "✅ Good balance of Domain Objects." || echo ""}

2. **Technical Concepts (${TECHNICAL_CONCEPTS_PERCENT}%)**
   ${TECHNICAL_CONCEPTS_PERCENT -lt 20 && echo "⚠️ Add more Technical Concepts to the vocabulary." || echo ""}
   ${TECHNICAL_CONCEPTS_PERCENT -gt 30 && echo "⚠️ Consider reducing the proportion of Technical Concepts." || echo ""}
   ${TECHNICAL_CONCEPTS_PERCENT -ge 20 && TECHNICAL_CONCEPTS_PERCENT -le 30 && echo "✅ Good balance of Technical Concepts." || echo ""}

3. **Relationship Types**
   ${RELATIONSHIP_BALANCE -lt 75 && echo "⚠️ Improve balance between relationship types." || echo "✅ Good balance of relationship types."}
   ${FUNCTIONAL_PERCENT -lt 30 && echo "- Add more Functional relationships (MANAGES, PROCESSES, VALIDATES, CONFIGURES)." || echo ""}
   ${TEMPORAL_PERCENT -lt 20 && echo "- Add more Temporal relationships (PRECEDES, FOLLOWS, TRIGGERS, DEPENDS_ON)." || echo ""}

### Taxonomy Improvements

${TAXONOMY_SCORE -lt 90 && echo "⚠️ Add missing primary taxonomies:" || echo "✅ All primary taxonomies are present."}
${STORAGE_TAXONOMY -eq 0 && echo "- Add Storage Taxonomy" || echo ""}
${MEDIA_TAXONOMY -eq 0 && echo "- Add Media Taxonomy" || echo ""}
${UI_TAXONOMY -eq 0 && echo "- Add UI Taxonomy" || echo ""}
${CONFIG_TAXONOMY -eq 0 && echo "- Add Configuration Taxonomy" || echo ""}

## Next Steps to Improve Alignment

1. ${OVERALL_ALIGNMENT -lt 70 && echo "**Focus on workflow alignment**" || echo "**Maintain workflow alignment**"}: Ensure information flows properly from notes to docs to vocabulary.
2. ${TERM_ALIGNMENT -lt 85 && echo "**Extract key terms from notes**" || echo "**Maintain term coverage**"}: Identify important concepts in notes and ensure they're documented and defined in vocabulary.
3. ${DOMAIN_ALIGNMENT -lt 95 && echo "**Align domain structure**" || echo "**Maintain domain structure**"}: Ensure notes directory structure reflects the domains defined in vocabulary.
4. ${CATEGORY_BALANCE -lt 80 && echo "**Balance term categories**" || echo "**Maintain term category balance**"}: Aim for a balanced distribution across Domain Objects, Technical Concepts, Business Concepts, and Cross-Cutting Terms.
5. ${RELATIONSHIP_BALANCE -lt 75 && echo "**Improve relationship type balance**" || echo "**Maintain relationship type balance**"}: Ensure a good mix of Hierarchical, Functional, and Temporal relationships.

---

*This analysis was generated on $(date) using the **analyze_notes_vocabulary_alignment.sh** script.*
EOF

echo -e "${GREEN}Analysis complete!${NC}"
echo -e "${GREEN}Report generated at: $OUTPUT_FILE${NC}"

# Cleanup temp files
rm -rf $TEMP_DIR

# Summary
echo -e "${BLUE}=== Summary ===${NC}"
echo -e "${BLUE}Overall Alignment: $OVERALL_ALIGNMENT%${NC}"
echo -e "${BLUE}Term Alignment: $TERM_ALIGNMENT%${NC}"
echo -e "${BLUE}Structure Alignment: $DOMAIN_ALIGNMENT%${NC}"
echo -e "${BLUE}Category Balance: $CATEGORY_BALANCE%${NC}"
echo -e "${BLUE}Relationship Type Balance: $RELATIONSHIP_BALANCE%${NC}"
echo -e "${YELLOW}Missing Domains: $MISSING_DOMAINS${NC}"

# New section to analyze incoming documents
echo -e "${BLUE}10. Analyzing documents in incoming directory...${NC}"

# Count files in incoming directory
INCOMING_FILES=$(find "$INCOMING_DIR" -type f -name "*.md" | wc -l)

if [ "$INCOMING_FILES" -eq 0 ]; then
    echo -e "${YELLOW}  No documents found in the incoming directory.${NC}"
else
    echo -e "${GREEN}  Found $INCOMING_FILES documents ready for inspection.${NC}"
    
    # Extract terms from incoming documents
    find "$INCOMING_DIR" -type f -name "*.md" -print0 | while IFS= read -r -d '' file; do
        # Extract headings (potential terms)
        grep -E "^#+\s+.*" "$file" | sed -E 's/^#+\s+(.+)/\1/' >> $TEMP_DIR/incoming_headings.txt
        
        # Extract emphasized text (potential terms)
        grep -E "\*\*.*\*\*" "$file" | grep -oE "\*\*[^*]+\*\*" | sed -E 's/\*\*(.+)\*\*/\1/' >> $TEMP_DIR/incoming_emphasized.txt
    done
    
    # Combine and get unique terms
    if [ -f "$TEMP_DIR/incoming_headings.txt" ] && [ -f "$TEMP_DIR/incoming_emphasized.txt" ]; then
        cat $TEMP_DIR/incoming_headings.txt $TEMP_DIR/incoming_emphasized.txt | sort | uniq > $TEMP_DIR/incoming_terms.txt
        INCOMING_TERMS_COUNT=$(wc -l < $TEMP_DIR/incoming_terms.txt)
        echo -e "${GREEN}  Extracted $INCOMING_TERMS_COUNT potential terms from incoming documents${NC}"
        
        # Check for new terms not in vocabulary yet
        comm -23 <(sort $TEMP_DIR/incoming_terms.txt) <(sort $TEMP_DIR/vocab_terms.txt) > $TEMP_DIR/new_incoming_terms.txt
        NEW_TERMS_COUNT=$(wc -l < $TEMP_DIR/new_incoming_terms.txt)
        echo -e "${YELLOW}  Found $NEW_TERMS_COUNT new terms in incoming documents not yet in vocabulary${NC}"
    else
        echo -e "${YELLOW}  No terms extracted from incoming documents.${NC}"
    fi
fi

exit 0 