//! Queries module for ontology operations
//!
//! This module provides utilities for querying and manipulating ontology data.

use std::collections::HashMap;
use uuid::Uuid;

use crate::ontology::{Ontology, Relationship, Term};
use crate::storage::{OntologyStorage, StorageError};

/// Search for terms matching a pattern across all ontologies
///
/// Returns a map of ontology IDs to lists of matching terms.
pub async fn search_terms_across_ontologies<S>(
    storage: &S,
    name_pattern: &str,
) -> Result<HashMap<Uuid, Vec<Term>>, StorageError>
where
    S: OntologyStorage + 'static,
{
    let results = storage.search_terms(name_pattern).await?;
    
    let mut grouped_results: HashMap<Uuid, Vec<Term>> = HashMap::new();
    
    for (ontology_id, term) in results {
        grouped_results
            .entry(ontology_id)
            .or_insert_with(Vec::new)
            .push(term);
    }
    
    Ok(grouped_results)
}

/// Find all relationships between two terms in an ontology
pub async fn find_relationships_between_terms<S>(
    storage: &S,
    ontology_id: &Uuid,
    source_term_id: &Uuid,
    target_term_id: &Uuid,
) -> Result<Vec<Relationship>, StorageError>
where
    S: OntologyStorage + 'static,
{
    storage
        .find_relationships_between_terms(ontology_id, source_term_id, target_term_id)
        .await
}

/// Find related terms for a given term based on relationships
///
/// This function calculates related terms by traversing relationships.
pub async fn find_related_terms<S>(
    storage: &S,
    ontology_id: &Uuid,
    term_id: &Uuid,
    relationship_types: Option<Vec<String>>,
    max_depth: Option<usize>,
) -> Result<Vec<Term>, StorageError>
where
    S: OntologyStorage + 'static,
{
    // Get all relationships for the term
    let relationships = storage.find_relationships_by_term(ontology_id, term_id).await?;
    
    // Filter by relationship types if specified
    let filtered_relationships = if let Some(types) = relationship_types {
        relationships
            .into_iter()
            .filter(|r| {
                match &r.relationship_type {
                    crate::ontology::RelationshipType::Custom(name) => types.contains(name),
                    other => {
                        let type_name = match other {
                            crate::ontology::RelationshipType::IsA => "IsA",
                            crate::ontology::RelationshipType::HasA => "HasA",
                            crate::ontology::RelationshipType::PartOf => "PartOf",
                            crate::ontology::RelationshipType::RelatedTo => "RelatedTo",
                            _ => unreachable!(),
                        };
                        types.iter().any(|t| t == type_name)
                    }
                }
            })
            .collect::<Vec<_>>()
    } else {
        relationships
    };
    
    // Get the related term IDs (either source or target, excluding the input term)
    let related_term_ids: Vec<Uuid> = filtered_relationships
        .iter()
        .map(|r| {
            if &r.source_term_id == term_id {
                r.target_term_id
            } else {
                r.source_term_id
            }
        })
        .collect();
    
    // Get the terms for these IDs
    let mut related_terms = Vec::new();
    for id in related_term_ids {
        match storage.get_term(ontology_id, &id).await {
            Ok(term) => related_terms.push(term),
            Err(StorageError::NotFound(_)) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(related_terms)
}

/// Merge two ontologies and save the result as a new ontology
pub async fn merge_ontologies<S>(
    storage: &S,
    ontology1_id: &Uuid,
    ontology2_id: &Uuid,
    new_name: String,
    new_description: Option<String>,
) -> Result<Ontology, StorageError>
where
    S: OntologyStorage + 'static,
{
    // Get the source ontologies
    let ontology1 = storage.get_ontology(ontology1_id).await?;
    let ontology2 = storage.get_ontology(ontology2_id).await?;
    
    // Create a new ontology
    let mut merged = Ontology::new(new_name);
    if let Some(desc) = new_description {
        merged.description = Some(desc);
    } else {
        merged.description = Some(format!(
            "Merged from {} and {}",
            ontology1.name, ontology2.name
        ));
    }
    
    // Add metadata about the source ontologies
    merged.metadata.insert(
        "merged_from".to_string(),
        format!("{},{}", ontology1_id, ontology2_id),
    );
    
    // Merge ontology1 into the new ontology
    merged.merge(&ontology1)?;
    
    // Merge ontology2 into the new ontology
    merged.merge(&ontology2)?;
    
    // Save the merged ontology
    storage.save_ontology(&merged).await?;
    
    Ok(merged)
}

/// Create a new ontology from a JSON definition
///
/// The JSON should have the following structure:
/// ```json
/// {
///   "name": "Ontology Name",
///   "description": "Optional description",
///   "terms": [
///     { "name": "Term1", "description": "Term 1 description" },
///     { "name": "Term2", "description": "Term 2 description" }
///   ],
///   "relationships": [
///     {
///       "source": "Term1",
///       "target": "Term2",
///       "type": "IsA",
///       "description": "Optional relationship description"
///     }
///   ]
/// }
/// ```
pub async fn create_ontology_from_json<S>(
    storage: &S,
    json: &str,
) -> Result<Ontology, StorageError>
where
    S: OntologyStorage + 'static,
{
    // Parse the JSON
    let data: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| StorageError::InvalidOperation(format!("Invalid JSON: {}", e)))?;
    
    // Extract the ontology details
    let name = data["name"]
        .as_str()
        .ok_or_else(|| StorageError::InvalidOperation("Missing ontology name".to_string()))?
        .to_string();
    
    let description = data["description"].as_str().map(|s| s.to_string());
    
    // Create the ontology
    let mut ontology = Ontology::new(name);
    ontology.description = description;
    
    // Add terms
    if let Some(terms_array) = data["terms"].as_array() {
        for term_data in terms_array {
            let term_name = term_data["name"]
                .as_str()
                .ok_or_else(|| StorageError::InvalidOperation("Missing term name".to_string()))?
                .to_string();
            
            let term_description = term_data["description"].as_str().map(|s| s.to_string());
            
            let mut term = Term::new(term_name);
            if let Some(desc) = term_description {
                term.description = Some(desc);
            }
            
            ontology.add_term(term);
        }
    }
    
    // Add relationships
    if let Some(relationships_array) = data["relationships"].as_array() {
        for rel_data in relationships_array {
            let source_name = rel_data["source"]
                .as_str()
                .ok_or_else(|| {
                    StorageError::InvalidOperation("Missing relationship source".to_string())
                })?
                .to_string();
            
            let target_name = rel_data["target"]
                .as_str()
                .ok_or_else(|| {
                    StorageError::InvalidOperation("Missing relationship target".to_string())
                })?
                .to_string();
            
            let rel_type_str = rel_data["type"]
                .as_str()
                .ok_or_else(|| {
                    StorageError::InvalidOperation("Missing relationship type".to_string())
                })?
                .to_string();
            
            let rel_description = rel_data["description"].as_str().map(|s| s.to_string());
            
            // Find the source and target terms
            let source_term = ontology
                .find_term_by_name(&source_name)
                .ok_or_else(|| {
                    StorageError::InvalidOperation(format!("Source term not found: {}", source_name))
                })?;
            
            let target_term = ontology
                .find_term_by_name(&target_name)
                .ok_or_else(|| {
                    StorageError::InvalidOperation(format!("Target term not found: {}", target_name))
                })?;
            
            // Parse the relationship type
            let relationship_type = match rel_type_str.as_str() {
                "IsA" => crate::ontology::RelationshipType::IsA,
                "HasA" => crate::ontology::RelationshipType::HasA,
                "PartOf" => crate::ontology::RelationshipType::PartOf,
                "RelatedTo" => crate::ontology::RelationshipType::RelatedTo,
                _ => crate::ontology::RelationshipType::Custom(rel_type_str),
            };
            
            // Create the relationship
            let mut relationship = Relationship::new(
                source_term.id,
                target_term.id,
                relationship_type,
            );
            
            if let Some(desc) = rel_description {
                relationship.description = Some(desc);
            }
            
            ontology.add_relationship(relationship);
        }
    }
    
    // Save the ontology
    storage.save_ontology(&ontology).await?;
    
    Ok(ontology)
}

/// Export an ontology to JSON format
pub async fn export_ontology_to_json<S>(
    storage: &S,
    ontology_id: &Uuid,
) -> Result<String, StorageError>
where
    S: OntologyStorage + 'static,
{
    // Get the ontology
    let ontology = storage.get_ontology(ontology_id).await?;
    
    // Create the JSON structure
    let mut json_data = serde_json::json!({
        "name": ontology.name,
        "description": ontology.description,
        "id": ontology.id.to_string(),
        "terms": [],
        "relationships": []
    });
    
    // Add terms
    let mut terms_array = Vec::new();
    for term in &ontology.terms {
        let term_json = serde_json::json!({
            "id": term.id.to_string(),
            "name": term.name,
            "description": term.description
        });
        terms_array.push(term_json);
    }
    json_data["terms"] = serde_json::json!(terms_array);
    
    // Add relationships
    let mut relationships_array = Vec::new();
    for rel in &ontology.relationships {
        // Get the source and target term names
        let source_term = ontology.find_term(&rel.source_term_id)
            .ok_or_else(|| StorageError::BackendError(
                format!("Source term not found: {}", rel.source_term_id)
            ))?;
        
        let target_term = ontology.find_term(&rel.target_term_id)
            .ok_or_else(|| StorageError::BackendError(
                format!("Target term not found: {}", rel.target_term_id)
            ))?;
        
        // Convert the relationship type to string
        let rel_type_str = match &rel.relationship_type {
            crate::ontology::RelationshipType::IsA => "IsA".to_string(),
            crate::ontology::RelationshipType::HasA => "HasA".to_string(),
            crate::ontology::RelationshipType::PartOf => "PartOf".to_string(),
            crate::ontology::RelationshipType::RelatedTo => "RelatedTo".to_string(),
            crate::ontology::RelationshipType::Custom(name) => name.clone(),
        };
        
        let rel_json = serde_json::json!({
            "id": rel.id.to_string(),
            "source": source_term.name,
            "source_id": source_term.id.to_string(),
            "target": target_term.name,
            "target_id": target_term.id.to_string(),
            "type": rel_type_str,
            "description": rel.description
        });
        relationships_array.push(rel_json);
    }
    json_data["relationships"] = serde_json::json!(relationships_array);
    
    // Convert to a pretty-printed JSON string
    let json_string = serde_json::to_string_pretty(&json_data)
        .map_err(|e| StorageError::BackendError(format!("Failed to serialize to JSON: {}", e)))?;
    
    Ok(json_string)
} 