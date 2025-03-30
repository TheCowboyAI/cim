use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::Source;

/// A relationship represents a connection between two terms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Unique identifier for the relationship
    pub id: Uuid,
    /// ID of the source term
    pub source_term_id: Uuid,
    /// ID of the target term
    pub target_term_id: Uuid,
    /// Type of the relationship
    pub relationship_type: String,
    /// Confidence score for the relationship (0.0 to 1.0)
    pub strength: f64,
    /// Sources where the relationship was found
    pub sources: Vec<Source>,
    /// Additional metadata about the relationship
    pub metadata: HashMap<String, String>,
}

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

impl Relationship {
    /// Create a new relationship between two terms
    pub fn new(
        source_term_id: Uuid,
        target_term_id: Uuid,
        relationship_type: String,
        strength: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_term_id,
            target_term_id,
            relationship_type,
            strength,
            sources: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a source to the relationship
    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    /// Add metadata to the relationship
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if the relationship is bidirectional
    pub fn is_bidirectional(&self) -> bool {
        self.metadata
            .get("bidirectional")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Set the relationship as bidirectional
    pub fn set_bidirectional(&mut self, value: bool) {
        self.metadata
            .insert("bidirectional".to_string(), value.to_string());
    }

    /// Create an "is-a" relationship
    pub fn is_a(source_term_id: Uuid, target_term_id: Uuid, strength: f64) -> Self {
        Self::new(
            source_term_id,
            target_term_id,
            relationship_types::IS_A.to_string(),
            strength,
        )
    }

    /// Create a "part-of" relationship
    pub fn part_of(source_term_id: Uuid, target_term_id: Uuid, strength: f64) -> Self {
        Self::new(
            source_term_id,
            target_term_id,
            relationship_types::PART_OF.to_string(),
            strength,
        )
    }

    /// Create a "related-to" relationship
    pub fn related_to(source_term_id: Uuid, target_term_id: Uuid, strength: f64) -> Self {
        Self::new(
            source_term_id,
            target_term_id,
            relationship_types::RELATED_TO.to_string(),
            strength,
        )
    }
} 