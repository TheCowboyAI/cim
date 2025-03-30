//! Extractor module for ontology extraction
//!
//! This module provides functionality for extracting ontology information
//! from various sources like codebases, documentation, etc.

pub mod markdown;

use std::path::Path;
use std::collections::HashMap;
use anyhow::{Result, Context};
use uuid::Uuid;
use crate::ontology::{Ontology, Term, Relationship};

/// Extract terms and relationships from a markdown file
///
/// # Arguments
///
/// * `path` - Path to the markdown file
/// * `ontology_id` - ID of the ontology to associate extracted terms with
///
/// # Returns
///
/// * `Result<(Vec<Term>, Vec<Relationship>)>` - Extracted terms and relationships
pub fn extract_from_markdown(path: &Path, ontology_id: &str) -> Result<(Vec<Term>, Vec<Relationship>)> {
    markdown::extract_from_markdown(path, ontology_id)
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
    markdown::extract_from_directory(dir_path, ontology_id)
}

/// Create or update an ontology from a markdown file
///
/// # Arguments
///
/// * `path` - Path to the markdown file
/// * `ontology_name` - Name of the ontology to create or update
/// * `ontology_description` - Description of the ontology
/// * `ontology_domain` - Domain of the ontology
///
/// # Returns
///
/// * `Result<Ontology>` - The created or updated ontology
pub fn create_ontology_from_markdown(
    path: &Path, 
    ontology_name: &str, 
    ontology_description: &str, 
    ontology_domain: &str
) -> Result<Ontology> {
    // Extract terms and relationships from the markdown file
    let (terms, relationships) = extract_from_markdown(path, "")?;
    
    // Create the ontology
    let mut metadata = HashMap::new();
    metadata.insert("domain".to_string(), ontology_domain.to_string());
    metadata.insert("source".to_string(), "markdown".to_string());
    
    let ontology = Ontology {
        id: Uuid::new_v4(),
        name: ontology_name.to_string(),
        description: Some(ontology_description.to_string()),
        terms,
        relationships,
        metadata,
    };
    
    Ok(ontology)
}

/// Create or update an ontology from a directory of markdown files
///
/// # Arguments
///
/// * `dir_path` - Path to the directory containing markdown files
/// * `ontology_name` - Name of the ontology to create or update
/// * `ontology_description` - Description of the ontology
/// * `ontology_domain` - Domain of the ontology
///
/// # Returns
///
/// * `Result<Ontology>` - The created or updated ontology
pub fn create_ontology_from_directory(
    dir_path: &Path, 
    ontology_name: &str, 
    ontology_description: &str, 
    ontology_domain: &str
) -> Result<Ontology> {
    // Extract terms and relationships from the directory
    let (terms, relationships) = extract_from_directory(dir_path, "")?;
    
    // Create the ontology
    let mut metadata = HashMap::new();
    metadata.insert("domain".to_string(), ontology_domain.to_string());
    metadata.insert("source".to_string(), "markdown_directory".to_string());
    
    let ontology = Ontology {
        id: Uuid::new_v4(),
        name: ontology_name.to_string(),
        description: Some(ontology_description.to_string()),
        terms,
        relationships,
        metadata,
    };
    
    Ok(ontology)
}

/// Placeholder function to avoid unused module warning
pub fn extract() -> bool {
    // This is a placeholder and will be implemented in the future
    true
} 