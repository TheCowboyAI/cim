//! MCP operations for ontology management
//!
//! This module defines the specific operations supported by the MCP server.

use crate::mcp::{MCPError, handler::{OperationHandler, mcp_error}};
use crate::ontology::{Ontology, Term, relationship::RelationshipType};
use crate::source::Source;
use crate::storage::OntologyStorage;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use walkdir;
use regex;
use std::path::Path;
use std::fs;

/// Ontology operations
pub struct OntologyOperations;

impl OntologyOperations {
    /// Register all operations
    pub fn register_all<S: OntologyStorage + 'static>(handler: &mut OperationHandler<S>) {
        handler.register("extract_ontology", Self::extract_ontology::<S>);
        handler.register("get_ontology", Self::get_ontology::<S>);
        handler.register("list_ontologies", Self::list_ontologies::<S>);
        handler.register("create_ontology", Self::create_ontology::<S>);
        handler.register("update_ontology", Self::update_ontology::<S>);
        handler.register("delete_ontology", Self::delete_ontology::<S>);
        handler.register("add_term", Self::add_term::<S>);
        handler.register("update_term", Self::update_term::<S>);
        handler.register("get_term", Self::get_term::<S>);
        handler.register("search_terms", Self::search_terms::<S>);
        handler.register("add_relationship", Self::add_relationship::<S>);
        handler.register("update_relationship", Self::update_relationship::<S>);
        handler.register("get_relationship", Self::get_relationship::<S>);
        handler.register("compare_ontologies", Self::compare_ontologies::<S>);
    }

    /// Check if a file is likely a text file based on its extension
    fn is_text_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            matches!(ext_str.as_str(), 
                "txt" | "md" | "markdown" | "rs" | "py" | "js" | "ts" | "html" | 
                "css" | "json" | "yaml" | "yml" | "toml" | "xml" | "sh" | "bash" | 
                "c" | "cpp" | "h" | "hpp" | "java" | "go" | "rb" | "pl" | "php")
        } else {
            false
        }
    }

    /// Extract an ontology from files
    fn extract_ontology<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Validate parameters
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'path' parameter"))?;

        let _recursive = params
            .get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Extracted Ontology");

        let description = params
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("Ontology extracted from files");

        // Create ontology
        let mut ontology = Ontology::new(name.to_string());
        ontology.description = Some(description.to_string());

        // Extract terms from files
        let path = Path::new(path);

        // Term extraction pattern (simple example: look for capitalized words with at least 3 characters)
        let term_pattern = regex::Regex::new(r"\b[A-Z][a-zA-Z]{2,}\b").unwrap();

        let mut _terms_extracted = 0;
        
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file() && Self::is_text_file(e.path()))
        {
            let path = entry.path();
            println!("Processing file: {}", path.display());
            
            // Read file content
            if let Ok(content) = fs::read_to_string(path) {
                // Extract terms from content
                for capture in term_pattern.captures_iter(&content) {
                    let term_name = capture[0].to_string();
                    
                    // Skip if term already exists
                    if ontology.find_term_by_name(&term_name).is_none() {
                        // Create term with source info
                        let source = Source::from_file(path.to_string_lossy().to_string());
                        let term = Term::with_source(term_name, source);
                        ontology.add_term(term);
                        _terms_extracted += 1;
                    }
                }
            }
        }

        // Placeholder for now
        Ok(json!({
            "status": "queued",
            "message": format!("Extraction from '{}' queued", path.display())
        }))
    }

    /// Get an ontology by ID
    fn get_ontology<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse ontology ID
        let id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let id = Uuid::parse_str(id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid UUID: {}", id_str)))?;

        // Placeholder for now
        Ok(json!({
            "id": id.to_string(),
            "message": "Ontology fetched (placeholder)"
        }))
    }

    /// List all ontologies
    fn list_ontologies<S: OntologyStorage>(
        _params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Placeholder for now
        Ok(json!({
            "ontologies": [],
            "message": "Ontologies listed (placeholder)"
        }))
    }

    /// Create a new ontology
    fn create_ontology<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'name' parameter"))?;

        let _description = params
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Placeholder for now
        Ok(json!({
            "id": Uuid::new_v4().to_string(),
            "name": name,
            "message": "Ontology created (placeholder)"
        }))
    }

    /// Update an ontology
    fn update_ontology<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse ontology ID
        let id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let id = Uuid::parse_str(id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid UUID: {}", id_str)))?;

        // Placeholder for now
        Ok(json!({
            "id": id.to_string(),
            "message": "Ontology updated (placeholder)"
        }))
    }

    /// Delete an ontology
    fn delete_ontology<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse ontology ID
        let id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let id = Uuid::parse_str(id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid UUID: {}", id_str)))?;

        // Placeholder for now
        Ok(json!({
            "id": id.to_string(),
            "message": "Ontology deleted (placeholder)"
        }))
    }

    /// Add a term to an ontology
    fn add_term<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'name' parameter"))?;

        let _definition = params.get("definition").and_then(|v| v.as_str()).map(String::from);
        let _domain = params.get("domain").and_then(|v| v.as_str()).map(String::from);

        let ontology_id = Uuid::parse_str(ontology_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid ontology UUID: {}", ontology_id_str)))?;

        // Placeholder for now
        Ok(json!({
            "id": Uuid::new_v4().to_string(),
            "name": name,
            "ontologyId": ontology_id.to_string(),
            "message": "Term added (placeholder)"
        }))
    }

    /// Update a term
    fn update_term<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let ontology_id_str = params
            .get("ontology_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontology_id' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid ontology UUID: {}", ontology_id_str)))?;

        let term_id_str = params
            .get("term_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'term_id' parameter"))?;

        let term_id = Uuid::parse_str(term_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid term UUID: {}", term_id_str)))?;

        // Placeholder for now
        Ok(json!({
            "id": term_id.to_string(),
            "ontologyId": ontology_id.to_string(),
            "message": "Term updated (placeholder)"
        }))
    }

    /// Get a term
    fn get_term<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let ontology_id_str = params
            .get("ontology_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontology_id' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid ontology UUID: {}", ontology_id_str)))?;

        let term_id_str = params
            .get("term_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'term_id' parameter"))?;

        let term_id = Uuid::parse_str(term_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid term UUID: {}", term_id_str)))?;

        // Placeholder for now
        Ok(json!({
            "id": term_id.to_string(),
            "ontologyId": ontology_id.to_string(),
            "name": "Placeholder term",
            "message": "Term fetched (placeholder)"
        }))
    }

    /// Search for terms
    fn search_terms<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let name_pattern = params
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'pattern' parameter"))?;

        let ontology_id = params.get("ontologyId").and_then(|v| v.as_str());

        // Search for terms
        let _terms: Vec<Value> = Vec::new(); // Placeholder
        
        // For specific ontology
        if let Some(id_str) = ontology_id {
            let _ontology_id = Uuid::parse_str(id_str)
                .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid ontology UUID: {}", id_str)))?;

            // Placeholder logic, removed async call
            // let terms = storage.find_terms_by_name(&ontology_id, name_pattern).await
        } else {
            // Across all ontologies
            // Placeholder logic, removed async call
            // let term_tuples = storage.search_terms(name_pattern).await
        };

        // Placeholder for now
        Ok(json!({
            "pattern": name_pattern,
            "results": [],
            "message": "Terms searched (placeholder)"
        }))
    }

    /// Add a relationship
    fn add_relationship<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let ontology_id_str = params
            .get("ontology_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontology_id' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid ontology UUID: {}", ontology_id_str)))?;

        let source_term_id_str = params
            .get("source_term_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'source_term_id' parameter"))?;

        let source_term_id = Uuid::parse_str(source_term_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid source term UUID: {}", source_term_id_str)))?;

        let target_term_id_str = params
            .get("target_term_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'target_term_id' parameter"))?;

        let target_term_id = Uuid::parse_str(target_term_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid target term UUID: {}", target_term_id_str)))?;

        let relationship_type = params
            .get("relationship_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'relationship_type' parameter"))?;

        let description = params
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Parse relationship type
        let rel_type = match relationship_type.to_lowercase().as_str() {
            "is_a" => RelationshipType::IsA,
            "has_a" => RelationshipType::HasA,
            "part_of" => RelationshipType::PartOf,
            "related_to" => RelationshipType::RelatedTo,
            _ => RelationshipType::Custom(relationship_type.to_string()),
        };

        // Placeholder for now
        Ok(json!({
            "id": Uuid::new_v4().to_string(),
            "source_term_id": source_term_id.to_string(),
            "target_term_id": target_term_id.to_string(),
            "relationship_type": format!("{:?}", rel_type),
            "description": description,
            "ontology_id": ontology_id.to_string(),
            "message": "Relationship added (placeholder)"
        }))
    }

    /// Update a relationship
    fn update_relationship<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let relationship_id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid ontology UUID: {}", ontology_id_str)))?;

        let relationship_id = Uuid::parse_str(relationship_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid relationship UUID: {}", relationship_id_str)))?;

        // Get relationship (removed async code)
        // let mut relationship = storage.get_relationship(&ontology_id, &relationship_id).await
        
        // Update fields
        // (removed async code)
        // storage.save_relationship(&ontology_id, &relationship).await
        
        // Placeholder for now
        Ok(json!({
            "id": relationship_id.to_string(),
            "ontologyId": ontology_id.to_string(),
            "message": "Relationship updated (placeholder)"
        }))
    }

    /// Get a relationship
    fn get_relationship<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse parameters
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let relationship_id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid ontology UUID: {}", ontology_id_str)))?;

        let relationship_id = Uuid::parse_str(relationship_id_str)
            .map_err(|_| mcp_error("INVALID_PARAMS", &format!("Invalid relationship UUID: {}", relationship_id_str)))?;

        // Get relationship (removed async code)
        // let mut relationship = storage.get_relationship(&ontology_id, &relationship_id).await
        
        // Placeholder for now
        Ok(json!({
            "id": relationship_id.to_string(),
            "ontologyId": ontology_id.to_string(),
            "sourceTermId": Uuid::new_v4().to_string(),
            "targetTermId": Uuid::new_v4().to_string(),
            "message": "Relationship fetched (placeholder)"
        }))
    }

    /// Compare two ontologies - making this synchronous
    fn compare_ontologies<S: OntologyStorage>(
        params: &HashMap<String, Value>,
        _storage: &Arc<S>,
    ) -> Result<Value, MCPError> {
        // Parse ontology IDs
        let source_id_str = params
            .get("sourceId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'sourceId' parameter"))?;

        let target_id_str = params
            .get("targetId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'targetId' parameter"))?;

        // Placeholder for now
        Ok(json!({
            "message": format!("Comparison between {} and {}", source_id_str, target_id_str),
            "status": "pending"
        }))
    }
} 