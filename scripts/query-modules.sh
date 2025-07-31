#!/bin/bash
# Query CIM modules from the registry

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Base URL for raw GitHub content
REGISTRY_BASE="https://raw.githubusercontent.com/thecowboyai/cim/main/registry"

# Function to display usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Query the CIM module registry

OPTIONS:
    -l, --list              List all modules
    -t, --type TYPE         Filter by module type (core, domain, template)
    -c, --category CAT      Filter by category
    -s, --search TERM       Search modules by name or description
    -p, --private           Show private domains
    --status STATUS         Filter by status (production, development, template)
    --production            Show only production-ready modules
    -h, --help              Show this help message

EXAMPLES:
    $0 --list                    # List all modules
    $0 --type domain            # Show only domain modules
    $0 --category security      # Show security-related modules
    $0 --search workflow        # Search for workflow modules
    $0 --private                # Show private domain registry
    $0 --production             # Show only production-ready modules
    $0 --status development     # Show modules in development

EOF
}

# Function to list all modules
list_modules() {
    echo -e "${BLUE}Fetching CIM modules...${NC}"
    curl -s "${REGISTRY_BASE}/modules.yaml" | grep -E "^  [a-zA-Z]" | sed 's/://g' | sort
}

# Function to filter by type
filter_by_type() {
    local type=$1
    echo -e "${BLUE}Fetching ${type} modules...${NC}"
    curl -s "${REGISTRY_BASE}/modules.yaml" | \
        awk -v type="$type" '
        /^  [a-zA-Z]/ { module = $1; gsub(/:/, "", module) }
        /type:/ && $2 == type { print module }
        '
}

# Function to filter by category
filter_by_category() {
    local category=$1
    echo -e "${BLUE}Fetching modules in category: ${category}${NC}"
    curl -s "${REGISTRY_BASE}/modules.yaml" | \
        awk -v cat="$category" '
        /^  [a-zA-Z]/ { module = $1; gsub(/:/, "", module) }
        /category:/ && $2 == cat { print module }
        '
}

# Function to search modules
search_modules() {
    local term=$1
    echo -e "${BLUE}Searching for: ${term}${NC}"
    curl -s "${REGISTRY_BASE}/modules.yaml" | \
        awk -v term="$term" '
        BEGIN { IGNORECASE = 1 }
        /^  [a-zA-Z]/ { 
            module = $1; 
            gsub(/:/, "", module)
            in_module = 1
        }
        in_module && /description:/ {
            desc = $0
            if (module ~ term || desc ~ term) {
                print module " -" substr(desc, index(desc, ":") + 1)
            }
            in_module = 0
        }
        '
}

# Function to filter by status
filter_by_status() {
    local status=$1
    echo -e "${BLUE}Fetching modules with status: ${status}${NC}"
    curl -s "${REGISTRY_BASE}/modules.yaml" | \
        awk -v status="$status" '
        /^  [a-zA-Z]/ { 
            module = $1
            gsub(/:/, "", module)
            in_module = 1
            found_status = 0
        }
        in_module && /status:/ && $2 == status { 
            found_status = 1
        }
        in_module && /description:/ && found_status {
            desc = $0
            print module " -" substr(desc, index(desc, ":") + 1)
            in_module = 0
            found_status = 0
        }
        /^  [a-zA-Z]/ && in_module {
            in_module = 0
            found_status = 0
        }
        '
}

# Function to show production modules
show_production_modules() {
    echo -e "${GREEN}Production-Ready Modules:${NC}"
    echo -e "${YELLOW}These modules meet all production standards:${NC}\n"
    curl -s "${REGISTRY_BASE}/modules.yaml" | \
        awk '
        /^  [a-zA-Z]/ { 
            module = $1
            gsub(/:/, "", module)
            in_module = 1
            is_production = 0
        }
        in_module && /status: production/ { 
            is_production = 1
        }
        in_module && /description:/ && is_production {
            desc = $0
            gsub(/.*description: "/, "", desc)
            gsub(/".*/, "", desc)
            printf "\033[0;32m%-25s\033[0m %s\n", module, desc
            in_module = 0
            is_production = 0
        }
        /^  [a-zA-Z]/ && in_module {
            in_module = 0
            is_production = 0
        }
        '
}

# Function to show private domains
show_private_domains() {
    echo -e "${BLUE}Private Domain Implementations:${NC}"
    curl -s "${REGISTRY_BASE}/private-domains.yaml" | \
        awk '
        /^  [a-zA-Z_]+:/ { 
            sector = $1
            gsub(/:/, "", sector)
            gsub(/_/, " ", sector)
            print "\n" toupper(sector)
        }
        /name:/ { print "  •" $2 }
        /status:/ { 
            status = $2
            gsub(/"/, "", status)
            if (status == "production") color = "\033[0;32m"
            else if (status == "development") color = "\033[1;33m"
            else if (status == "pilot") color = "\033[0;34m"
            else color = "\033[0m"
            print "    Status: " color status "\033[0m"
        }
        '
}

# Main script
main() {
    if [ $# -eq 0 ]; then
        usage
        exit 0
    fi

    while [[ $# -gt 0 ]]; do
        case $1 in
            -l|--list)
                list_modules
                exit 0
                ;;
            -t|--type)
                filter_by_type "$2"
                shift 2
                exit 0
                ;;
            -c|--category)
                filter_by_category "$2"
                shift 2
                exit 0
                ;;
            -s|--search)
                search_modules "$2"
                shift 2
                exit 0
                ;;
            -p|--private)
                show_private_domains
                exit 0
                ;;
            --status)
                filter_by_status "$2"
                shift 2
                exit 0
                ;;
            --production)
                show_production_modules
                exit 0
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