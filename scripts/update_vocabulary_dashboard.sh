#!/bin/bash
#
# update_vocabulary_dashboard.sh
#
# Script to automatically update the vocabulary quality dashboard
# with the latest metrics and data.
#
# Usage: ./scripts/update_vocabulary_dashboard.sh
#

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Dashboard file
DASHBOARD_FILE="docs/vocabulary_quality_dashboard.md"

# Check if dashboard file exists
if [ ! -f "$DASHBOARD_FILE" ]; then
    echo -e "${RED}Error: Dashboard file not found at $DASHBOARD_FILE${NC}"
    exit 1
fi

echo -e "${BLUE}=== Updating Vocabulary Quality Dashboard ===${NC}"
echo -e "${BLUE}Backing up current dashboard...${NC}"

# Create backup
cp "$DASHBOARD_FILE" "${DASHBOARD_FILE}.bak"
echo -e "${GREEN}Backup created at ${DASHBOARD_FILE}.bak${NC}"

# Run standardize vocabulary script to get latest compliance metrics
echo -e "${BLUE}Running vocabulary standardization check...${NC}"
STANDARDIZE_OUTPUT=$(./scripts/standardize_vocabulary_enhanced.sh vocabulary/domains)

# Extract compliance metrics
TOTAL_FILES=$(echo "$STANDARDIZE_OUTPUT" | grep "checked" | grep -oE "[0-9]+ files? checked" | grep -oE "[0-9]+")
COMPLIANT_FILES=$(echo "$STANDARDIZE_OUTPUT" | grep "compliant" | grep -oE "[0-9]+ compliant files?" | grep -oE "[0-9]+")
NON_COMPLIANT_FILES=$(echo "$STANDARDIZE_OUTPUT" | grep "non-compliant" | grep -oE "[0-9]+ non-compliant files?" | grep -oE "[0-9]+")

# Calculate compliance percentage
if [ -z "$TOTAL_FILES" ] || [ "$TOTAL_FILES" -eq 0 ]; then
    COMPLIANCE_PERCENT=0
else
    COMPLIANCE_PERCENT=$((COMPLIANT_FILES * 100 / TOTAL_FILES))
fi

echo -e "${GREEN}Compliance: $COMPLIANCE_PERCENT% ($COMPLIANT_FILES/$TOTAL_FILES files)${NC}"

# Extract term counts (vocabulary domains)
echo -e "${BLUE}Extracting term and relationship counts...${NC}"
TERM_COUNT=$(grep -r "term:" vocabulary/domains --include="*.md" | wc -l)
RELATIONSHIP_COUNT=$(grep -r "-\[:" vocabulary/domains --include="*.md" | wc -l)
echo -e "${GREEN}Terms: $TERM_COUNT, Relationships: $RELATIONSHIP_COUNT${NC}"

# Extract coverage metrics
# Extract code reference metrics
CODE_REFS=$(grep -r "codeReference:" vocabulary/domains --include="*.md" | wc -l)
CODE_REF_PERCENT=$((CODE_REFS * 100 / TERM_COUNT))
echo -e "${GREEN}Code references: $CODE_REFS ($CODE_REF_PERCENT%)${NC}"

# Calculate category metrics
echo -e "${BLUE}Extracting term category metrics...${NC}"
# Create temp directory
TEMP_DIR="/tmp/vocab_dashboard"
mkdir -p $TEMP_DIR

# Extract term categories
grep -r ":" vocabulary/domains --include="*.md" | grep -oE ':[A-Za-z0-9_]+:[A-Za-z0-9_]+' | cut -d':' -f3 | sort | uniq -c > $TEMP_DIR/term_categories.txt

# Count term categories
DOMAIN_OBJECTS_COUNT=$(grep -E "(Entity|Aggregate|ValueObject|Policy|Command|Query|Event|Service)" $TEMP_DIR/term_categories.txt | awk '{s+=$1} END {print s}')
TECHNICAL_CONCEPTS_COUNT=$(grep -E "(Pattern|Architecture|Protocol|Algorithm)" $TEMP_DIR/term_categories.txt | awk '{s+=$1} END {print s}')
BUSINESS_CONCEPTS_COUNT=$(grep -E "(Process|Rule|Workflow)" $TEMP_DIR/term_categories.txt | awk '{s+=$1} END {print s}')
CROSS_CUTTING_COUNT=$(grep -E "(Security|Configuration|Monitoring)" $TEMP_DIR/term_categories.txt | awk '{s+=$1} END {print s}')

# Default to 0 if grep returns nothing
DOMAIN_OBJECTS_COUNT=${DOMAIN_OBJECTS_COUNT:-0}
TECHNICAL_CONCEPTS_COUNT=${TECHNICAL_CONCEPTS_COUNT:-0}
BUSINESS_CONCEPTS_COUNT=${BUSINESS_CONCEPTS_COUNT:-0}
CROSS_CUTTING_COUNT=${CROSS_CUTTING_COUNT:-0}

# Calculate percentages
TOTAL_CATEGORIZED=$((DOMAIN_OBJECTS_COUNT + TECHNICAL_CONCEPTS_COUNT + BUSINESS_CONCEPTS_COUNT + CROSS_CUTTING_COUNT))
if [ $TOTAL_CATEGORIZED -gt 0 ]; then
    DOMAIN_OBJECTS_PERCENT=$((DOMAIN_OBJECTS_COUNT * 100 / TOTAL_CATEGORIZED))
    TECHNICAL_CONCEPTS_PERCENT=$((TECHNICAL_CONCEPTS_COUNT * 100 / TOTAL_CATEGORIZED))
    BUSINESS_CONCEPTS_PERCENT=$((BUSINESS_CONCEPTS_COUNT * 100 / TOTAL_CATEGORIZED))
    CROSS_CUTTING_PERCENT=$((CROSS_CUTTING_COUNT * 100 / TOTAL_CATEGORIZED))
else
    DOMAIN_OBJECTS_PERCENT=0
    TECHNICAL_CONCEPTS_PERCENT=0
    BUSINESS_CONCEPTS_PERCENT=0
    CROSS_CUTTING_PERCENT=0
fi

echo -e "${GREEN}Term Categories:${NC}"
echo -e "${GREEN}  Domain Objects: $DOMAIN_OBJECTS_COUNT ($DOMAIN_OBJECTS_PERCENT%)${NC}"
echo -e "${GREEN}  Technical Concepts: $TECHNICAL_CONCEPTS_COUNT ($TECHNICAL_CONCEPTS_PERCENT%)${NC}"
echo -e "${GREEN}  Business Concepts: $BUSINESS_CONCEPTS_COUNT ($BUSINESS_CONCEPTS_PERCENT%)${NC}"
echo -e "${GREEN}  Cross-Cutting: $CROSS_CUTTING_COUNT ($CROSS_CUTTING_PERCENT%)${NC}"

# Extract relationship metrics
echo -e "${BLUE}Extracting relationship type metrics...${NC}"
# Extract relationship types
grep -r "-\[:" vocabulary/domains --include="*.md" | grep -oE "\-\[:[A-Z_]+" | sed 's/-\[://g' | sort | uniq -c > $TEMP_DIR/relationship_types.txt

# Count relationship types
HIERARCHICAL_REL_COUNT=$(grep -E "(IS_A|PART_OF|CONTAINS|EXTENDS)" $TEMP_DIR/relationship_types.txt | awk '{s+=$1} END {print s}')
FUNCTIONAL_REL_COUNT=$(grep -E "(MANAGES|PROCESSES|VALIDATES|CONFIGURES)" $TEMP_DIR/relationship_types.txt | awk '{s+=$1} END {print s}')
TEMPORAL_REL_COUNT=$(grep -E "(PRECEDES|FOLLOWS|TRIGGERS|DEPENDS_ON)" $TEMP_DIR/relationship_types.txt | awk '{s+=$1} END {print s}')
OTHER_REL_COUNT=$(grep -vE "(IS_A|PART_OF|CONTAINS|EXTENDS|MANAGES|PROCESSES|VALIDATES|CONFIGURES|PRECEDES|FOLLOWS|TRIGGERS|DEPENDS_ON)" $TEMP_DIR/relationship_types.txt | awk '{s+=$1} END {print s}')

# Default to 0 if grep returns nothing
HIERARCHICAL_REL_COUNT=${HIERARCHICAL_REL_COUNT:-0}
FUNCTIONAL_REL_COUNT=${FUNCTIONAL_REL_COUNT:-0}
TEMPORAL_REL_COUNT=${TEMPORAL_REL_COUNT:-0}
OTHER_REL_COUNT=${OTHER_REL_COUNT:-0}

# Calculate percentages
TOTAL_RELATIONSHIPS=$((HIERARCHICAL_REL_COUNT + FUNCTIONAL_REL_COUNT + TEMPORAL_REL_COUNT + OTHER_REL_COUNT))
if [ $TOTAL_RELATIONSHIPS -gt 0 ]; then
    HIERARCHICAL_PERCENT=$((HIERARCHICAL_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
    FUNCTIONAL_PERCENT=$((FUNCTIONAL_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
    TEMPORAL_PERCENT=$((TEMPORAL_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
    OTHER_PERCENT=$((OTHER_REL_COUNT * 100 / TOTAL_RELATIONSHIPS))
else
    HIERARCHICAL_PERCENT=0
    FUNCTIONAL_PERCENT=0
    TEMPORAL_PERCENT=0
    OTHER_PERCENT=0
fi

echo -e "${GREEN}Relationship Types:${NC}"
echo -e "${GREEN}  Hierarchical: $HIERARCHICAL_REL_COUNT ($HIERARCHICAL_PERCENT%)${NC}"
echo -e "${GREEN}  Functional: $FUNCTIONAL_REL_COUNT ($FUNCTIONAL_PERCENT%)${NC}"
echo -e "${GREEN}  Temporal: $TEMPORAL_REL_COUNT ($TEMPORAL_PERCENT%)${NC}"
echo -e "${GREEN}  Other: $OTHER_REL_COUNT ($OTHER_PERCENT%)${NC}"

# Run the workflow analyzer to get workflow metrics
echo -e "${BLUE}Running workflow analysis...${NC}"
if [ -f "./scripts/workflow_analyzer.sh" ]; then
    ./scripts/workflow_analyzer.sh
    WORKFLOW_REPORT_EXISTS=true
else
    echo -e "${YELLOW}Workflow analyzer script not found. Skipping workflow analysis.${NC}"
    WORKFLOW_REPORT_EXISTS=false
fi

# Run notes alignment analysis for additional metrics
echo -e "${BLUE}Running notes alignment analysis...${NC}"
if [ -f "./scripts/analyze_notes_vocabulary_alignment.sh" ]; then
    ./scripts/analyze_notes_vocabulary_alignment.sh
    NOTES_ALIGNMENT_EXISTS=true
else
    echo -e "${YELLOW}Notes alignment script not found. Skipping notes alignment analysis.${NC}"
    NOTES_ALIGNMENT_EXISTS=false
fi

# Extract notes alignment metrics if they exist
if [ "$NOTES_ALIGNMENT_EXISTS" = true ] && [ -f "docs/notes_vocabulary_alignment.md" ]; then
    echo -e "${BLUE}Extracting notes alignment metrics...${NC}"
    OVERALL_ALIGNMENT=$(grep "Overall Alignment" "docs/notes_vocabulary_alignment.md" | grep -oE "[0-9]+%" | head -1 | tr -d '%')
    TERM_ALIGNMENT=$(grep "Term Alignment" "docs/notes_vocabulary_alignment.md" | grep -oE "[0-9]+%" | head -1 | tr -d '%')
    STRUCTURE_ALIGNMENT=$(grep "Structure Alignment" "docs/notes_vocabulary_alignment.md" | grep -oE "[0-9]+%" | head -1 | tr -d '%')
    NOTES_TO_DOCS_FLOW=$(grep "Notes -> Docs" "docs/notes_vocabulary_alignment.md" | grep -oE "[0-9]+%" | head -1 | tr -d '%')
    DOCS_TO_VOCAB_FLOW=$(grep "Docs -> Vocabulary" "docs/notes_vocabulary_alignment.md" | grep -oE "[0-9]+%" | head -1 | tr -d '%')
    MISSING_DOMAINS=$(grep "Missing Domains" "docs/notes_vocabulary_alignment.md" | grep -oE "[0-9]+" | head -1)
    
    echo -e "${GREEN}Notes Alignment Metrics:${NC}"
    echo -e "${GREEN}  Overall Alignment: $OVERALL_ALIGNMENT%${NC}"
    echo -e "${GREEN}  Term Alignment: $TERM_ALIGNMENT%${NC}"
    echo -e "${GREEN}  Structure Alignment: $STRUCTURE_ALIGNMENT%${NC}"
    echo -e "${GREEN}  Notes->Docs Flow: $NOTES_TO_DOCS_FLOW%${NC}"
    echo -e "${GREEN}  Docs->Vocabulary Flow: $DOCS_TO_VOCAB_FLOW%${NC}"
    echo -e "${GREEN}  Missing Domains: $MISSING_DOMAINS${NC}"
else
    echo -e "${YELLOW}Notes alignment report not found. Using default values.${NC}"
    OVERALL_ALIGNMENT=0
    TERM_ALIGNMENT=0
    STRUCTURE_ALIGNMENT=0
    NOTES_TO_DOCS_FLOW=0
    DOCS_TO_VOCAB_FLOW=0
    MISSING_DOMAINS=0
fi

# Extract workflow metrics if they exist
if [ "$WORKFLOW_REPORT_EXISTS" = true ] && [ -f "docs/workflow_analysis_report.md" ]; then
    echo -e "${BLUE}Extracting workflow metrics...${NC}"
    WORKFLOW_SCORE=$(grep "Overall Workflow Score" "docs/workflow_analysis_report.md" | grep -oE "[0-9]+%" | head -1 | tr -d '%')
    VOCAB_TO_CODE_FLOW=$(grep "Vocabulary -> Code" "docs/workflow_analysis_report.md" | grep -oE "[0-9]+%" | head -1 | tr -d '%')
    
    echo -e "${GREEN}Workflow Metrics:${NC}"
    echo -e "${GREEN}  Overall Workflow Score: $WORKFLOW_SCORE%${NC}"
    echo -e "${GREEN}  Vocabulary->Code Flow: $VOCAB_TO_CODE_FLOW%${NC}"
else
    echo -e "${YELLOW}Workflow report not found. Using default values.${NC}"
    WORKFLOW_SCORE=0
    VOCAB_TO_CODE_FLOW=0
fi

# Calculate quality rating based on all metrics
QUALITY_RATING=$(( (COMPLIANCE_PERCENT + CODE_REF_PERCENT + OVERALL_ALIGNMENT + WORKFLOW_SCORE) / 4 ))
if [ $QUALITY_RATING -ge 90 ]; then
    RATING="5/5"
    RATING_COLOR="brightgreen"
elif [ $QUALITY_RATING -ge 80 ]; then
    RATING="4/5"
    RATING_COLOR="green"
elif [ $QUALITY_RATING -ge 70 ]; then
    RATING="3/5"
    RATING_COLOR="yellowgreen"
elif [ $QUALITY_RATING -ge 60 ]; then
    RATING="2/5"
    RATING_COLOR="orange"
else
    RATING="1/5"
    RATING_COLOR="red"
fi

echo -e "${BLUE}Overall Quality Rating: $RATING ($QUALITY_RATING%)${NC}"

# Update the dashboard with new metrics
echo -e "${BLUE}Updating dashboard with new metrics...${NC}"

# Replace placeholder values in the dashboard
sed -i "s/\!\[Quality-[^]]*\]/\!\[Quality-$RATING\](https:\/\/img.shields.io\/badge\/Quality-$RATING-$RATING_COLOR)/g" "$DASHBOARD_FILE"
sed -i "s/\!\[Compliance-[^]]*\]/\!\[Compliance-$COMPLIANCE_PERCENT%\](https:\/\/img.shields.io\/badge\/Compliance-$COMPLIANCE_PERCENT%25-${COMPLIANCE_PERCENT -ge 90 && echo "brightgreen" || echo "orange"})/g" "$DASHBOARD_FILE"
sed -i "s/\!\[Terms-[^]]*\]/\!\[Terms-$TERM_COUNT\](https:\/\/img.shields.io\/badge\/Terms-$TERM_COUNT-blue)/g" "$DASHBOARD_FILE"
sed -i "s/\!\[Relationships-[^]]*\]/\!\[Relationships-$RELATIONSHIP_COUNT\](https:\/\/img.shields.io\/badge\/Relationships-$RELATIONSHIP_COUNT-blue)/g" "$DASHBOARD_FILE"
sed -i "s/\!\[Code_References-[^]]*\]/\!\[Code_References-$CODE_REF_PERCENT%\](https:\/\/img.shields.io\/badge\/Code_References-$CODE_REF_PERCENT%25-${CODE_REF_PERCENT -ge 75 && echo "brightgreen" || echo "orange"})/g" "$DASHBOARD_FILE"

if [ "$NOTES_ALIGNMENT_EXISTS" = true ]; then
    sed -i "s/\!\[Notes_Alignment-[^]]*\]/\!\[Notes_Alignment-$OVERALL_ALIGNMENT%\](https:\/\/img.shields.io\/badge\/Notes_Alignment-$OVERALL_ALIGNMENT%25-${OVERALL_ALIGNMENT -ge 70 && echo "green" || echo "orange"})/g" "$DASHBOARD_FILE"
fi

if [ "$WORKFLOW_REPORT_EXISTS" = true ]; then
    sed -i "s/\!\[Workflow-[^]]*\]/\!\[Workflow-$WORKFLOW_SCORE%\](https:\/\/img.shields.io\/badge\/Workflow-$WORKFLOW_SCORE%25-${WORKFLOW_SCORE -ge 70 && echo "green" || echo "orange"})/g" "$DASHBOARD_FILE"
fi

# Update Last Updated date
TODAY=$(date +%Y-%m-%d)
sed -i "s/\!\[Updated-[^]]*\]/\!\[Updated-$TODAY\](https:\/\/img.shields.io\/badge\/Updated-$TODAY-blue)/g" "$DASHBOARD_FILE"

# Update compliance metrics in the dashboard
sed -i "s/Compliance Rate: [0-9]\+%/Compliance Rate: $COMPLIANCE_PERCENT%/g" "$DASHBOARD_FILE"
sed -i "s/Compliant Files: [0-9]\+ \/ [0-9]\+/Compliant Files: $COMPLIANT_FILES \/ $TOTAL_FILES/g" "$DASHBOARD_FILE"

# Update content metrics in the dashboard
sed -i "s/Total Terms: [0-9]\+/Total Terms: $TERM_COUNT/g" "$DASHBOARD_FILE"
sed -i "s/Total Relationships: [0-9]\+/Total Relationships: $RELATIONSHIP_COUNT/g" "$DASHBOARD_FILE"
sed -i "s/Code Reference Coverage: [0-9]\+%/Code Reference Coverage: $CODE_REF_PERCENT%/g" "$DASHBOARD_FILE"

# Update notes alignment metrics if they exist
if [ "$NOTES_ALIGNMENT_EXISTS" = true ]; then
    # Check if the Notes Alignment section exists
    if grep -q "## Notes Alignment Metrics" "$DASHBOARD_FILE"; then
        # Update the metrics
        sed -i "s/Overall Alignment: [0-9]\+%/Overall Alignment: $OVERALL_ALIGNMENT%/g" "$DASHBOARD_FILE"
        sed -i "s/Term Alignment: [0-9]\+%/Term Alignment: $TERM_ALIGNMENT%/g" "$DASHBOARD_FILE"
        sed -i "s/Structure Alignment: [0-9]\+%/Structure Alignment: $STRUCTURE_ALIGNMENT%/g" "$DASHBOARD_FILE"
        sed -i "s/Notes → Docs Flow: [0-9]\+%/Notes → Docs Flow: $NOTES_TO_DOCS_FLOW%/g" "$DASHBOARD_FILE"
        sed -i "s/Docs → Vocabulary Flow: [0-9]\+%/Docs → Vocabulary Flow: $DOCS_TO_VOCAB_FLOW%/g" "$DASHBOARD_FILE"
        sed -i "s/Missing Domains: [0-9]\+/Missing Domains: $MISSING_DOMAINS/g" "$DASHBOARD_FILE"
    else
        # Add the section
        NOTES_SECTION="## Notes Alignment Metrics

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Overall Alignment | $OVERALL_ALIGNMENT% | 90% | ${OVERALL_ALIGNMENT -ge 90 && echo "✅" || echo "⚠️"} |
| Term Alignment | $TERM_ALIGNMENT% | 85% | ${TERM_ALIGNMENT -ge 85 && echo "✅" || echo "⚠️"} |
| Structure Alignment | $STRUCTURE_ALIGNMENT% | 95% | ${STRUCTURE_ALIGNMENT -ge 95 && echo "✅" || echo "⚠️"} |
| Notes → Docs Flow | $NOTES_TO_DOCS_FLOW% | 75% | ${NOTES_TO_DOCS_FLOW -ge 75 && echo "✅" || echo "⚠️"} |
| Docs → Vocabulary Flow | $DOCS_TO_VOCAB_FLOW% | 80% | ${DOCS_TO_VOCAB_FLOW -ge 80 && echo "✅" || echo "⚠️"} |
| Missing Domains | $MISSING_DOMAINS | 0 | ${MISSING_DOMAINS -eq 0 && echo "✅" || echo "⚠️"} |

"
        # Add the section before the Areas for Improvement section
        sed -i "/## Areas for Improvement/i $NOTES_SECTION" "$DASHBOARD_FILE"
    fi
fi

# Update workflow metrics if they exist
if [ "$WORKFLOW_REPORT_EXISTS" = true ]; then
    # Check if the Workflow Metrics section exists
    if grep -q "## Workflow Metrics" "$DASHBOARD_FILE"; then
        # Update the metrics
        sed -i "s/Overall Workflow Score: [0-9]\+%/Overall Workflow Score: $WORKFLOW_SCORE%/g" "$DASHBOARD_FILE"
        sed -i "s/Vocabulary → Code Flow: [0-9]\+%/Vocabulary → Code Flow: $VOCAB_TO_CODE_FLOW%/g" "$DASHBOARD_FILE"
    else
        # Add the section
        WORKFLOW_SECTION="## Workflow Metrics

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Overall Workflow Score | $WORKFLOW_SCORE% | 85% | ${WORKFLOW_SCORE -ge 85 && echo "✅" || echo "⚠️"} |
| Notes → Docs Flow | $NOTES_TO_DOCS_FLOW% | 75% | ${NOTES_TO_DOCS_FLOW -ge 75 && echo "✅" || echo "⚠️"} |
| Docs → Vocabulary Flow | $DOCS_TO_VOCAB_FLOW% | 80% | ${DOCS_TO_VOCAB_FLOW -ge 80 && echo "✅" || echo "⚠️"} |
| Vocabulary → Code Flow | $VOCAB_TO_CODE_FLOW% | 70% | ${VOCAB_TO_CODE_FLOW -ge 70 && echo "✅" || echo "⚠️"} |

"
        # Add the section after the Notes Alignment Metrics section or before Areas for Improvement
        if grep -q "## Notes Alignment Metrics" "$DASHBOARD_FILE"; then
            sed -i "/## Notes Alignment Metrics/a $WORKFLOW_SECTION" "$DASHBOARD_FILE"
        else
            sed -i "/## Areas for Improvement/i $WORKFLOW_SECTION" "$DASHBOARD_FILE"
        fi
    fi
fi

# Update Term Categories section
# Check if the Term Categories section exists
if grep -q "## Term Categories" "$DASHBOARD_FILE"; then
    # Update the metrics
    sed -i "s/Domain Objects: [0-9]\+ ([0-9]\+%)/Domain Objects: $DOMAIN_OBJECTS_COUNT ($DOMAIN_OBJECTS_PERCENT%)/g" "$DASHBOARD_FILE"
    sed -i "s/Technical Concepts: [0-9]\+ ([0-9]\+%)/Technical Concepts: $TECHNICAL_CONCEPTS_COUNT ($TECHNICAL_CONCEPTS_PERCENT%)/g" "$DASHBOARD_FILE"
    sed -i "s/Business Concepts: [0-9]\+ ([0-9]\+%)/Business Concepts: $BUSINESS_CONCEPTS_COUNT ($BUSINESS_CONCEPTS_PERCENT%)/g" "$DASHBOARD_FILE"
    sed -i "s/Cross-Cutting Terms: [0-9]\+ ([0-9]\+%)/Cross-Cutting Terms: $CROSS_CUTTING_COUNT ($CROSS_CUTTING_PERCENT%)/g" "$DASHBOARD_FILE"
else
    # Add the section
    CATEGORIES_SECTION="## Term Categories

| Category | Count | Percentage | Target | Status |
|----------|-------|------------|--------|--------|
| Domain Objects | $DOMAIN_OBJECTS_COUNT | $DOMAIN_OBJECTS_PERCENT% | 35-45% | ${DOMAIN_OBJECTS_PERCENT -ge 35 && DOMAIN_OBJECTS_PERCENT -le 45 && echo "✅" || echo "⚠️"} |
| Technical Concepts | $TECHNICAL_CONCEPTS_COUNT | $TECHNICAL_CONCEPTS_PERCENT% | 20-30% | ${TECHNICAL_CONCEPTS_PERCENT -ge 20 && TECHNICAL_CONCEPTS_PERCENT -le 30 && echo "✅" || echo "⚠️"} |
| Business Concepts | $BUSINESS_CONCEPTS_COUNT | $BUSINESS_CONCEPTS_PERCENT% | 20-30% | ${BUSINESS_CONCEPTS_PERCENT -ge 20 && BUSINESS_CONCEPTS_PERCENT -le 30 && echo "✅" || echo "⚠️"} |
| Cross-Cutting Terms | $CROSS_CUTTING_COUNT | $CROSS_CUTTING_PERCENT% | 5-15% | ${CROSS_CUTTING_PERCENT -ge 5 && CROSS_CUTTING_PERCENT -le 15 && echo "✅" || echo "⚠️"} |

"
    # Add the section after the Content Metrics section
    sed -i "/## Content Metrics/a $CATEGORIES_SECTION" "$DASHBOARD_FILE"
fi

# Update Relationship Types section
# Check if the Relationship Types section exists
if grep -q "## Relationship Types" "$DASHBOARD_FILE"; then
    # Update the metrics
    sed -i "s/Hierarchical: [0-9]\+ ([0-9]\+%)/Hierarchical: $HIERARCHICAL_REL_COUNT ($HIERARCHICAL_PERCENT%)/g" "$DASHBOARD_FILE"
    sed -i "s/Functional: [0-9]\+ ([0-9]\+%)/Functional: $FUNCTIONAL_REL_COUNT ($FUNCTIONAL_PERCENT%)/g" "$DASHBOARD_FILE"
    sed -i "s/Temporal: [0-9]\+ ([0-9]\+%)/Temporal: $TEMPORAL_REL_COUNT ($TEMPORAL_PERCENT%)/g" "$DASHBOARD_FILE"
    sed -i "s/Other: [0-9]\+ ([0-9]\+%)/Other: $OTHER_REL_COUNT ($OTHER_PERCENT%)/g" "$DASHBOARD_FILE"
else
    # Add the section
    RELATIONSHIP_SECTION="## Relationship Types

| Type | Count | Percentage | Target | Status |
|------|-------|------------|--------|--------|
| Hierarchical | $HIERARCHICAL_REL_COUNT | $HIERARCHICAL_PERCENT% | 30-40% | ${HIERARCHICAL_PERCENT -ge 30 && HIERARCHICAL_PERCENT -le 40 && echo "✅" || echo "⚠️"} |
| Functional | $FUNCTIONAL_REL_COUNT | $FUNCTIONAL_PERCENT% | 30-40% | ${FUNCTIONAL_PERCENT -ge 30 && FUNCTIONAL_PERCENT -le 40 && echo "✅" || echo "⚠️"} |
| Temporal | $TEMPORAL_REL_COUNT | $TEMPORAL_PERCENT% | 20-30% | ${TEMPORAL_PERCENT -ge 20 && TEMPORAL_PERCENT -le 30 && echo "✅" || echo "⚠️"} |
| Other | $OTHER_REL_COUNT | $OTHER_PERCENT% | 0-10% | ${OTHER_PERCENT -ge 0 && OTHER_PERCENT -le 10 && echo "✅" || echo "⚠️"} |

"
    # Add the section after the Term Categories section or Content Metrics
    if grep -q "## Term Categories" "$DASHBOARD_FILE"; then
        sed -i "/## Term Categories/a $RELATIONSHIP_SECTION" "$DASHBOARD_FILE"
    else
        sed -i "/## Content Metrics/a $RELATIONSHIP_SECTION" "$DASHBOARD_FILE"
    fi
fi

# Update quality rating in dashboard
sed -i "s/Quality Rating: [0-9]\/5/Quality Rating: $RATING/g" "$DASHBOARD_FILE"

# Update the generation date
sed -i "s/Generated: [0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}/Generated: $TODAY/g" "$DASHBOARD_FILE"

# Add to Recent Improvements section if quality improved
if grep -q "Quality Rating: [1-4]\/5" "${DASHBOARD_FILE}.bak" && [ "$RATING" = "5/5" ]; then
    echo -e "- $TODAY: Achieved 5/5 quality rating" >> "$DASHBOARD_FILE.improvements"
    sed -i "/## Recent Improvements/a - $TODAY: Achieved 5/5 quality rating" "$DASHBOARD_FILE"
elif [ $QUALITY_RATING -gt $(grep "Quality Rating:" "${DASHBOARD_FILE}.bak" | grep -oE "[0-9]+/5" | cut -d'/' -f1) ]; then
    echo -e "- $TODAY: Improved quality rating to $RATING" >> "$DASHBOARD_FILE.improvements"
    sed -i "/## Recent Improvements/a - $TODAY: Improved quality rating to $RATING" "$DASHBOARD_FILE"
fi

if [ $COMPLIANCE_PERCENT -eq 100 ] && grep -q "Compliance Rate: [0-9]\{1,2\}%" "${DASHBOARD_FILE}.bak"; then
    echo -e "- $TODAY: Achieved 100% vocabulary compliance" >> "$DASHBOARD_FILE.improvements"
    sed -i "/## Recent Improvements/a - $TODAY: Achieved 100% vocabulary compliance" "$DASHBOARD_FILE"
fi

if [ "$WORKFLOW_REPORT_EXISTS" = true ] && [ $WORKFLOW_SCORE -ge 85 ] && ! grep -q "Overall Workflow Score: [0-9]\{2\}%" "${DASHBOARD_FILE}.bak"; then
    echo -e "- $TODAY: Added workflow analysis metrics" >> "$DASHBOARD_FILE.improvements"
    sed -i "/## Recent Improvements/a - $TODAY: Added workflow analysis metrics" "$DASHBOARD_FILE"
fi

echo -e "${GREEN}Dashboard updated successfully!${NC}"
echo -e "${GREEN}Dashboard file: $DASHBOARD_FILE${NC}"

# Display final metrics summary
echo -e "${BLUE}=== Metrics Summary ===${NC}"
echo -e "${BLUE}Quality Rating: $RATING ($QUALITY_RATING%)${NC}"
echo -e "${BLUE}Compliance: $COMPLIANCE_PERCENT% ($COMPLIANT_FILES/$TOTAL_FILES files)${NC}"
echo -e "${BLUE}Content: $TERM_COUNT terms, $RELATIONSHIP_COUNT relationships, $CODE_REF_PERCENT% code refs${NC}"

if [ "$NOTES_ALIGNMENT_EXISTS" = true ]; then
    echo -e "${BLUE}Notes Alignment: $OVERALL_ALIGNMENT% overall, $TERM_ALIGNMENT% terms, $STRUCTURE_ALIGNMENT% structure${NC}"
fi

if [ "$WORKFLOW_REPORT_EXISTS" = true ]; then
    echo -e "${BLUE}Workflow: $WORKFLOW_SCORE% overall, $VOCAB_TO_CODE_FLOW% vocabulary→code flow${NC}"
fi

# Cleanup
rm -rf $TEMP_DIR

exit 0 