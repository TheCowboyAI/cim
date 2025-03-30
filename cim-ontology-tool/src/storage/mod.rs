//! Storage module for the ontology tool
//!
//! This module provides interfaces and implementations for storing ontologies.

pub mod neo4j;

use crate::ontology::{Ontology, Relationship, Term};
use async_trait::async_trait;
use uuid::Uuid;

/// Result type for storage operations
pub type Result<T> = std::result::Result<T, StorageError>;

/// Error type for storage operations
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Entity not found: {0}")]
    NotFoundError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

/// Trait for ontology storage providers
#[async_trait]
pub trait OntologyStorage: Send + Sync {
    /// Save an ontology
    async fn save_ontology(&self, ontology: &Ontology) -> Result<()>;

    /// Load an ontology by ID
    async fn load_ontology(&self, id: &Uuid) -> Result<Ontology>;

    /// List all ontologies
    async fn list_ontologies(&self) -> Result<Vec<OntologySummary>>;

    /// Delete an ontology
    async fn delete_ontology(&self, id: &Uuid) -> Result<()>;

    /// Save a term
    async fn save_term(&self, ontology_id: &Uuid, term: &Term) -> Result<()>;

    /// Load a term by ID
    async fn load_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> Result<Term>;

    /// Save a relationship
    async fn save_relationship(
        &self,
        ontology_id: &Uuid,
        relationship: &Relationship,
    ) -> Result<()>;

    /// Load a relationship by ID
    async fn load_relationship(
        &self,
        ontology_id: &Uuid,
        relationship_id: &Uuid,
    ) -> Result<Relationship>;

    /// Search for terms by name pattern
    async fn search_terms(&self, name_pattern: &str) -> Result<Vec<Term>>;

    /// Find terms by domain
    async fn find_terms_by_domain(&self, domain: &str) -> Result<Vec<Term>>;

    /// Find relationships by type
    async fn find_relationships_by_type(
        &self,
        relationship_type: &str,
    ) -> Result<Vec<Relationship>>;
}

/// Summary information about an ontology
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OntologySummary {
    /// Ontology ID
    pub id: Uuid,
    /// Ontology name
    pub name: String,
    /// Ontology description
    pub description: Option<String>,
    /// Number of terms in the ontology
    pub term_count: usize,
    /// Number of relationships in the ontology
    pub relationship_count: usize,
} 