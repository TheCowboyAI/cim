//! Relationship domain model
//!
//! This module defines relationship types and the Relationship entity,
//! which represents connections between terms in an ontology.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::source::Source;
use std::fmt;

/// Type of relationship between terms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// "is a" relationship (hyponym/hypernym)
    IsA,
    /// "has a" relationship (meronym/holonym)
    HasA,
    /// "part of" relationship
    PartOf,
    /// Generic related-to relationship
    RelatedTo,
    /// Custom relationship type with a name
    Custom(String),
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationshipType::IsA => write!(f, "is_a"),
            RelationshipType::HasA => write!(f, "has_a"),
            RelationshipType::PartOf => write!(f, "part_of"),
            RelationshipType::RelatedTo => write!(f, "related_to"),
            RelationshipType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Represents a relationship between two terms in an ontology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Unique identifier for the relationship
    pub id: Uuid,
    /// ID of the source term
    pub source_term_id: Uuid,
    /// ID of the target term
    pub target_term_id: Uuid,
    /// Type of the relationship
    pub relationship_type: RelationshipType,
    /// Optional description of the relationship
    pub description: Option<String>,
    /// Optional source of the relationship
    pub source: Option<Source>,
    /// Additional metadata about the relationship
    pub metadata: HashMap<String, String>,
    /// Weight/strength of the relationship (0.0 to 1.0)
    pub strength: f64,
}

impl Relationship {
    /// Create a new relationship with the given source and target terms and relationship type
    pub fn new(source_term_id: Uuid, target_term_id: Uuid, relationship_type: RelationshipType) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_term_id,
            target_term_id,
            relationship_type,
            description: None,
            source: None,
            metadata: HashMap::new(),
            strength: 1.0,
        }
    }

    /// Create a new relationship with the given source and target terms, relationship type, and source
    pub fn with_source(
        source_term_id: Uuid,
        target_term_id: Uuid,
        relationship_type: RelationshipType,
        source: Source,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_term_id,
            target_term_id,
            relationship_type,
            description: None,
            source: Some(source),
            metadata: HashMap::new(),
            strength: 1.0,
        }
    }

    /// Set the description of the relationship
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add metadata to the relationship
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Create a new relationship with a specified strength
    pub fn with_strength(
        source_term_id: Uuid,
        target_term_id: Uuid,
        relationship_type: RelationshipType,
        strength: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_term_id,
            target_term_id,
            relationship_type,
            description: None,
            source: None,
            metadata: HashMap::new(),
            strength,
        }
    }
} 