//! Storage interfaces and implementations
//!
//! This module provides the storage layer for persisting ontologies
//! and related entities.

use async_trait::async_trait;
use uuid::Uuid;
use std::fmt;

use crate::ontology::{Ontology, Term, Relationship};

// Commenting out Neo4j until we fix the parameter issues
// pub mod neo4j;

/// Error types for storage operations
#[derive(Debug)]
pub enum StorageError {
    /// Entity not found
    NotFound(String),
    /// Connection error
    ConnectionError(String),
    /// Query execution error
    QueryError(String),
    /// Data serialization error
    SerializationError(String),
    /// General backend error
    BackendError(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::NotFound(msg) => write!(f, "Not found: {}", msg),
            StorageError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            StorageError::QueryError(msg) => write!(f, "Query error: {}", msg),
            StorageError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            StorageError::BackendError(msg) => write!(f, "Backend error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

/// Result type for storage operations
pub type StorageResult<T> = Result<T, StorageError>;

/// Storage interface for ontology persistence
#[async_trait]
pub trait OntologyStorage: Send + Sync {
    /// Initialize the storage backend
    async fn initialize(&self) -> StorageResult<()>;

    /// Save an ontology (create or update)
    async fn save_ontology(&self, ontology: &Ontology) -> StorageResult<()>;

    /// Get an ontology by ID
    async fn get_ontology(&self, id: &Uuid) -> StorageResult<Ontology>;
    
    /// Load an ontology by ID (alias for get_ontology for backward compatibility)
    async fn load_ontology(&self, id: &Uuid) -> StorageResult<Ontology> {
        self.get_ontology(id).await
    }

    /// List all ontologies
    async fn list_ontologies(&self) -> StorageResult<Vec<Ontology>>;

    /// Delete an ontology
    async fn delete_ontology(&self, id: &Uuid) -> StorageResult<()>;

    /// Save a term
    async fn save_term(&self, ontology_id: &Uuid, term: &Term) -> StorageResult<()>;

    /// Get a term by ID
    async fn get_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> StorageResult<Term>;
    
    /// Load a term by ID (alias for get_term for backward compatibility)
    async fn load_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> StorageResult<Term> {
        self.get_term(ontology_id, term_id).await
    }

    /// Delete a term
    async fn delete_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> StorageResult<()>;

    /// Save a relationship
    async fn save_relationship(&self, ontology_id: &Uuid, relationship: &Relationship) -> StorageResult<()>;

    /// Get a relationship by ID
    async fn get_relationship(&self, ontology_id: &Uuid, relationship_id: &Uuid) -> StorageResult<Relationship>;
    
    /// Load a relationship by ID (alias for get_relationship for backward compatibility)
    async fn load_relationship(&self, ontology_id: &Uuid, relationship_id: &Uuid) -> StorageResult<Relationship> {
        self.get_relationship(ontology_id, relationship_id).await
    }

    /// Delete a relationship
    async fn delete_relationship(&self, ontology_id: &Uuid, relationship_id: &Uuid) -> StorageResult<()>;

    /// Find terms by name pattern within an ontology
    async fn find_terms_by_name(&self, ontology_id: &Uuid, name_pattern: &str) -> StorageResult<Vec<Term>>;

    /// Search for terms across all ontologies
    async fn search_terms(&self, name_pattern: &str) -> StorageResult<Vec<(Uuid, Term)>>;

    /// Find relationships by type
    async fn find_relationships_by_type(
        &self,
        ontology_id: &Uuid,
        relationship_type: &str,
    ) -> StorageResult<Vec<Relationship>>;
} 