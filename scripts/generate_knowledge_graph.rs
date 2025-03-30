use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use regex::Regex;

#[derive(Debug)]
struct Term {
    name: String,
    category: String,
    term_type: String,
    taxonomy: String,
    definition: String,
    relationships: Vec<(String, String)>, // (relationship_type, related_term)
    usage_context: String,
    code_reference: String,
    domain: String,
}

fn main() -> io::Result<()> {
    println!("Generating Knowledge Graph from Vocabulary...");
    
    // Directory containing vocabulary domain files
    let vocab_dir = PathBuf::from("vocabulary/domains");
    
    // Output files
    let dot_file = PathBuf::from("docs/knowledge_graph.dot");
    let json_file = PathBuf::from("docs/knowledge_graph.json");
    
    // Parse vocabulary files
    let terms = parse_vocabulary_files(&vocab_dir)?;
    
    // Generate DOT file for GraphViz
    generate_dot_file(&terms, &dot_file)?;
    
    // Generate JSON representation
    generate_json_file(&terms, &json_file)?;
    
    println!("Knowledge graph generated:");
    println!("DOT file: {}", dot_file.display());
    println!("JSON file: {}", json_file.display());
    println!("\nTo visualize the graph, run: dot -Tsvg {} -o docs/knowledge_graph.svg", dot_file.display());
    
    Ok(())
}

fn parse_vocabulary_files(dir: &Path) -> io::Result<Vec<Term>> {
    let mut terms = Vec::new();
    
    // Patterns for extracting term information
    let term_pattern = Regex::new(r"### Term: (.+)").unwrap();
    let attribute_pattern = Regex::new(r"\*\*(.+)\*\*:\s*(.+)").unwrap();
    let relationship_pattern = Regex::new(r"\* (.+): (.+)").unwrap();
    
    // Process each .md file in the vocabulary/domains directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let domain = path.file_stem().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path)?;
            
            // Parse terms in the file
            let mut current_term: Option<Term> = None;
            let mut lines = content.lines().peekable();
            
            while let Some(line) = lines.next() {
                // Check for term header
                if let Some(caps) = term_pattern.captures(line) {
                    // If we're already parsing a term, add it to the list
                    if let Some(term) = current_term.take() {
                        terms.push(term);
                    }
                    
                    // Start a new term
                    current_term = Some(Term {
                        name: caps[1].trim().to_string(),
                        category: String::new(),
                        term_type: String::new(),
                        taxonomy: String::new(),
                        definition: String::new(),
                        relationships: Vec::new(),
                        usage_context: String::new(),
                        code_reference: String::new(),
                        domain: domain.clone(),
                    });
                } 
                // Check for attributes
                else if let Some(caps) = attribute_pattern.captures(line) {
                    let attr_name = caps[1].trim().to_lowercase();
                    let attr_value = caps[2].trim().to_string();
                    
                    if let Some(term) = &mut current_term {
                        match attr_name.as_str() {
                            "category" => term.category = attr_value,
                            "type" => term.term_type = attr_value,
                            "taxonomy" => term.taxonomy = attr_value,
                            "definition" => term.definition = attr_value,
                            "usage context" => term.usage_context = attr_value,
                            "code reference" => term.code_reference = attr_value,
                            _ => {}
                        }
                    }
                }
                // Check for relationships
                else if let Some(caps) = relationship_pattern.captures(line) {
                    let rel_type = caps[1].trim().to_string();
                    let related_term = caps[2].trim().to_string();
                    
                    if let Some(term) = &mut current_term {
                        term.relationships.push((rel_type, related_term));
                    }
                }
            }
            
            // Add the last term if there is one
            if let Some(term) = current_term {
                terms.push(term);
            }
        }
    }
    
    Ok(terms)
}

fn generate_dot_file(terms: &[Term], output_file: &Path) -> io::Result<()> {
    let mut file = File::create(output_file)?;
    
    // Write DOT file header
    writeln!(file, "digraph KnowledgeGraph {{")?;
    writeln!(file, "  graph [rankdir=LR, fontname=\"Arial\"];")?;
    writeln!(file, "  node [shape=box, style=filled, fontname=\"Arial\"];")?;
    writeln!(file, "  edge [fontname=\"Arial\"];")?;
    
    // Group nodes by domain
    writeln!(file, "  // Nodes grouped by domain")?;
    let domains: HashSet<_> = terms.iter().map(|t| &t.domain).collect();
    
    for domain in domains {
        writeln!(file, "  subgraph cluster_{} {{", domain)?;
        writeln!(file, "    label=\"{} Domain\";", domain.replace('_', " ").to_uppercase())?;
        writeln!(file, "    style=filled;")?;
        writeln!(file, "    color=lightgrey;")?;
        writeln!(file, "    node [style=filled, fillcolor=white];")?;
        
        // Add nodes for this domain
        for term in terms.iter().filter(|t| &t.domain == domain) {
            // Escape double quotes in the definition
            let def = term.definition.replace('\"', "\\\"");
            // Truncate definition if too long
            let short_def = if def.len() > 100 {
                format!("{}...", &def[..97])
            } else {
                def
            };
            
            // Color based on term type
            let color = match term.term_type.as_str() {
                "Entity" => "lightblue",
                "Value Object" => "lightgreen",
                "Aggregate" => "pink",
                "Service" => "lightyellow",
                "Platform" => "lightcyan",
                "Tool" => "lavender",
                "Protocol" => "peachpuff",
                "Framework" => "thistle",
                _ => "white",
            };
            
            writeln!(
                file,
                "    \"{}\" [label=\"{}\nType: {}\nTaxonomy: {}\", tooltip=\"{}\", fillcolor=\"{}\"];",
                term.name, term.name, term.term_type, term.taxonomy, short_def, color
            )?;
        }
        
        writeln!(file, "  }}")?;
    }
    
    // Add edges for relationships
    writeln!(file, "  // Edges for relationships")?;
    
    // Create a map for quick term lookup
    let term_map: HashMap<_, _> = terms.iter().map(|t| (t.name.clone(), t)).collect();
    
    for term in terms {
        for (rel_type, related_term) in &term.relationships {
            // Skip if related term contains commas (likely a list)
            if related_term.contains(',') {
                // For comma-separated lists, create multiple edges
                for rt in related_term.split(',').map(|s| s.trim()) {
                    if term_map.contains_key(rt) {
                        writeln!(
                            file,
                            "  \"{}\" -> \"{}\" [label=\"{}\"];",
                            term.name, rt, rel_type
                        )?;
                    }
                }
            } else {
                // Check if the related term exists in our graph
                if term_map.contains_key(related_term) {
                    writeln!(
                        file,
                        "  \"{}\" -> \"{}\" [label=\"{}\"];",
                        term.name, related_term, rel_type
                    )?;
                }
            }
        }
    }
    
    // Close the DOT file
    writeln!(file, "}}")?;
    
    Ok(())
}

fn generate_json_file(terms: &[Term], output_file: &Path) -> io::Result<()> {
    let mut file = File::create(output_file)?;
    
    // Generate simple JSON structure
    writeln!(file, "{{")?;
    writeln!(file, "  \"nodes\": [")?;
    
    // Add nodes
    for (i, term) in terms.iter().enumerate() {
        let comma = if i < terms.len() - 1 { "," } else { "" };
        writeln!(
            file,
            "    {{\"id\": \"{}\", \"label\": \"{}\", \"domain\": \"{}\", \"type\": \"{}\", \"taxonomy\": \"{}\"}}{}",
            term.name, term.name, term.domain, term.term_type, term.taxonomy, comma
        )?;
    }
    
    writeln!(file, "  ],")?;
    writeln!(file, "  \"edges\": [")?;
    
    // Create a map for quick term lookup
    let term_map: HashMap<_, _> = terms.iter().map(|t| (t.name.clone(), t)).collect();
    
    // Add edges
    let mut edges = Vec::new();
    for term in terms {
        for (rel_type, related_term) in &term.relationships {
            // Skip if related term contains commas (likely a list)
            if related_term.contains(',') {
                // For comma-separated lists, create multiple edges
                for rt in related_term.split(',').map(|s| s.trim()) {
                    if term_map.contains_key(rt) {
                        edges.push((term.name.clone(), rt.to_string(), rel_type.clone()));
                    }
                }
            } else {
                // Check if the related term exists in our graph
                if term_map.contains_key(related_term) {
                    edges.push((term.name.clone(), related_term.clone(), rel_type.clone()));
                }
            }
        }
    }
    
    // Write edges to file
    for (i, (source, target, rel_type)) in edges.iter().enumerate() {
        let comma = if i < edges.len() - 1 { "," } else { "" };
        writeln!(
            file,
            "    {{\"source\": \"{}\", \"target\": \"{}\", \"label\": \"{}\"}}{}",
            source, target, rel_type, comma
        )?;
    }
    
    writeln!(file, "  ]")?;
    writeln!(file, "}}")?;
    
    Ok(())
} 