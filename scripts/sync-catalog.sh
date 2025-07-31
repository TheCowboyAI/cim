#!/bin/bash
# Sync modules-graph.json with cim_modules_catalog.md

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

GRAPH_FILE="$PROJECT_ROOT/registry/modules-graph.json"
CATALOG_FILE="$PROJECT_ROOT/doc/cim_modules_catalog.md"

echo "Syncing module graph with catalog..."

# Extract production modules count
PRODUCTION_COUNT=$(jq '.graph.metadata.production_ready' "$GRAPH_FILE")
TOTAL_COUNT=$(jq '.graph.metadata.total_modules' "$GRAPH_FILE")

# Update the production-ready modules section in the catalog
TEMP_FILE=$(mktemp)

# Read the catalog and update production module information
awk -v prod_count="$PRODUCTION_COUNT" -v total="$TOTAL_COUNT" '
/^### Production-Ready Modules/ {
    print
    print "Only " prod_count " modules currently meet production standards:"
    next
}
/^Only [0-9]+ modules currently meet production standards:/ {
    next
}
{ print }
' "$CATALOG_FILE" > "$TEMP_FILE"

# Get git info for each module from the graph
echo "Updating module git information..."

# Create a temporary file with module status
jq -r '.graph.nodes | to_entries[] | "\(.key):\(.value.status):\(.value.git.latest_commit // "unknown"):\(.value.version.current)"' "$GRAPH_FILE" > /tmp/module_status.txt

# Update mindmap with status indicators
awk '
BEGIN {
    while ((getline line < "/tmp/module_status.txt") > 0) {
        split(line, parts, ":")
        module = parts[1]
        status = parts[2]
        commit = parts[3]
        version = parts[4]
        
        # Add status emoji
        if (status == "production") emoji = " ✅"
        else if (status == "development") emoji = " 🚧"
        else if (status == "template") emoji = " 📋"
        else emoji = ""
        
        module_info[module] = emoji
    }
}
/^      / && $1 in module_info {
    print $0 module_info[$1]
    next
}
{ print }
' "$TEMP_FILE" > "$CATALOG_FILE"

# Clean up
rm -f "$TEMP_FILE" /tmp/module_status.txt

# Generate updated timestamp
TIMESTAMP=$(date -I)
sed -i "s/Last updated: .*/Last updated: $TIMESTAMP/" "$CATALOG_FILE"

echo "Catalog sync complete!"