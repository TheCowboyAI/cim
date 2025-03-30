//! Ontology domain model
//!
//! This module contains the core domain entities for representing ontologies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

mod term;
mod relationship;
mod source;

pub use term::Term;
pub use relationship::Relationship;
pub use source::Source;

/// An ontology represents a collection of terms and their relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ontology {
    /// Unique identifier for the ontology
    pub id: Uuid,
    /// Name of the ontology
    pub name: String,
    /// Optional description of the ontology
    pub description: Option<String>,
    /// List of terms in the ontology
    pub terms: Vec<Term>,
    /// List of relationships between terms
    pub relationships: Vec<Relationship>,
    /// Additional metadata about the ontology
    pub metadata: HashMap<String, String>,
}

impl Ontology {
    /// Create a new ontology with the given name
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            terms: Vec::new(),
            relationships: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a term to the ontology
    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }

    /// Add a relationship to the ontology
    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    /// Find a term by its ID
    pub fn find_term(&self, id: &Uuid) -> Option<&Term> {
        self.terms.iter().find(|t| &t.id == id)
    }

    /// Find a term by its name
    pub fn find_term_by_name(&self, name: &str) -> Option<&Term> {
        self.terms.iter().find(|t| t.name == name)
    }

    /// Find a relationship by its ID
    pub fn find_relationship(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationships.iter().find(|r| &r.id == id)
    }

    /// Find relationships for a term
    pub fn find_relationships_for_term(&self, term_id: &Uuid) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| &r.source_term_id == term_id || &r.target_term_id == term_id)
            .collect()
    }

    /// Merge another ontology into this one
    pub fn merge(&mut self, other: &Ontology) -> Result<(), String> {
        // Simple merge strategy for now
        for term in &other.terms {
            if self.find_term_by_name(&term.name).is_none() {
                self.add_term(term.clone());
            }
        }

        for relationship in &other.relationships {
            if self.find_relationship(&relationship.id).is_none() {
                self.add_relationship(relationship.clone());
            }
        }

        Ok(())
    }
} 