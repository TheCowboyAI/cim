//! MCP operations for ontology management
//!
//! This module defines the specific operations supported by the MCP server.

use std::path::{Path, PathBuf};
use crate::mcp::handler::{mcp_error, OperationHandler};
use crate::ontology::{Ontology, Relationship, Term};
use crate::storage::OntologyStorage;
use crate::extractor;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Ontology operations
pub struct OntologyOperations;

impl OntologyOperations {
    /// Register all ontology operations with the handler
    pub fn register_all<S: OntologyStorage + 'static>(handler: &mut OperationHandler<S>) {
        // Register operation handlers directly with cloned parameters
        handler
            .register("extractOntology", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::extract_ontology(&params, &storage).await
                }
            })
            .register("extractFromMarkdown", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::extract_from_markdown(&params, &storage).await
                }
            })
            .register("getOntology", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::get_ontology(&params, &storage).await
                }
            })
            .register("listOntologies", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::list_ontologies(&params, &storage).await
                }
            })
            .register("createOntology", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::create_ontology(&params, &storage).await
                }
            })
            .register("updateOntology", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::update_ontology(&params, &storage).await
                }
            })
            .register("deleteOntology", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::delete_ontology(&params, &storage).await
                }
            })
            .register("addTerm", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::add_term(&params, &storage).await
                }
            })
            .register("updateTerm", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::update_term(&params, &storage).await
                }
            })
            .register("getTerm", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::get_term(&params, &storage).await
                }
            })
            .register("searchTerms", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::search_terms(&params, &storage).await
                }
            })
            .register("addRelationship", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::add_relationship(&params, &storage).await
                }
            })
            .register("updateRelationship", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::update_relationship(&params, &storage).await
                }
            })
            .register("getRelationship", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::get_relationship(&params, &storage).await
                }
            })
            .register("compareOntologies", |params, storage| {
                let params = params.clone();
                let storage = Arc::clone(storage);
                async move {
                    Self::compare_ontologies(&params, &storage).await
                }
            });
    }

    /// Extract ontology from files
    async fn extract_ontology(
        params: &HashMap<String, Value>,
        _storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Validate parameters
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'path' parameter"))?;

        // TODO: Implement actual extraction logic
        // For now, return a placeholder response
        Ok(json!({
            "message": format!("Extraction requested for path: {}", path),
            "status": "pending",
        }))
    }

    /// Extract ontology from a markdown file
    async fn extract_from_markdown(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Validate parameters
        let file_path = params
            .get("filePath")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'filePath' parameter"))?;

        let ontology_name = params
            .get("ontologyName")
            .and_then(|v| v.as_str())
            .unwrap_or("Extracted Ontology");

        let ontology_description = params
            .get("ontologyDescription")
            .and_then(|v| v.as_str())
            .unwrap_or("Ontology extracted from markdown file");

        let ontology_domain = params
            .get("ontologyDomain")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        // Create the path
        let path = PathBuf::from(file_path);
        
        // Check if it's a file or directory
        let is_dir = path.is_dir();
        
        // Process the file or directory
        let ontology = if is_dir {
            extractor::create_ontology_from_directory(
                &path,
                ontology_name,
                ontology_description,
                ontology_domain,
            )
            .map_err(|e| {
                mcp_error(
                    "EXTRACTION_ERROR",
                    &format!("Failed to extract ontology from directory: {}", e),
                )
            })?
        } else {
            extractor::create_ontology_from_markdown(
                &path,
                ontology_name,
                ontology_description,
                ontology_domain,
            )
            .map_err(|e| {
                mcp_error(
                    "EXTRACTION_ERROR",
                    &format!("Failed to extract ontology from markdown: {}", e),
                )
            })?
        };
        
        // Save the ontology to storage
        storage.save_ontology(&ontology).await.map_err(|e| {
            mcp_error(
                "STORAGE_ERROR",
                &format!("Failed to save extracted ontology: {}", e),
            )
        })?;
        
        // Return the ontology data with statistics
        Ok(json!({
            "id": ontology.id.to_string(),
            "name": ontology.name,
            "description": ontology.description,
            "termCount": ontology.terms.len(),
            "relationshipCount": ontology.relationships.len(),
            "source": if is_dir { "directory" } else { "file" },
            "sourcePath": file_path,
        }))
    }

    /// Get an ontology by ID
    async fn get_ontology(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse the ontology ID
        let id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let id = Uuid::parse_str(id_str).map_err(|_| {
            mcp_error("INVALID_UUID", &format!("Invalid UUID format: {}", id_str))
        })?;

        // Get the ontology from storage
        let ontology = storage.load_ontology(&id).await.map_err(|e| {
            mcp_error(
                "STORAGE_ERROR",
                &format!("Failed to load ontology: {}", e),
            )
        })?;

        // Convert to JSON
        Ok(serde_json::to_value(&ontology).map_err(|e| {
            mcp_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize ontology: {}", e),
            )
        })?)
    }

    /// List all ontologies
    async fn list_ontologies(
        _params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Get the list of ontologies from storage
        let ontologies = storage.list_ontologies().await.map_err(|e| {
            mcp_error(
                "STORAGE_ERROR",
                &format!("Failed to list ontologies: {}", e),
            )
        })?;

        // Convert to JSON
        Ok(serde_json::to_value(&ontologies).map_err(|e| {
            mcp_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize ontologies: {}", e),
            )
        })?)
    }

    /// Create a new ontology
    async fn create_ontology(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse parameters
        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'name' parameter"))?;

        let description = params.get("description").and_then(|v| v.as_str()).map(String::from);

        // Create the ontology
        let mut ontology = Ontology::new(name.to_string());
        ontology.description = description;

        // Save to storage
        storage.save_ontology(&ontology).await.map_err(|e| {
            mcp_error(
                "STORAGE_ERROR",
                &format!("Failed to save ontology: {}", e),
            )
        })?;

        // Return the created ontology ID
        Ok(json!({
            "id": ontology.id.to_string(),
            "name": ontology.name,
        }))
    }

    /// Update an existing ontology
    async fn update_ontology(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse the ontology ID
        let id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let id = Uuid::parse_str(id_str).map_err(|_| {
            mcp_error("INVALID_UUID", &format!("Invalid UUID format: {}", id_str))
        })?;

        // Get the existing ontology
        let mut ontology = storage.load_ontology(&id).await.map_err(|e| {
            mcp_error(
                "STORAGE_ERROR",
                &format!("Failed to load ontology: {}", e),
            )
        })?;

        // Update fields
        if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
            ontology.name = name.to_string();
        }

        if let Some(description) = params.get("description").and_then(|v| v.as_str()) {
            ontology.description = Some(description.to_string());
        }

        // Save the updated ontology
        storage.save_ontology(&ontology).await.map_err(|e| {
            mcp_error(
                "STORAGE_ERROR",
                &format!("Failed to save ontology: {}", e),
            )
        })?;

        // Return success
        Ok(json!({
            "id": ontology.id.to_string(),
            "name": ontology.name,
            "updated": true,
        }))
    }

    /// Delete an ontology
    async fn delete_ontology(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse the ontology ID
        let id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let id = Uuid::parse_str(id_str).map_err(|_| {
            mcp_error("INVALID_UUID", &format!("Invalid UUID format: {}", id_str))
        })?;

        // Delete the ontology
        storage.delete_ontology(&id).await.map_err(|e| {
            mcp_error(
                "STORAGE_ERROR",
                &format!("Failed to delete ontology: {}", e),
            )
        })?;

        // Return success
        Ok(json!({
            "id": id_str,
            "deleted": true,
        }))
    }

    /// Add a term to an ontology
    async fn add_term(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse the ontology ID
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid ontology UUID format: {}", ontology_id_str),
            )
        })?;

        // Parse term parameters
        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'name' parameter"))?;

        let definition = params.get("definition").and_then(|v| v.as_str()).map(String::from);
        let domain = params.get("domain").and_then(|v| v.as_str()).map(String::from);

        // Create the term
        let mut term = Term::new(name.to_string());
        term.definition = definition;
        term.domain = domain;

        // Save the term
        storage.save_term(&ontology_id, &term).await.map_err(|e| {
            mcp_error("STORAGE_ERROR", &format!("Failed to save term: {}", e))
        })?;

        // Return the created term ID
        Ok(json!({
            "id": term.id.to_string(),
            "name": term.name,
        }))
    }

    /// Update an existing term
    async fn update_term(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse IDs
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid ontology UUID format: {}", ontology_id_str),
            )
        })?;

        let term_id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let term_id = Uuid::parse_str(term_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid term UUID format: {}", term_id_str),
            )
        })?;

        // Get the existing term
        let mut term = storage.load_term(&ontology_id, &term_id).await.map_err(|e| {
            mcp_error("STORAGE_ERROR", &format!("Failed to load term: {}", e))
        })?;

        // Update fields
        if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
            term.name = name.to_string();
        }

        if let Some(definition) = params.get("definition").and_then(|v| v.as_str()) {
            term.definition = Some(definition.to_string());
        }

        if let Some(domain) = params.get("domain").and_then(|v| v.as_str()) {
            term.domain = Some(domain.to_string());
        }

        // Save the updated term
        storage.save_term(&ontology_id, &term).await.map_err(|e| {
            mcp_error("STORAGE_ERROR", &format!("Failed to save term: {}", e))
        })?;

        // Return success
        Ok(json!({
            "id": term.id.to_string(),
            "name": term.name,
            "updated": true,
        }))
    }

    /// Get a term by ID
    async fn get_term(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse IDs
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid ontology UUID format: {}", ontology_id_str),
            )
        })?;

        let term_id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let term_id = Uuid::parse_str(term_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid term UUID format: {}", term_id_str),
            )
        })?;

        // Get the term
        let term = storage.load_term(&ontology_id, &term_id).await.map_err(|e| {
            mcp_error("STORAGE_ERROR", &format!("Failed to load term: {}", e))
        })?;

        // Convert to JSON
        Ok(serde_json::to_value(&term).map_err(|e| {
            mcp_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize term: {}", e),
            )
        })?)
    }

    /// Search for terms by name pattern
    async fn search_terms(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse parameters
        let name_pattern = params
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'pattern' parameter"))?;

        // Search for terms
        let terms = storage.search_terms(name_pattern).await.map_err(|e| {
            mcp_error("STORAGE_ERROR", &format!("Failed to search terms: {}", e))
        })?;

        // Convert to JSON
        Ok(serde_json::to_value(&terms).map_err(|e| {
            mcp_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize terms: {}", e),
            )
        })?)
    }

    /// Add a relationship to an ontology
    async fn add_relationship(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse the ontology ID
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid ontology UUID format: {}", ontology_id_str),
            )
        })?;

        // Parse term IDs
        let source_term_id_str = params
            .get("sourceTermId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                mcp_error(
                    "INVALID_PARAMS",
                    "Missing or invalid 'sourceTermId' parameter",
                )
            })?;

        let source_term_id = Uuid::parse_str(source_term_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid source term UUID format: {}", source_term_id_str),
            )
        })?;

        let target_term_id_str = params
            .get("targetTermId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                mcp_error(
                    "INVALID_PARAMS",
                    "Missing or invalid 'targetTermId' parameter",
                )
            })?;

        let target_term_id = Uuid::parse_str(target_term_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid target term UUID format: {}", target_term_id_str),
            )
        })?;

        // Parse relationship parameters
        let relationship_type = params
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'type' parameter"))?;

        let strength = params
            .get("strength")
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);

        // Create the relationship
        let relationship = Relationship::new(
            source_term_id,
            target_term_id,
            relationship_type.to_string(),
            strength,
        );

        // Save the relationship
        storage
            .save_relationship(&ontology_id, &relationship)
            .await
            .map_err(|e| {
                mcp_error(
                    "STORAGE_ERROR",
                    &format!("Failed to save relationship: {}", e),
                )
            })?;

        // Return the created relationship ID
        Ok(json!({
            "id": relationship.id.to_string(),
            "type": relationship.relationship_type,
        }))
    }

    /// Update an existing relationship
    async fn update_relationship(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse IDs
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid ontology UUID format: {}", ontology_id_str),
            )
        })?;

        let relationship_id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let relationship_id = Uuid::parse_str(relationship_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid relationship UUID format: {}", relationship_id_str),
            )
        })?;

        // Get the existing relationship
        let mut relationship = storage
            .load_relationship(&ontology_id, &relationship_id)
            .await
            .map_err(|e| {
                mcp_error(
                    "STORAGE_ERROR",
                    &format!("Failed to load relationship: {}", e),
                )
            })?;

        // Update fields
        if let Some(relationship_type) = params.get("type").and_then(|v| v.as_str()) {
            relationship.relationship_type = relationship_type.to_string();
        }

        if let Some(strength) = params.get("strength").and_then(|v| v.as_f64()) {
            relationship.strength = strength;
        }

        // Save the updated relationship
        storage
            .save_relationship(&ontology_id, &relationship)
            .await
            .map_err(|e| {
                mcp_error(
                    "STORAGE_ERROR",
                    &format!("Failed to save relationship: {}", e),
                )
            })?;

        // Return success
        Ok(json!({
            "id": relationship.id.to_string(),
            "type": relationship.relationship_type,
            "updated": true,
        }))
    }

    /// Get a relationship by ID
    async fn get_relationship(
        params: &HashMap<String, Value>,
        storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse IDs
        let ontology_id_str = params
            .get("ontologyId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'ontologyId' parameter"))?;

        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid ontology UUID format: {}", ontology_id_str),
            )
        })?;

        let relationship_id_str = params
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'id' parameter"))?;

        let relationship_id = Uuid::parse_str(relationship_id_str).map_err(|_| {
            mcp_error(
                "INVALID_UUID",
                &format!("Invalid relationship UUID format: {}", relationship_id_str),
            )
        })?;

        // Get the relationship
        let relationship = storage
            .load_relationship(&ontology_id, &relationship_id)
            .await
            .map_err(|e| {
                mcp_error(
                    "STORAGE_ERROR",
                    &format!("Failed to load relationship: {}", e),
                )
            })?;

        // Convert to JSON
        Ok(serde_json::to_value(&relationship).map_err(|e| {
            mcp_error(
                "SERIALIZATION_ERROR",
                &format!("Failed to serialize relationship: {}", e),
            )
        })?)
    }

    /// Compare two ontologies
    async fn compare_ontologies(
        params: &HashMap<String, Value>,
        _storage: &Arc<impl OntologyStorage>,
    ) -> Result<Value, crate::mcp::MCPError> {
        // Parse ontology IDs
        let source_id_str = params
            .get("sourceId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'sourceId' parameter"))?;

        let target_id_str = params
            .get("targetId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_error("INVALID_PARAMS", "Missing or invalid 'targetId' parameter"))?;

        // TODO: Implement actual comparison logic
        // For now, return a placeholder response
        Ok(json!({
            "message": format!("Comparison requested between {} and {}", source_id_str, target_id_str),
            "status": "pending",
        }))
    }
} 