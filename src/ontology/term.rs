//! Term domain model
//!
//! This module defines the Term entity, which represents a concept
//! within an ontology.

use uuid::Uuid;
use crate::source::Source;

/// A term represents a concept within an ontology
#[derive(Debug, Clone)]
pub struct Term {
    /// Unique identifier for the term
    pub id: Uuid,
    /// Name of the term
    pub name: String,
    /// Optional description of the term
    pub description: Option<String>,
    /// Domain to which this term belongs
    pub domain: Option<String>,
    /// Optional definition of the term
    pub definition: Option<String>,
    /// Source from which this term was derived
    pub source: Option<Source>,
}

impl Term {
    /// Create a new term with the given name
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            domain: None,
            definition: None,
            source: None,
        }
    }

    /// Create a new term with the given name and source
    pub fn with_source(name: String, source: Source) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            domain: None,
            definition: None,
            source: Some(source),
        }
    }

    /// Set the term's description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the term's domain
    pub fn with_domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Set the term's definition
    pub fn with_definition(mut self, definition: String) -> Self {
        self.definition = Some(definition);
        self
    }
} 