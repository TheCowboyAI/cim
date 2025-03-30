//! Event handlers for ontology operations
//!
//! This module implements event handlers for ontology-related events.

use super::{Event, EventError, EventHandler, EventType};
use crate::ontology::{Ontology, Relationship, Term};
use crate::storage::OntologyStorage;
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

/// Handler for ontology-related events
pub struct OntologyEventHandler<S: OntologyStorage> {
    /// Name of the handler
    name: String,
    
    /// Storage backend
    storage: Arc<S>,
}

impl<S: OntologyStorage> OntologyEventHandler<S> {
    /// Create a new ontology event handler
    pub fn new(name: &str, storage: Arc<S>) -> Self {
        Self {
            name: name.to_string(),
            storage,
        }
    }
    
    /// Handle creating a new ontology
    async fn handle_ontology_created(&self, event: &Event) -> Result<(), EventError> {
        let payload = &event.payload;
        
        let name = payload["name"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'name' in ontology creation event".to_string())
        )?;
        
        let description = payload["description"].as_str().map(String::from);
        
        // Create the ontology
        let mut ontology = Ontology::new(name.to_string());
        ontology.description = description;
        
        // Save to storage
        self.storage.save_ontology(&ontology).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to save ontology: {}", e))
        )?;
        
        println!("Created ontology: {} (ID: {})", name, ontology.id);
        
        Ok(())
    }
    
    /// Handle updating an ontology
    async fn handle_ontology_updated(&self, event: &Event) -> Result<(), EventError> {
        let payload = &event.payload;
        
        let id_str = payload["id"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'id' in ontology update event".to_string())
        )?;
        
        let id = Uuid::parse_str(id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid UUID format: {}", id_str))
        )?;
        
        // Get the existing ontology
        let mut ontology = self.storage.load_ontology(&id).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to load ontology: {}", e))
        )?;
        
        // Update fields
        if let Some(name) = payload["name"].as_str() {
            ontology.name = name.to_string();
        }
        
        if let Some(description) = payload["description"].as_str() {
            ontology.description = Some(description.to_string());
        }
        
        // Save the updated ontology
        self.storage.save_ontology(&ontology).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to save ontology: {}", e))
        )?;
        
        println!("Updated ontology: {} (ID: {})", ontology.name, ontology.id);
        
        Ok(())
    }
    
    /// Handle deleting an ontology
    async fn handle_ontology_deleted(&self, event: &Event) -> Result<(), EventError> {
        let payload = &event.payload;
        
        let id_str = payload["id"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'id' in ontology deletion event".to_string())
        )?;
        
        let id = Uuid::parse_str(id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid UUID format: {}", id_str))
        )?;
        
        // Delete the ontology
        self.storage.delete_ontology(&id).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to delete ontology: {}", e))
        )?;
        
        println!("Deleted ontology with ID: {}", id);
        
        Ok(())
    }
    
    /// Handle adding a term to an ontology
    async fn handle_term_added(&self, event: &Event) -> Result<(), EventError> {
        let payload = &event.payload;
        
        let ontology_id_str = payload["ontologyId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'ontologyId' in term addition event".to_string())
        )?;
        
        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid ontology UUID format: {}", ontology_id_str))
        )?;
        
        let name = payload["name"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'name' in term addition event".to_string())
        )?;
        
        let definition = payload["definition"].as_str().map(String::from);
        let domain = payload["domain"].as_str().map(String::from);
        
        // Create the term
        let mut term = Term::new(name.to_string());
        term.definition = definition;
        term.domain = domain;
        
        // Save the term
        self.storage.save_term(&ontology_id, &term).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to save term: {}", e))
        )?;
        
        println!("Added term: {} (ID: {}) to ontology {}", name, term.id, ontology_id);
        
        Ok(())
    }
    
    /// Handle updating a term
    async fn handle_term_updated(&self, event: &Event) -> Result<(), EventError> {
        let payload = &event.payload;
        
        let ontology_id_str = payload["ontologyId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'ontologyId' in term update event".to_string())
        )?;
        
        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid ontology UUID format: {}", ontology_id_str))
        )?;
        
        let term_id_str = payload["id"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'id' in term update event".to_string())
        )?;
        
        let term_id = Uuid::parse_str(term_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid term UUID format: {}", term_id_str))
        )?;
        
        // Get the existing term
        let mut term = self.storage.load_term(&ontology_id, &term_id).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to load term: {}", e))
        )?;
        
        // Update fields
        if let Some(name) = payload["name"].as_str() {
            term.name = name.to_string();
        }
        
        if let Some(definition) = payload["definition"].as_str() {
            term.definition = Some(definition.to_string());
        }
        
        if let Some(domain) = payload["domain"].as_str() {
            term.domain = Some(domain.to_string());
        }
        
        // Save the updated term
        self.storage.save_term(&ontology_id, &term).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to save term: {}", e))
        )?;
        
        println!("Updated term: {} (ID: {}) in ontology {}", term.name, term.id, ontology_id);
        
        Ok(())
    }
    
    /// Handle adding a relationship to an ontology
    async fn handle_relationship_added(&self, event: &Event) -> Result<(), EventError> {
        let payload = &event.payload;
        
        let ontology_id_str = payload["ontologyId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'ontologyId' in relationship addition event".to_string())
        )?;
        
        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid ontology UUID format: {}", ontology_id_str))
        )?;
        
        let source_term_id_str = payload["sourceTermId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'sourceTermId' in relationship addition event".to_string())
        )?;
        
        let source_term_id = Uuid::parse_str(source_term_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid source term UUID format: {}", source_term_id_str))
        )?;
        
        let target_term_id_str = payload["targetTermId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'targetTermId' in relationship addition event".to_string())
        )?;
        
        let target_term_id = Uuid::parse_str(target_term_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid target term UUID format: {}", target_term_id_str))
        )?;
        
        let relationship_type = payload["type"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'type' in relationship addition event".to_string())
        )?;
        
        let strength = payload["strength"].as_f64().unwrap_or(1.0);
        
        // Create the relationship
        let relationship = Relationship::new(
            source_term_id,
            target_term_id,
            relationship_type.to_string(),
            strength,
        );
        
        // Save the relationship
        self.storage.save_relationship(&ontology_id, &relationship).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to save relationship: {}", e))
        )?;
        
        println!("Added relationship of type '{}' (ID: {}) to ontology {}", 
            relationship_type, relationship.id, ontology_id);
        
        Ok(())
    }
    
    /// Handle updating a relationship
    async fn handle_relationship_updated(&self, event: &Event) -> Result<(), EventError> {
        let payload = &event.payload;
        
        let ontology_id_str = payload["ontologyId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'ontologyId' in relationship update event".to_string())
        )?;
        
        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid ontology UUID format: {}", ontology_id_str))
        )?;
        
        let relationship_id_str = payload["id"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'id' in relationship update event".to_string())
        )?;
        
        let relationship_id = Uuid::parse_str(relationship_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid relationship UUID format: {}", relationship_id_str))
        )?;
        
        // Get the existing relationship
        let mut relationship = self.storage.load_relationship(&ontology_id, &relationship_id).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to load relationship: {}", e))
        )?;
        
        // Update fields
        if let Some(relationship_type) = payload["type"].as_str() {
            relationship.relationship_type = relationship_type.to_string();
        }
        
        if let Some(strength) = payload["strength"].as_f64() {
            relationship.strength = strength;
        }
        
        // Save the updated relationship
        self.storage.save_relationship(&ontology_id, &relationship).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to save relationship: {}", e))
        )?;
        
        println!("Updated relationship of type '{}' (ID: {}) in ontology {}", 
            relationship.relationship_type, relationship.id, ontology_id);
        
        Ok(())
    }
}

#[async_trait]
impl<S: OntologyStorage + 'static> EventHandler for OntologyEventHandler<S> {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn event_types(&self) -> Vec<EventType> {
        vec![
            EventType::OntologyCreated,
            EventType::OntologyUpdated,
            EventType::OntologyDeleted,
            EventType::TermAdded,
            EventType::TermUpdated,
            EventType::RelationshipAdded,
            EventType::RelationshipUpdated,
        ]
    }
    
    async fn handle_event(&self, event: Event) -> Result<(), EventError> {
        match event.event_type {
            EventType::OntologyCreated => self.handle_ontology_created(&event).await,
            EventType::OntologyUpdated => self.handle_ontology_updated(&event).await,
            EventType::OntologyDeleted => self.handle_ontology_deleted(&event).await,
            EventType::TermAdded => self.handle_term_added(&event).await,
            EventType::TermUpdated => self.handle_term_updated(&event).await,
            EventType::RelationshipAdded => self.handle_relationship_added(&event).await,
            EventType::RelationshipUpdated => self.handle_relationship_updated(&event).await,
            _ => Ok(()),
        }
    }
}

/// Handler for query-related events
pub struct QueryEventHandler<S: OntologyStorage> {
    /// Name of the handler
    name: String,
    
    /// Storage backend
    storage: Arc<S>,
}

impl<S: OntologyStorage> QueryEventHandler<S> {
    /// Create a new query event handler
    pub fn new(name: &str, storage: Arc<S>) -> Self {
        Self {
            name: name.to_string(),
            storage,
        }
    }
    
    /// Handle querying an ontology
    async fn handle_ontology_query(&self, event: &Event) -> Result<Ontology, EventError> {
        let payload = &event.payload;
        
        let id_str = payload["id"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'id' in ontology query event".to_string())
        )?;
        
        let id = Uuid::parse_str(id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid UUID format: {}", id_str))
        )?;
        
        // Get the ontology from storage
        let ontology = self.storage.load_ontology(&id).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to load ontology: {}", e))
        )?;
        
        Ok(ontology)
    }
    
    /// Handle listing all ontologies
    async fn handle_ontologies_list(&self, _event: &Event) -> Result<Vec<crate::storage::OntologySummary>, EventError> {
        // Get all ontologies from storage
        let ontologies = self.storage.list_ontologies().await.map_err(|e| 
            EventError::HandlerError(format!("Failed to list ontologies: {}", e))
        )?;
        
        Ok(ontologies)
    }
    
    /// Handle getting a term
    async fn handle_term_query(&self, event: &Event) -> Result<Term, EventError> {
        let payload = &event.payload;
        
        let ontology_id_str = payload["ontologyId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'ontologyId' in term query event".to_string())
        )?;
        
        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid ontology UUID format: {}", ontology_id_str))
        )?;
        
        let term_id_str = payload["id"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'id' in term query event".to_string())
        )?;
        
        let term_id = Uuid::parse_str(term_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid term UUID format: {}", term_id_str))
        )?;
        
        // Get the term from storage
        let term = self.storage.load_term(&ontology_id, &term_id).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to load term: {}", e))
        )?;
        
        Ok(term)
    }
    
    /// Handle searching for terms
    async fn handle_terms_search(&self, event: &Event) -> Result<Vec<Term>, EventError> {
        let payload = &event.payload;
        
        let pattern = payload["pattern"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'pattern' in terms search event".to_string())
        )?;
        
        // Search for terms
        let terms = self.storage.search_terms(pattern).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to search terms: {}", e))
        )?;
        
        Ok(terms)
    }
    
    /// Handle getting a relationship
    async fn handle_relationship_query(&self, event: &Event) -> Result<Relationship, EventError> {
        let payload = &event.payload;
        
        let ontology_id_str = payload["ontologyId"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'ontologyId' in relationship query event".to_string())
        )?;
        
        let ontology_id = Uuid::parse_str(ontology_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid ontology UUID format: {}", ontology_id_str))
        )?;
        
        let relationship_id_str = payload["id"].as_str().ok_or_else(|| 
            EventError::HandlerError("Missing 'id' in relationship query event".to_string())
        )?;
        
        let relationship_id = Uuid::parse_str(relationship_id_str).map_err(|_| 
            EventError::HandlerError(format!("Invalid relationship UUID format: {}", relationship_id_str))
        )?;
        
        // Get the relationship from storage
        let relationship = self.storage.load_relationship(&ontology_id, &relationship_id).await.map_err(|e| 
            EventError::HandlerError(format!("Failed to load relationship: {}", e))
        )?;
        
        Ok(relationship)
    }
}

#[async_trait]
impl<S: OntologyStorage + 'static> EventHandler for QueryEventHandler<S> {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn event_types(&self) -> Vec<EventType> {
        vec![
            EventType::OntologiesListed,
            EventType::TermsSearched,
        ]
    }
    
    async fn handle_event(&self, event: Event) -> Result<(), EventError> {
        match event.event_type {
            EventType::OntologiesListed => {
                match self.handle_ontologies_list(&event).await {
                    Ok(ontologies) => {
                        println!("Listed {} ontologies", ontologies.len());
                    },
                    Err(e) => return Err(e),
                }
                Ok(())
            },
            EventType::TermsSearched => {
                match self.handle_terms_search(&event).await {
                    Ok(terms) => {
                        println!("Found {} terms matching search", terms.len());
                    },
                    Err(e) => return Err(e),
                }
                Ok(())
            },
            _ => Ok(()),
        }
    }
} 