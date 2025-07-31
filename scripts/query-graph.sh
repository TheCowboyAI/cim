#!/bin/bash
# Query the CIM module graph

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GRAPH_FILE="$SCRIPT_DIR/../registry/modules-graph.json"

# Function to display usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Query the CIM module dependency graph

OPTIONS:
    -m, --module MODULE     Show details for a specific module
    -d, --dependencies MOD  Show dependencies of a module
    -r, --dependents MOD    Show modules that depend on this module
    -g, --git MODULE        Show git information for a module
    -p, --production        List all production modules with git info
    --outdated              Show modules not updated in 30 days
    --graph                 Generate dependency graph visualization
    -h, --help              Show this help message

EXAMPLES:
    $0 --module cim-domain          # Show details for cim-domain
    $0 --dependencies cim-security  # Show what cim-security depends on
    $0 --dependents cim-domain      # Show what depends on cim-domain
    $0 --git cim-ipld              # Show git commit/version info
    $0 --production                 # List production modules with versions

EOF
}

# Function to show module details
show_module_details() {
    local module=$1
    
    if ! jq -e ".graph.nodes[\"$module\"]" "$GRAPH_FILE" > /dev/null 2>&1; then
        echo -e "${RED}Module '$module' not found${NC}"
        return 1
    fi
    
    echo -e "${BLUE}Module: ${module}${NC}"
    jq -r ".graph.nodes[\"$module\"] | 
        \"Type: \\(.type)
Status: \\(.status)
Repository: \\(.repository)
Current Version: \\(.version.current)
Latest Commit: \\(.git.latest_commit // \"unknown\")
Last Updated: \\(.git.last_updated // \"unknown\")
Dependencies: \\(.dependencies | length)
Dependents: \\(.dependents | length)\"" "$GRAPH_FILE"
}

# Function to show dependencies
show_dependencies() {
    local module=$1
    
    echo -e "${BLUE}Dependencies of ${module}:${NC}"
    deps=$(jq -r ".graph.nodes[\"$module\"].dependencies[]? // empty" "$GRAPH_FILE" 2>/dev/null)
    
    if [ -z "$deps" ]; then
        echo "No dependencies"
    else
        echo "$deps" | while read -r dep; do
            status=$(jq -r ".graph.nodes[\"$dep\"].status // \"unknown\"" "$GRAPH_FILE")
            echo -e "  • $dep ${YELLOW}[$status]${NC}"
        done
    fi
}

# Function to show dependents
show_dependents() {
    local module=$1
    
    echo -e "${BLUE}Modules that depend on ${module}:${NC}"
    deps=$(jq -r ".graph.nodes[\"$module\"].dependents[]? // empty" "$GRAPH_FILE" 2>/dev/null)
    
    if [ -z "$deps" ]; then
        echo "No dependents"
    else
        echo "$deps" | while read -r dep; do
            status=$(jq -r ".graph.nodes[\"$dep\"].status // \"unknown\"" "$GRAPH_FILE")
            echo -e "  • $dep ${YELLOW}[$status]${NC}"
        done
    fi
}

# Function to show git information
show_git_info() {
    local module=$1
    
    echo -e "${BLUE}Git information for ${module}:${NC}"
    jq -r ".graph.nodes[\"$module\"] | 
        \"Latest Commit: \\(.git.latest_commit // \"not tracked\")
Latest Tag: \\(.git.latest_tag // \"no tags\")
Last Updated: \\(.git.last_updated // \"never\")
Current Version: \\(.version.current)
Releases: \\(.version.releases | join(\", \") // \"none\")\"" "$GRAPH_FILE"
}

# Function to show production modules
show_production_modules() {
    echo -e "${GREEN}Production-Ready Modules:${NC}\n"
    
    jq -r '.graph.nodes | to_entries[] | 
        select(.value.status == "production") | 
        "\(.key)|\(.value.version.current)|\(.value.git.latest_commit // "unknown")|\(.value.git.last_updated // "never")"' \
        "$GRAPH_FILE" | \
    while IFS='|' read -r module version commit updated; do
        printf "%-25s v%-8s %s  %s\n" "$module" "$version" "$commit" "$updated"
    done
}

# Function to show outdated modules
show_outdated_modules() {
    echo -e "${YELLOW}Modules not updated in 30+ days:${NC}\n"
    
    THIRTY_DAYS_AGO=$(date -d "30 days ago" -Iseconds)
    
    jq -r --arg cutoff "$THIRTY_DAYS_AGO" '.graph.nodes | to_entries[] | 
        select(.value.git.last_updated // "1970-01-01T00:00:00Z" < $cutoff) | 
        "\(.key)|\(.value.git.last_updated // "never")"' \
        "$GRAPH_FILE" | \
    while IFS='|' read -r module updated; do
        printf "%-25s %s\n" "$module" "$updated"
    done
}

# Function to generate graph visualization
generate_graph() {
    echo -e "${BLUE}Generating dependency graph...${NC}"
    
    if [ -x "$SCRIPT_DIR/generate-graph-viz.js" ]; then
        node "$SCRIPT_DIR/generate-graph-viz.js"
    else
        echo -e "${RED}Graph generation script not found${NC}"
        return 1
    fi
}

# Main script
main() {
    if [ $# -eq 0 ]; then
        usage
        exit 0
    fi

    while [[ $# -gt 0 ]]; do
        case $1 in
            -m|--module)
                show_module_details "$2"
                shift 2
                ;;
            -d|--dependencies)
                show_dependencies "$2"
                shift 2
                ;;
            -r|--dependents)
                show_dependents "$2"
                shift 2
                ;;
            -g|--git)
                show_git_info "$2"
                shift 2
                ;;
            -p|--production)
                show_production_modules
                shift
                ;;
            --outdated)
                show_outdated_modules
                shift
                ;;
            --graph)
                generate_graph
                shift
                ;;
            -h|--help)
                usage
                exit 0
                ;;
            *)
                echo -e "${RED}Unknown option: $1${NC}"
                usage
                exit 1
                ;;
        esac
    done
}

main "$@"