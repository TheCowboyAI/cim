#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Read the modules graph
const graphPath = path.join(__dirname, '..', 'registry', 'modules-graph.json');
const graphData = JSON.parse(fs.readFileSync(graphPath, 'utf8'));

// Generate Mermaid graph
function generateMermaid(graph) {
  let mermaid = `graph TB
    %% CIM Module Dependency Graph
    %% Auto-generated from modules-graph.json
    %% Last updated: ${graph.metadata.last_updated}
    
    %% Nodes
`;

  // Group nodes by category
  const categories = {
    'template': 'Template',
    'core': 'Core Infrastructure',
    'domain': 'Domain Modules',
    'storage': 'Storage Modules',
    'graph': 'Graph Systems',
    'security': 'Security Modules',
    'edge': 'Edge Computing'
  };

  // Create subgraphs for each category
  const nodesByType = {};
  Object.entries(graph.nodes).forEach(([id, node]) => {
    const type = node.type || 'core';
    if (!nodesByType[type]) nodesByType[type] = [];
    nodesByType[type].push(node);
  });

  // Generate subgraphs
  Object.entries(nodesByType).forEach(([type, nodes]) => {
    const categoryName = categories[type] || type;
    mermaid += `    subgraph "${categoryName}"\n`;
    
    nodes.forEach(node => {
      const status = node.status;
      const label = `${node.id}<br/>${getStatusEmoji(status)}`;
      const style = getNodeStyle(status);
      
      mermaid += `        ${node.id}["${label}"]\n`;
      if (style) {
        mermaid += `        style ${node.id} ${style}\n`;
      }
    });
    
    mermaid += `    end\n\n`;
  });

  // Add edges
  mermaid += `    %% Dependencies\n`;
  graph.edges.forEach(edge => {
    if (edge.type === 'dependency') {
      mermaid += `    ${edge.from} --> ${edge.to}\n`;
    }
  });

  // Add legend
  mermaid += `
    %% Legend
    subgraph "Legend"
        production["Production Ready 🟢"]
        development["In Development 🟡"]
        template["Template 🔵"]
        
        style production fill:#2ECC71,stroke:#27AE60,stroke-width:3px,color:#FFF
        style development fill:#F39C12,stroke:#E67E22,stroke-width:2px,color:#FFF
        style template fill:#3498DB,stroke:#2980B9,stroke-width:2px,color:#FFF
    end
`;

  return mermaid;
}

function getStatusEmoji(status) {
  switch (status) {
    case 'production': return '🟢';
    case 'development': return '🟡';
    case 'template': return '🔵';
    default: return '⚪';
  }
}

function getNodeStyle(status) {
  switch (status) {
    case 'production':
      return 'fill:#2ECC71,stroke:#27AE60,stroke-width:3px,color:#FFF';
    case 'development':
      return 'fill:#F39C12,stroke:#E67E22,stroke-width:2px,color:#FFF';
    case 'template':
      return 'fill:#3498DB,stroke:#2980B9,stroke-width:2px,color:#FFF';
    default:
      return 'fill:#95A5A6,stroke:#7F8C8D,stroke-width:2px,color:#FFF';
  }
}

// Generate and output the Mermaid graph
const mermaidGraph = generateMermaid(graphData.graph);
console.log(mermaidGraph);