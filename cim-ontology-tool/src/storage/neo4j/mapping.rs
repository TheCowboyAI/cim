//! Mapping functions for Neo4j nodes to domain entities

use crate::ontology::{Ontology, Relationship, Term};
use crate::storage::{OntologySummary, Result, StorageError};
use neo4rs::Node;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// Convert a Neo4j Node to an Ontology
pub fn node_to_ontology(node: Node) -> Result<Ontology> {
    let id = node
        .get::<String>("id")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get ontology ID: {}", e))
        })
        .and_then(|id_str| {
            Uuid::parse_str(&id_str).map_err(|e| {
                StorageError::SerializationError(format!("Failed to parse UUID: {}", e))
            })
        })?;

    let name = node
        .get::<String>("name")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get ontology name: {}", e))
        })?;

    let description = node.get::<String>("description").ok();

    let metadata = node
        .get::<Value>("metadata")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get ontology metadata: {}", e))
        })
        .and_then(|value| {
            serde_json::from_value::<HashMap<String, String>>(value).map_err(|e| {
                StorageError::SerializationError(format!(
                    "Failed to deserialize ontology metadata: {}",
                    e
                ))
            })
        })?;

    Ok(Ontology {
        id,
        name,
        description,
        terms: Vec::new(),           // Terms will be loaded separately
        relationships: Vec::new(),   // Relationships will be loaded separately
        metadata,
    })
}

/// Convert a Neo4j Node to a Term
pub fn node_to_term(node: Node) -> Result<Term> {
    let id = node
        .get::<String>("id")
        .map_err(|e| StorageError::SerializationError(format!("Failed to get term ID: {}", e)))
        .and_then(|id_str| {
            Uuid::parse_str(&id_str).map_err(|e| {
                StorageError::SerializationError(format!("Failed to parse UUID: {}", e))
            })
        })?;

    let name = node
        .get::<String>("name")
        .map_err(|e| StorageError::SerializationError(format!("Failed to get term name: {}", e)))?;

    let definition = node.get::<String>("definition").ok();
    let domain = node.get::<String>("domain").ok();

    let metadata = node
        .get::<Value>("metadata")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get term metadata: {}", e))
        })
        .and_then(|value| {
            serde_json::from_value::<HashMap<String, String>>(value).map_err(|e| {
                StorageError::SerializationError(format!(
                    "Failed to deserialize term metadata: {}",
                    e
                ))
            })
        })?;

    Ok(Term {
        id,
        name,
        definition,
        domain,
        sources: Vec::new(), // Sources will be loaded separately if needed
        metadata,
    })
}

/// Convert a Neo4j Node to a Relationship
pub fn node_to_relationship(node: Node) -> Result<Relationship> {
    let id = node
        .get::<String>("id")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get relationship ID: {}", e))
        })
        .and_then(|id_str| {
            Uuid::parse_str(&id_str).map_err(|e| {
                StorageError::SerializationError(format!("Failed to parse UUID: {}", e))
            })
        })?;

    let source_term_id = node
        .get::<String>("source_id")
        .map_err(|e| {
            StorageError::SerializationError(format!(
                "Failed to get source term ID: {}",
                e
            ))
        })
        .and_then(|id_str| {
            Uuid::parse_str(&id_str).map_err(|e| {
                StorageError::SerializationError(format!("Failed to parse UUID: {}", e))
            })
        })?;

    let target_term_id = node
        .get::<String>("target_id")
        .map_err(|e| {
            StorageError::SerializationError(format!(
                "Failed to get target term ID: {}",
                e
            ))
        })
        .and_then(|id_str| {
            Uuid::parse_str(&id_str).map_err(|e| {
                StorageError::SerializationError(format!("Failed to parse UUID: {}", e))
            })
        })?;

    let relationship_type = node
        .get::<String>("type")
        .map_err(|e| {
            StorageError::SerializationError(format!(
                "Failed to get relationship type: {}",
                e
            ))
        })?;

    let strength = node
        .get::<f64>("strength")
        .map_err(|e| {
            StorageError::SerializationError(format!(
                "Failed to get relationship strength: {}",
                e
            ))
        })?;

    let metadata = node
        .get::<Value>("metadata")
        .map_err(|e| {
            StorageError::SerializationError(format!(
                "Failed to get relationship metadata: {}",
                e
            ))
        })
        .and_then(|value| {
            serde_json::from_value::<HashMap<String, String>>(value).map_err(|e| {
                StorageError::SerializationError(format!(
                    "Failed to deserialize relationship metadata: {}",
                    e
                ))
            })
        })?;

    Ok(Relationship {
        id,
        source_term_id,
        target_term_id,
        relationship_type,
        strength,
        sources: Vec::new(), // Sources will be loaded separately if needed
        metadata,
    })
}

/// Convert a Neo4j Node to an OntologySummary
pub fn node_to_ontology_summary(node: Node) -> Result<OntologySummary> {
    let id = node
        .get::<String>("id")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get ontology ID: {}", e))
        })
        .and_then(|id_str| {
            Uuid::parse_str(&id_str).map_err(|e| {
                StorageError::SerializationError(format!("Failed to parse UUID: {}", e))
            })
        })?;

    let name = node
        .get::<String>("name")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get ontology name: {}", e))
        })?;

    let description = node.get::<String>("description").ok();

    let term_count = node
        .get::<i64>("termCount")
        .map_err(|e| {
            StorageError::SerializationError(format!("Failed to get term count: {}", e))
        })?
        as usize;

    let relationship_count = node
        .get::<i64>("relationshipCount")
        .map_err(|e| {
            StorageError::SerializationError(format!(
                "Failed to get relationship count: {}",
                e
            ))
        })?
        as usize;

    Ok(OntologySummary {
        id,
        name,
        description,
        term_count,
        relationship_count,
    })
} 