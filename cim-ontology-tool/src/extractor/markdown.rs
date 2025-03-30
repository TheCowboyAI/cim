//! Markdown extractor for ontology extraction
//!
//! This module provides functionality for extracting ontology information
//! from markdown files.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use anyhow::{Result, Context, anyhow};
use regex::Regex;
use uuid::Uuid;
use once_cell::sync::Lazy;

use crate::ontology::{Term, Relationship, Source};

// Regex patterns for extracting terms and relationships
// Pattern for finding term definitions (## Term Name)
static TERM_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^##\s+(.+)$").expect("Failed to compile term pattern regex")
});

// Pattern for finding term attributes (- **attribute**: value)
static ATTRIBUTE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^-\s+\*\*([^:]+)\*\*:\s+(.+)$").expect("Failed to compile attribute pattern regex")
});

// Pattern for finding relationships (- **related to**: [Term2](relationship_type))
static RELATIONSHIP_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^-\s+\*\*([^:]+)\*\*:\s+\[([^]]+)\]\(([^)]+)\)$").expect("Failed to compile relationship pattern regex")
});

// Pattern for term definition in Cypher style (CREATE (term:Term {name:"Term Name", definition:"Definition"}))
static CYPHER_TERM_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"CREATE\s+\(.*:Term\s+\{([^}]+)\}\)").expect("Failed to compile cypher term pattern regex")
});

// Pattern for relationship definition in Cypher style (CREATE (term1)-[:RELATIONSHIP_TYPE]->(term2))
static CYPHER_RELATIONSHIP_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"CREATE\s+\(([^)]+)\)-\[:([^]]+)\]->\(([^)]+)\)").expect("Failed to compile cypher relationship pattern regex")
});

/// Common relationship types
pub mod relationship_types {
    pub const IS_A: &str = "is_a";
    pub const PART_OF: &str = "part_of";
    pub const HAS_PART: &str = "has_part";
    pub const RELATED_TO: &str = "related_to";
    pub const SYNONYM_OF: &str = "synonym_of";
    pub const ANTONYM_OF: &str = "antonym_of";
    pub const INSTANCE_OF: &str = "instance_of";
    pub const CAUSES: &str = "causes";
    pub const PRECEDES: &str = "precedes";
    pub const FOLLOWS: &str = "follows";
    pub const MANAGES: &str = "manages";
    pub const CONFIGURES: &str = "configures";
    pub const DEPENDS_ON: &str = "depends_on";
    pub const VALIDATES: &str = "validates";
}

/// Extract terms and relationships from a markdown file
///
/// # Arguments
///
/// * `path` - Path to the markdown file
/// * `ontology_id` - ID of the ontology to associate extracted terms with (can be empty)
///
/// # Returns
///
/// * `Result<(Vec<Term>, Vec<Relationship>)>` - Extracted terms and relationships
pub fn extract_from_markdown(path: &Path, ontology_id: &str) -> Result<(Vec<Term>, Vec<Relationship>)> {
    let file = File::open(path).with_context(|| format!("Failed to open markdown file: {:?}", path))?;
    let reader = BufReader::new(file);
    
    let mut terms = Vec::new();
    let mut relationships = Vec::new();
    let mut current_term: Option<Term> = None;
    let mut term_id_map: HashMap<String, Uuid> = HashMap::new();
    
    // Create source information
    let file_name = path.file_name()
        .map(|os_str| os_str.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown.md".to_string());
    
    let source = Source::from_file(path.to_path_buf());
    
    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result.with_context(|| format!("Failed to read line {} from file", line_number + 1))?;
        let line = line.trim();
        
        // Skip empty lines
        if line.is_empty() {
            continue;
        }
        
        // Check for term definitions
        if let Some(caps) = TERM_PATTERN.captures(line) {
            // If we were parsing a term, add it to our list
            if let Some(term) = current_term.take() {
                terms.push(term);
            }
            
            // Create a new term
            let term_name = caps[1].trim().to_string();
            let term_id = Uuid::new_v4();
            term_id_map.insert(term_name.clone(), term_id);
            
            let mut term = Term {
                id: term_id,
                name: term_name,
                definition: None,
                domain: None,
                sources: vec![source.clone()],
                metadata: HashMap::new(),
            };
            
            if !ontology_id.is_empty() {
                term.metadata.insert("ontology_id".to_string(), ontology_id.to_string());
            }
            
            current_term = Some(term);
        }
        // Check for term attributes
        else if let Some(caps) = ATTRIBUTE_PATTERN.captures(line) {
            if let Some(term) = &mut current_term {
                let attr_name = caps[1].trim().to_lowercase();
                let attr_value = caps[2].trim().to_string();
                
                match attr_name.as_str() {
                    "definition" => {
                        term.definition = Some(attr_value);
                    },
                    "domain" => {
                        term.domain = Some(attr_value);
                    },
                    _ => {
                        // Store as metadata
                        term.metadata.insert(attr_name, attr_value);
                    }
                }
            }
        }
        // Check for relationships
        else if let Some(caps) = RELATIONSHIP_PATTERN.captures(line) {
            if let Some(term) = &current_term {
                let rel_type = caps[1].trim().to_lowercase().replace(" ", "_");
                let target_term_name = caps[2].trim().to_string();
                let rel_subtype = caps[3].trim().to_string();
                
                // We might not have seen the target term yet, so we'll create a placeholder ID
                let target_term_id = term_id_map
                    .entry(target_term_name.clone())
                    .or_insert_with(Uuid::new_v4)
                    .clone();
                
                // Determine the relationship type
                let relationship_type = match rel_type.as_str() {
                    "is a" | "is an" | "is" => relationship_types::IS_A.to_string(),
                    "part of" => relationship_types::PART_OF.to_string(),
                    "has part" => relationship_types::HAS_PART.to_string(),
                    "related to" => relationship_types::RELATED_TO.to_string(),
                    "synonym of" => relationship_types::SYNONYM_OF.to_string(),
                    "antonym of" => relationship_types::ANTONYM_OF.to_string(),
                    "instance of" => relationship_types::INSTANCE_OF.to_string(),
                    "causes" => relationship_types::CAUSES.to_string(),
                    "precedes" => relationship_types::PRECEDES.to_string(),
                    "follows" => relationship_types::FOLLOWS.to_string(),
                    "manages" => relationship_types::MANAGES.to_string(),
                    "configures" => relationship_types::CONFIGURES.to_string(),
                    "depends on" => relationship_types::DEPENDS_ON.to_string(),
                    "validates" => relationship_types::VALIDATES.to_string(),
                    _ => rel_type.clone(),
                };
                
                // Create the relationship
                let mut relationship = Relationship {
                    id: Uuid::new_v4(),
                    source_term_id: term.id,
                    target_term_id,
                    relationship_type,
                    strength: 1.0,
                    sources: vec![source.clone()],
                    metadata: HashMap::new(),
                };
                
                // Add the subtype as metadata
                if !rel_subtype.is_empty() {
                    relationship.metadata.insert("subtype".to_string(), rel_subtype);
                }
                
                relationships.push(relationship);
            }
        }
        // Check for Cypher-style term definitions (for markdown with embedded Cypher)
        else if let Some(caps) = CYPHER_TERM_PATTERN.captures(line) {
            let props_str = &caps[1];
            let mut name = String::new();
            let mut definition = None;
            let mut domain = None;
            let mut metadata = HashMap::new();
            
            // Parse the properties
            for prop in props_str.split(',') {
                let parts: Vec<&str> = prop.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().trim_matches('"');
                    let value = parts[1].trim().trim_matches('"');
                    
                    match key {
                        "name" => name = value.to_string(),
                        "definition" => definition = Some(value.to_string()),
                        "domain" => domain = Some(value.to_string()),
                        _ => {
                            metadata.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
            
            if !name.is_empty() {
                let term_id = Uuid::new_v4();
                term_id_map.insert(name.clone(), term_id);
                
                let mut term = Term {
                    id: term_id,
                    name,
                    definition,
                    domain,
                    sources: vec![source.clone()],
                    metadata,
                };
                
                if !ontology_id.is_empty() {
                    term.metadata.insert("ontology_id".to_string(), ontology_id.to_string());
                }
                
                terms.push(term);
            }
        }
        // Check for Cypher-style relationship definitions
        else if let Some(caps) = CYPHER_RELATIONSHIP_PATTERN.captures(line) {
            let source_term_name = caps[1].trim();
            let relationship_type = caps[2].trim();
            let target_term_name = caps[3].trim();
            
            // Get or create term IDs
            let source_term_id = term_id_map
                .entry(source_term_name.to_string())
                .or_insert_with(Uuid::new_v4)
                .clone();
                
            let target_term_id = term_id_map
                .entry(target_term_name.to_string())
                .or_insert_with(Uuid::new_v4)
                .clone();
            
            // Create the relationship
            let relationship = Relationship {
                id: Uuid::new_v4(),
                source_term_id,
                target_term_id,
                relationship_type: relationship_type.to_string(),
                strength: 1.0,
                sources: vec![source.clone()],
                metadata: HashMap::new(),
            };
            
            relationships.push(relationship);
        }
    }
    
    // Add the last term if there is one
    if let Some(term) = current_term {
        terms.push(term);
    }
    
    // Create terms for any relationships that reference non-existent terms
    for (term_name, term_id) in term_id_map.iter() {
        if !terms.iter().any(|t| &t.id == term_id) {
            let term = Term {
                id: *term_id,
                name: term_name.clone(),
                definition: None,
                domain: None,
                sources: vec![source.clone()],
                metadata: HashMap::new(),
            };
            terms.push(term);
        }
    }
    
    Ok((terms, relationships))
}

/// Extract terms and relationships from a directory of markdown files
///
/// # Arguments
///
/// * `dir_path` - Path to the directory containing markdown files
/// * `ontology_id` - ID of the ontology to associate extracted terms with
///
/// # Returns
///
/// * `Result<(Vec<Term>, Vec<Relationship>)>` - Extracted terms and relationships
pub fn extract_from_directory(dir_path: &Path, ontology_id: &str) -> Result<(Vec<Term>, Vec<Relationship>)> {
    let mut all_terms = Vec::new();
    let mut all_relationships = Vec::new();
    
    let entries = fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read directory: {:?}", dir_path))?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Skip directories
        if path.is_dir() {
            continue;
        }
        
        // Only process markdown files
        if let Some(ext) = path.extension() {
            if ext == "md" || ext == "markdown" {
                let (terms, relationships) = extract_from_markdown(&path, ontology_id)?;
                all_terms.extend(terms);
                all_relationships.extend(relationships);
            }
        }
    }
    
    Ok((all_terms, all_relationships))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_extract_from_markdown() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.md");
        
        let mut file = File::create(&file_path)?;
        writeln!(file, "# Sample Ontology\n")?;
        writeln!(file, "## Database\n")?;
        writeln!(file, "- **definition**: A structured collection of data\n")?;
        writeln!(file, "- **domain**: Computing\n")?;
        writeln!(file, "## Table\n")?;
        writeln!(file, "- **definition**: A data structure that organizes information\n")?;
        writeln!(file, "- **is part of**: [Database](component)\n")?;
        writeln!(file, "## Query\n")?;
        writeln!(file, "- **definition**: A request for data from a database\n")?;
        writeln!(file, "- **related to**: [Database](accesses)\n")?;
        writeln!(file, "- **related to**: [Table](targets)\n")?;
        
        let (terms, relationships) = extract_from_markdown(&file_path, "test-ontology")?;
        
        // Verify terms
        assert_eq!(terms.len(), 3);
        assert!(terms.iter().any(|t| t.name == "Database" && t.definition.as_deref() == Some("A structured collection of data")));
        assert!(terms.iter().any(|t| t.name == "Table" && t.definition.as_deref() == Some("A data structure that organizes information")));
        assert!(terms.iter().any(|t| t.name == "Query" && t.definition.as_deref() == Some("A request for data from a database")));
        
        // Verify relationships
        assert_eq!(relationships.len(), 3);
        
        Ok(())
    }
} 