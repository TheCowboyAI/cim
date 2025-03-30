use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::Source;

/// A term represents a concept or word in the ontology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    /// Unique identifier for the term
    pub id: Uuid,
    /// Name of the term
    pub name: String,
    /// Optional definition of the term
    pub definition: Option<String>,
    /// Optional domain the term belongs to
    pub domain: Option<String>,
    /// Sources where the term was found
    pub sources: Vec<Source>,
    /// Additional metadata about the term
    pub metadata: HashMap<String, String>,
}

impl Term {
    /// Create a new term with the given name
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            definition: None,
            domain: None,
            sources: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new term with the given name and definition
    pub fn with_definition(name: String, definition: String) -> Self {
        let mut term = Self::new(name);
        term.definition = Some(definition);
        term
    }

    /// Add a source to the term
    pub fn add_source(&mut self, source: Source) {
        self.sources.push(source);
    }

    /// Set the domain for the term
    pub fn with_domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Add metadata to the term
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if the term has a definition
    pub fn has_definition(&self) -> bool {
        self.definition.is_some()
    }

    /// Check if the term belongs to a domain
    pub fn has_domain(&self) -> bool {
        self.domain.is_some()
    }

    /// Get the confidence score for the term, if available
    pub fn confidence_score(&self) -> Option<f64> {
        self.metadata
            .get("confidence_score")
            .and_then(|s| s.parse::<f64>().ok())
    }
} 