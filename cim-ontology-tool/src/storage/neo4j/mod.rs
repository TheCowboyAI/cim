//! Neo4j storage implementation for ontologies
//!
//! This module provides a Neo4j-based implementation of the OntologyStorage trait.

use crate::ontology::{Ontology, Relationship, Term};
use crate::storage::{OntologyStorage, OntologySummary, Result, StorageError};
use async_trait::async_trait;
use neo4rs::{Graph, Node, Query};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

mod queries;
mod mapping;

use queries::*;

/// Configuration for Neo4j connection
#[derive(Debug, Clone)]
pub struct Neo4jConfig {
    /// URL for the Neo4j database
    pub url: String,
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
    /// Database name (optional)
    pub database: Option<String>,
    /// Connection pool size
    pub pool_size: usize,
}

impl Default for Neo4jConfig {
    fn default() -> Self {
        Self {
            url: "bolt://localhost:7687".to_string(),
            username: "neo4j".to_string(),
            password: "password".to_string(),
            database: None,
            pool_size: 10,
        }
    }
}

/// Neo4j implementation of the OntologyStorage trait
#[derive(Clone)]
pub struct Neo4jStorage {
    /// Neo4j graph connection
    graph: Arc<Mutex<Graph>>,
    /// Configuration
    config: Neo4jConfig,
}

impl Neo4jStorage {
    /// Create a new Neo4jStorage with the given configuration
    pub async fn new(config: Neo4jConfig) -> Result<Self> {
        let graph = Graph::new(&config.url, &config.username, &config.password)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect to Neo4j: {}", e)))?;

        Ok(Self {
            graph: Arc::new(Mutex::new(graph)),
            config,
        })
    }

    /// Execute a Cypher query with parameters
    async fn execute_query<T, F>(&self, query: &str, _params: HashMap<String, serde_json::Value>, mapper: F) -> Result<Vec<T>>
    where
        F: FnMut(Node) -> Result<T> + Send,
    {
        let graph = self.graph.lock().await;
        let mut result = graph
            .execute(Query::new(query.to_string()))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to execute query: {}", e)))?;

        let mut mapped_results = Vec::new();
        let mut mapper = mapper;

        while let Some(row) = result
            .next()
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to fetch result: {}", e)))?
        {
            if let Ok(node) = row.get::<Node>("n") {
                mapped_results.push(mapper(node)?);
            }
        }

        Ok(mapped_results)
    }

    /// Execute a Cypher query that doesn't return results
    async fn execute_command(&self, query: &str, _params: HashMap<String, serde_json::Value>) -> Result<()> {
        let graph = self.graph.lock().await;
        let _ = graph
            .execute(Query::new(query.to_string()))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to execute command: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl OntologyStorage for Neo4jStorage {
    async fn save_ontology(&self, ontology: &Ontology) -> Result<()> {
        // Create or update the ontology node
        let mut params = HashMap::new();
        params.insert("id".to_string(), json!(ontology.id.to_string()));
        params.insert("name".to_string(), json!(ontology.name));
        params.insert(
            "description".to_string(),
            json!(ontology.description.clone().unwrap_or_default()),
        );
        params.insert("metadata".to_string(), json!(ontology.metadata));

        // Using MERGE to create or update
        self.execute_command(CREATE_OR_UPDATE_ONTOLOGY, params).await?;

        // Save all terms
        for term in &ontology.terms {
            self.save_term(&ontology.id, term).await?;
        }

        // Save all relationships
        for relationship in &ontology.relationships {
            self.save_relationship(&ontology.id, relationship).await?;
        }

        Ok(())
    }

    async fn load_ontology(&self, id: &Uuid) -> Result<Ontology> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), json!(id.to_string()));

        // Load the ontology node
        let ontologies = self
            .execute_query(GET_ONTOLOGY_BY_ID, params.clone(), |node| {
                mapping::node_to_ontology(node)
            })
            .await?;

        if ontologies.is_empty() {
            return Err(StorageError::NotFoundError(format!(
                "Ontology with ID {} not found",
                id
            )));
        }

        let mut ontology = ontologies[0].clone();

        // Load all terms for this ontology
        let terms = self
            .execute_query(GET_TERMS_FOR_ONTOLOGY, params.clone(), |node| {
                mapping::node_to_term(node)
            })
            .await?;

        // Load all relationships for this ontology
        let relationships = self
            .execute_query(
                GET_RELATIONSHIPS_FOR_ONTOLOGY,
                params,
                |node| mapping::node_to_relationship(node),
            )
            .await?;

        ontology.terms = terms;
        ontology.relationships = relationships;

        Ok(ontology)
    }

    async fn list_ontologies(&self) -> Result<Vec<OntologySummary>> {
        let ontologies = self
            .execute_query(LIST_ONTOLOGIES, HashMap::new(), |node| {
                mapping::node_to_ontology_summary(node)
            })
            .await?;

        Ok(ontologies)
    }

    async fn delete_ontology(&self, id: &Uuid) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), json!(id.to_string()));

        self.execute_command(DELETE_ONTOLOGY, params).await
    }

    async fn save_term(&self, ontology_id: &Uuid, term: &Term) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("ontology_id".to_string(), json!(ontology_id.to_string()));
        params.insert("term_id".to_string(), json!(term.id.to_string()));
        params.insert("name".to_string(), json!(term.name));
        params.insert(
            "definition".to_string(),
            json!(term.definition.clone().unwrap_or_default()),
        );
        params.insert(
            "domain".to_string(),
            json!(term.domain.clone().unwrap_or_default()),
        );
        params.insert("metadata".to_string(), json!(term.metadata));

        // Create or update the term
        self.execute_command(CREATE_OR_UPDATE_TERM, params).await?;

        // TODO: Handle sources for the term

        Ok(())
    }

    async fn load_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> Result<Term> {
        let mut params = HashMap::new();
        params.insert("ontology_id".to_string(), json!(ontology_id.to_string()));
        params.insert("term_id".to_string(), json!(term_id.to_string()));

        let terms = self
            .execute_query(GET_TERM_BY_ID, params, |node| mapping::node_to_term(node))
            .await?;

        if terms.is_empty() {
            return Err(StorageError::NotFoundError(format!(
                "Term with ID {} not found in ontology {}",
                term_id, ontology_id
            )));
        }

        Ok(terms[0].clone())
    }

    async fn save_relationship(
        &self,
        ontology_id: &Uuid,
        relationship: &Relationship,
    ) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("ontology_id".to_string(), json!(ontology_id.to_string()));
        params.insert(
            "relationship_id".to_string(),
            json!(relationship.id.to_string()),
        );
        params.insert(
            "source_term_id".to_string(),
            json!(relationship.source_term_id.to_string()),
        );
        params.insert(
            "target_term_id".to_string(),
            json!(relationship.target_term_id.to_string()),
        );
        params.insert(
            "relationship_type".to_string(),
            json!(relationship.relationship_type),
        );
        params.insert("strength".to_string(), json!(relationship.strength));
        params.insert("metadata".to_string(), json!(relationship.metadata));

        // Create or update the relationship
        self.execute_command(CREATE_OR_UPDATE_RELATIONSHIP, params).await?;

        // TODO: Handle sources for the relationship

        Ok(())
    }

    async fn load_relationship(
        &self,
        ontology_id: &Uuid,
        relationship_id: &Uuid,
    ) -> Result<Relationship> {
        let mut params = HashMap::new();
        params.insert("ontology_id".to_string(), json!(ontology_id.to_string()));
        params.insert(
            "relationship_id".to_string(),
            json!(relationship_id.to_string()),
        );

        let relationships = self
            .execute_query(GET_RELATIONSHIP_BY_ID, params, |node| {
                mapping::node_to_relationship(node)
            })
            .await?;

        if relationships.is_empty() {
            return Err(StorageError::NotFoundError(format!(
                "Relationship with ID {} not found in ontology {}",
                relationship_id, ontology_id
            )));
        }

        Ok(relationships[0].clone())
    }

    async fn search_terms(&self, name_pattern: &str) -> Result<Vec<Term>> {
        let mut params = HashMap::new();
        params.insert("name_pattern".to_string(), json!(name_pattern));

        let terms = self
            .execute_query(SEARCH_TERMS_BY_NAME, params, |node| {
                mapping::node_to_term(node)
            })
            .await?;

        Ok(terms)
    }

    async fn find_terms_by_domain(&self, domain: &str) -> Result<Vec<Term>> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), json!(domain));

        let terms = self
            .execute_query(FIND_TERMS_BY_DOMAIN, params, |node| {
                mapping::node_to_term(node)
            })
            .await?;

        Ok(terms)
    }

    async fn find_relationships_by_type(
        &self,
        relationship_type: &str,
    ) -> Result<Vec<Relationship>> {
        let mut params = HashMap::new();
        params.insert("relationship_type".to_string(), json!(relationship_type));

        let relationships = self
            .execute_query(FIND_RELATIONSHIPS_BY_TYPE, params, |node| {
                mapping::node_to_relationship(node)
            })
            .await?;

        Ok(relationships)
    }
} 