#!/bin/bash
set -e

echo "CIM Knowledge Graph Generator"
echo "============================"
echo

# Run the main Mermaid graph generator
echo "Generating main knowledge graph..."
./scripts/generate_mermaid_graph.sh

# Run the domain-specific graph generator
echo "Generating domain-specific knowledge graphs..."
./scripts/generate_domain_graphs.sh

echo
echo "Knowledge graph generation complete. You can view the graphs in:"
echo "  - Main graph: docs/knowledge_graph.md"
echo "  - Domain graphs: docs/knowledge_graphs/" 