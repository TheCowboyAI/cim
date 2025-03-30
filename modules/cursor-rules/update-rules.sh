#!/usr/bin/env bash
# update-rules.sh - Manage and update cursor rules
#
# This script provides utilities for managing cursor rules, including
# synchronizing them between the module configuration and the .cursor/rules directory.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CURSOR_RULES_DIR="$PROJECT_ROOT/.cursor/rules"
CONFIG_FILE="$SCRIPT_DIR/config.nix"
DEFAULT_FILE="$SCRIPT_DIR/default.nix"

# Color outputs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print header
print_header() {
  echo -e "${BLUE}=======================================${NC}"
  echo -e "${BLUE}      Cursor Rules Manager v1.0        ${NC}"
  echo -e "${BLUE}=======================================${NC}"
  echo ""
}

# Print usage
print_usage() {
  echo "Usage: $0 [command]"
  echo ""
  echo "Commands:"
  echo "  sync       Synchronize rules between module and .cursor/rules directory"
  echo "  export     Export rules from module to .cursor/rules directory"
  echo "  import     Import rules from .cursor/rules directory to module"
  echo "  list       List all configured rules"
  echo "  status     Show status of rules"
  echo "  help       Show this help message"
  echo ""
}

# List all rules defined in the module
list_rules() {
  echo -e "${BLUE}Cursor Rules:${NC}"
  echo ""
  
  # Ensure nix is available
  if ! command -v nix &> /dev/null; then
    echo -e "${RED}Error: nix command not found${NC}"
    exit 1
  fi
  
  # TODO: Implement actual rule listing with nix eval
  # For now, we'll use a simple grep to find rule definitions
  grep -n '".*"' "$CONFIG_FILE" | grep -E '= \{$' | while read -r line; do
    rule_name=$(echo "$line" | sed -E 's/.*"(.*)".*/\1/')
    rule_line=$(echo "$line" | cut -d':' -f1)
    
    # Extract description
    desc_line=$((rule_line + 2))
    description=$(sed "${desc_line}q;d" "$CONFIG_FILE" | sed -E 's/.*"(.*)".*/\1/')
    
    # Extract enabled status
    enabled_line=$((rule_line + 3))
    enabled=$(sed "${enabled_line}q;d" "$CONFIG_FILE" | grep -q "true" && echo "enabled" || echo "disabled")
    
    if [[ "$enabled" == "enabled" ]]; then
      status="${GREEN}●${NC}"
    else
      status="${RED}○${NC}"
    fi
    
    echo -e "$status ${YELLOW}$rule_name${NC}: $description"
  done
}

# Check if the .cursor/rules directory exists
check_cursor_dir() {
  if [[ ! -d "$CURSOR_RULES_DIR" ]]; then
    echo -e "${YELLOW}Creating .cursor/rules directory...${NC}"
    mkdir -p "$CURSOR_RULES_DIR"
  fi
}

# Export rules from the module to .cursor/rules
export_rules() {
  check_cursor_dir
  
  echo -e "${BLUE}Exporting rules from module to .cursor/rules...${NC}"
  echo ""
  
  # TODO: Implement proper rule export using nix
  # For now, just print a message
  echo -e "${YELLOW}This functionality is not yet implemented.${NC}"
  echo "In the future, this will extract rule content from the module configuration"
  echo "and create corresponding .mdc files in the .cursor/rules directory."
}

# Import rules from .cursor/rules to the module
import_rules() {
  check_cursor_dir
  
  echo -e "${BLUE}Importing rules from .cursor/rules to module...${NC}"
  echo ""
  
  # TODO: Implement proper rule import using nix
  # For now, just print a message
  echo -e "${YELLOW}This functionality is not yet implemented.${NC}"
  echo "In the future, this will scan the .cursor/rules directory for .mdc files"
  echo "and update the module configuration accordingly."
}

# Synchronize rules bidirectionally
sync_rules() {
  echo -e "${BLUE}Synchronizing rules...${NC}"
  echo ""
  
  # TODO: Implement proper rule synchronization
  # For now, just print a message
  echo -e "${YELLOW}This functionality is not yet implemented.${NC}"
  echo "In the future, this will synchronize rules between the module configuration"
  echo "and the .cursor/rules directory, merging changes from both sources."
}

# Show status of rules
show_status() {
  echo -e "${BLUE}Cursor Rules Status:${NC}"
  echo ""
  
  # Check module files
  if [[ -f "$DEFAULT_FILE" ]]; then
    echo -e "${GREEN}✓${NC} Module definition file exists"
  else
    echo -e "${RED}✗${NC} Module definition file is missing"
  fi
  
  if [[ -f "$CONFIG_FILE" ]]; then
    echo -e "${GREEN}✓${NC} Module configuration file exists"
  else
    echo -e "${RED}✗${NC} Module configuration file is missing"
  fi
  
  # Check cursor rules directory
  if [[ -d "$CURSOR_RULES_DIR" ]]; then
    rule_count=$(find "$CURSOR_RULES_DIR" -name "*.mdc" -type f | wc -l)
    echo -e "${GREEN}✓${NC} .cursor/rules directory exists with $rule_count rule files"
  else
    echo -e "${RED}✗${NC} .cursor/rules directory is missing"
  fi
  
  echo ""
  list_rules
}

# Main function
main() {
  print_header
  
  # Process command
  case "$1" in
    "sync")
      sync_rules
      ;;
    "export")
      export_rules
      ;;
    "import")
      import_rules
      ;;
    "list")
      list_rules
      ;;
    "status")
      show_status
      ;;
    "help"|"")
      print_usage
      ;;
    *)
      echo -e "${RED}Error: Unknown command '$1'${NC}"
      print_usage
      exit 1
      ;;
  esac
}

main "$@" 