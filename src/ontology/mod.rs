//! Ontology domain model
//!
//! This module contains the core domain models for representing ontologies,
//! terms, relationships, and related entities.

mod term;
pub mod relationship;

pub use term::Term;
pub use relationship::{Relationship, RelationshipType};

use uuid::Uuid;
use std::collections::HashMap;

/// An ontology defines a set of terms and their relationships
#[derive(Debug, Clone)]
pub struct Ontology {
    /// Unique identifier for the ontology
    pub id: Uuid,
    /// Name of the ontology
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Terms contained in this ontology
    pub terms: Vec<Term>,
    /// Relationships between terms
    pub relationships: Vec<Relationship>,
    /// Metadata associated with the ontology
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
    pub fn find_term_by_id(&self, id: &Uuid) -> Option<&Term> {
        self.terms.iter().find(|t| t.id == *id)
    }

    /// Find a term by name (case-sensitive)
    pub fn find_term_by_name(&self, name: &str) -> Option<&Term> {
        self.terms.iter().find(|t| t.name == name)
    }

    /// Find a relationship by its ID
    pub fn find_relationship_by_id(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationships.iter().find(|r| r.id == *id)
    }

    /// Find relationships involving a term
    pub fn find_relationships_for_term(&self, term_id: &Uuid) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source_term_id == *term_id || r.target_term_id == *term_id)
            .collect()
    }

    /// Get all terms related to the given term
    pub fn get_related_terms(&self, term_id: &Uuid) -> Vec<(&Term, &Relationship)> {
        let mut result = Vec::new();
        
        for rel in self.find_relationships_for_term(term_id) {
            if rel.source_term_id == *term_id {
                if let Some(target) = self.find_term_by_id(&rel.target_term_id) {
                    result.push((target, rel));
                }
            } else if let Some(source) = self.find_term_by_id(&rel.source_term_id) {
                result.push((source, rel));
            }
        }
        
        result
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
} 