//! Neo4j storage implementation for ontology persistence
//!
//! This module provides an implementation of the OntologyStorage trait using Neo4j.

use async_trait::async_trait;
use neo4rs::{query, Graph, Node};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::ontology::{Ontology, Relationship, RelationshipType, Term};
use crate::source::Source;
use crate::storage::{OntologyStorage, StorageError, StorageResult};

/// Neo4j connection configuration
#[derive(Debug, Clone)]
pub struct Neo4jConfig {
    /// URI for the Neo4j server
    pub uri: String,
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
    /// Database name (optional)
    pub database: Option<String>,
}

impl Neo4jConfig {
    /// Create a new Neo4j configuration
    pub fn new(uri: String, username: String, password: String) -> Self {
        Self {
            uri,
            username,
            password,
            database: None,
        }
    }

    /// Set the database name
    pub fn with_database(mut self, database: String) -> Self {
        self.database = Some(database);
        self
    }
}

/// Neo4j storage implementation
pub struct Neo4jStorage {
    /// Neo4j connection
    connection: Arc<Mutex<Option<Graph>>>,
    /// Configuration
    config: Neo4jConfig,
}

impl Neo4jStorage {
    /// Create a new Neo4j storage instance
    pub fn new(config: Neo4jConfig) -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
            config,
        }
    }

    /// Get a connection to the Neo4j database
    async fn get_connection(&self) -> StorageResult<Arc<Graph>> {
        let mut conn_guard = self.connection.lock().await;

        if conn_guard.is_none() {
            // Connect to Neo4j
            let graph = neo4rs::Graph::new(&self.config.uri, &self.config.username, &self.config.password)
                .await
                .map_err(|e| StorageError::ConnectionError(format!("Failed to connect to Neo4j: {}", e)))?;

            *conn_guard = Some(graph);
        }

        // Clone the connection for use in this function
        let conn = Arc::new(conn_guard.as_ref().unwrap().clone());
        Ok(conn)
    }

    /// Convert a Neo4j Node to a Term
    async fn node_to_term(&self, node: Node) -> Result<Term, StorageError> {
        let id = node.get::<String>("id")
            .map_err(|e| StorageError::BackendError(format!("Failed to get term ID: {}", e)))?;
        let id = Uuid::parse_str(&id)
            .map_err(|e| StorageError::BackendError(format!("Invalid term ID: {}", e)))?;

        let name = node.get::<String>("name")
            .map_err(|e| StorageError::BackendError(format!("Failed to get term name: {}", e)))?;

        let description = node.get::<String>("description").ok();

        // Create the term
        let mut term = Term::new(name);
        term.id = id;
        if let Some(desc) = description {
            term.description = Some(desc);
        }

        Ok(term)
    }

    /// Convert a Neo4j Relation to a Relationship
    async fn rel_to_relationship(&self, 
                            rel: neo4rs::Relation, 
                            ontology_id: &Uuid) -> Result<Relationship, StorageError> {
        let id = rel.get::<String>("id")
            .map_err(|e| StorageError::BackendError(format!("Failed to get relationship ID: {}", e)))?;
        let id = Uuid::parse_str(&id)
            .map_err(|e| StorageError::BackendError(format!("Invalid relationship ID: {}", e)))?;

        let source_term_id = rel.get::<String>("source_term_id")
            .map_err(|e| StorageError::BackendError(format!("Failed to get source term ID: {}", e)))?;
        let source_term_id = Uuid::parse_str(&source_term_id)
            .map_err(|e| StorageError::BackendError(format!("Invalid source term ID: {}", e)))?;

        let target_term_id = rel.get::<String>("target_term_id")
            .map_err(|e| StorageError::BackendError(format!("Failed to get target term ID: {}", e)))?;
        let target_term_id = Uuid::parse_str(&target_term_id)
            .map_err(|e| StorageError::BackendError(format!("Invalid target term ID: {}", e)))?;

        let rel_type = rel.get::<String>("relationship_type")
            .map_err(|e| StorageError::BackendError(format!("Failed to get relationship type: {}", e)))?;

        // Parse the relationship type
        let relationship_type = match rel_type.as_str() {
            "IS_A" => RelationshipType::IsA,
            "HAS_A" => RelationshipType::HasA,
            "PART_OF" => RelationshipType::PartOf,
            "RELATED_TO" => RelationshipType::RelatedTo,
            _ => RelationshipType::Custom(rel_type),
        };

        let description = rel.get::<String>("description").ok();

        // Create the relationship
        let mut relationship = Relationship::new(source_term_id, target_term_id, relationship_type);
        relationship.id = id;
        if let Some(desc) = description {
            relationship.description = Some(desc);
        }

        Ok(relationship)
    }

    /// Get a string representation of a RelationshipType
    fn get_rel_type_name(rel_type: &RelationshipType) -> String {
        match rel_type {
            RelationshipType::IsA => "IS_A".to_string(),
            RelationshipType::HasA => "HAS_A".to_string(),
            RelationshipType::PartOf => "PART_OF".to_string(),
            RelationshipType::RelatedTo => "RELATED_TO".to_string(),
            RelationshipType::Custom(name) => name.clone(),
        }
    }

    // Helper function to create parameter HashMap
    fn create_params<T: Into<Value>>(params: Vec<(&str, T)>) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        for (key, value) in params {
            map.insert(key.to_string(), value.into());
        }
        map
    }
}

#[async_trait]
impl OntologyStorage for Neo4jStorage {
    /// Initialize the storage backend
    async fn initialize(&self) -> StorageResult<()> {
        let conn = self.get_connection().await?;

        // Create constraints
        let create_constraints = r#"
            CREATE CONSTRAINT ontology_id IF NOT EXISTS FOR (o:Ontology) REQUIRE o.id IS UNIQUE;
            CREATE CONSTRAINT term_id IF NOT EXISTS FOR (t:Term) REQUIRE t.id IS UNIQUE;
            CREATE CONSTRAINT relationship_id IF NOT EXISTS FOR (r:OntologyRelationship) REQUIRE r.id IS UNIQUE;
        "#;

        conn.run(query(create_constraints))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create constraints: {}", e)))?;

        Ok(())
    }

    /// Save an ontology
    async fn save_ontology(&self, ontology: &Ontology) -> StorageResult<()> {
        let conn = self.get_connection().await?;

        // Create or update the ontology node
        let create_ontology = r#"
            MERGE (o:Ontology {id: $id})
            ON CREATE SET o.created_at = datetime()
            SET o.name = $name,
                o.description = $description,
                o.updated_at = datetime()
            RETURN o
        "#;

        let mut params = HashMap::new();
        params.insert("id", neo4rs::Node::from(ontology.id.to_string()));
        params.insert("name", neo4rs::Node::from(ontology.name.clone()));
        params.insert("description", neo4rs::Node::from(ontology.description.clone().unwrap_or_default()));

        conn.run(query(create_ontology).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to save ontology: {}", e)))?;

        // Save terms
        for term in &ontology.terms {
            self.save_term(&ontology.id, term).await?;
        }

        // Save relationships
        for relationship in &ontology.relationships {
            self.save_relationship(&ontology.id, relationship).await?;
        }

        Ok(())
    }

    /// Get an ontology by ID
    async fn get_ontology(&self, id: &Uuid) -> StorageResult<Ontology> {
        let conn = self.get_connection().await?;

        // Get the ontology node
        let get_ontology = r#"
            MATCH (o:Ontology {id: $id})
            RETURN o.name as name, o.description as description
        "#;

        let mut params = HashMap::new();
        params.insert("id", neo4rs::Node::from(id.to_string()));

        let mut result = conn.execute(query(get_ontology).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to get ontology: {}", e)))?;

        if let Some(row) = result.next().await {
            let name: String = row.get("name").map_err(|e| StorageError::BackendError(format!("Failed to get name: {}", e)))?;
            let description: Option<String> = row.get("description").ok();

            let mut ontology = Ontology::new(name);
            ontology.id = *id;
            ontology.description = description;

            // Get terms
            let get_terms = r#"
                MATCH (o:Ontology {id: $id})-[:CONTAINS]->(t:Term)
                RETURN t
            "#;

            let mut params = HashMap::new();
            params.insert("id", neo4rs::Node::from(id.to_string()));

            let mut term_result = conn.execute(query(get_terms).params(params))
                .await
                .map_err(|e| StorageError::QueryError(format!("Failed to get terms: {}", e)))?;

            while let Some(row) = term_result.next().await {
                let term_node: neo4rs::Node = row.get("t").map_err(|e| StorageError::BackendError(format!("Failed to get term node: {}", e)))?;
                let term_id: String = term_node.get("id").map_err(|e| StorageError::BackendError(format!("Failed to get term id: {}", e)))?;
                let term_id = Uuid::parse_str(&term_id).map_err(|e| StorageError::BackendError(format!("Invalid term UUID: {}", e)))?;
                
                // Get full term
                let term = self.get_term(id, &term_id).await?;
                ontology.add_term(term);
            }

            // Get relationships
            let get_relationships = r#"
                MATCH (o:Ontology {id: $id})-[:CONTAINS]->(r:OntologyRelationship)
                RETURN r
            "#;

            let mut params = HashMap::new();
            params.insert("id", neo4rs::Node::from(id.to_string()));

            let mut rel_result = conn.execute(query(get_relationships).params(params))
                .await
                .map_err(|e| StorageError::QueryError(format!("Failed to get relationships: {}", e)))?;

            while let Some(row) = rel_result.next().await {
                let rel_node: neo4rs::Node = row.get("r").map_err(|e| StorageError::BackendError(format!("Failed to get relationship node: {}", e)))?;
                let rel_id: String = rel_node.get("id").map_err(|e| StorageError::BackendError(format!("Failed to get relationship id: {}", e)))?;
                let rel_id = Uuid::parse_str(&rel_id).map_err(|e| StorageError::BackendError(format!("Invalid relationship UUID: {}", e)))?;
                
                // Get full relationship
                let relationship = self.get_relationship(id, &rel_id).await?;
                ontology.add_relationship(relationship);
            }

            Ok(ontology)
        } else {
            Err(StorageError::NotFound(format!("Ontology not found: {}", id)))
        }
    }

    /// Get an ontology by name
    async fn get_ontology_by_name(&self, name: &str) -> Result<Ontology, StorageError> {
        let query = "
            MATCH (o:Ontology)
            WHERE o.name = $name
            RETURN o
        ";

        let params = Self::create_params(vec![
            ("name", name),
        ]);

        let mut result = self.get_connection().await?.execute(query::new(query).params(params)).await
            .map_err(|e| StorageError::BackendError(format!("Failed to get ontology by name: {}", e)))?;

        if let Some(row) = result.next().await {
            let node = row.get::<Node>("o")
                .map_err(|e| StorageError::BackendError(format!("Failed to get ontology node: {}", e)))?;

            let id = node.get::<String>("id")
                .map_err(|e| StorageError::BackendError(format!("Failed to get ontology ID: {}", e)))?;
            let id = Uuid::parse_str(&id)
                .map_err(|e| StorageError::BackendError(format!("Invalid ontology ID: {}", e)))?;

            // Get the full ontology by ID
            self.get_ontology(&id).await
        } else {
            Err(StorageError::NotFound(format!("Ontology not found by name: {}", name)))
        }
    }

    /// List all ontologies
    async fn list_ontologies(&self) -> Result<Vec<Ontology>, StorageError> {
        let conn = self.get_connection().await?;

        // Get all ontology IDs
        let get_ontologies = r#"
            MATCH (o:Ontology)
            RETURN o.id as id
        "#;

        let mut result = conn.execute(query(get_ontologies))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to list ontologies: {}", e)))?;

        let mut ontologies = Vec::new();
        while let Some(row) = result.next().await {
            let id: String = row.get("id").map_err(|e| StorageError::BackendError(format!("Failed to get id: {}", e)))?;
            let id = Uuid::parse_str(&id).map_err(|e| StorageError::BackendError(format!("Invalid UUID: {}", e)))?;
            
            // Get the full ontology
            match self.get_ontology(&id).await {
                Ok(ontology) => ontologies.push(ontology),
                Err(e) => eprintln!("Error getting ontology {}: {}", id, e),
            }
        }

        Ok(ontologies)
    }

    /// Delete an ontology
    async fn delete_ontology(&self, id: &Uuid) -> StorageResult<()> {
        let conn = self.get_connection().await?;

        // Delete the ontology and all its terms and relationships
        let delete_ontology = r#"
            MATCH (o:Ontology {id: $id})
            OPTIONAL MATCH (o)-[:CONTAINS]->(t:Term)
            OPTIONAL MATCH (o)-[:CONTAINS]->(r:OntologyRelationship)
            DETACH DELETE o, t, r
        "#;

        let mut params = HashMap::new();
        params.insert("id", neo4rs::Node::from(id.to_string()));

        conn.run(query(delete_ontology).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete ontology: {}", e)))?;

        Ok(())
    }

    /// Save a term to an ontology
    async fn save_term(&self, ontology_id: &Uuid, term: &Term) -> StorageResult<()> {
        let conn = self.get_connection().await?;

        // Create or update the term node
        let create_term = r#"
            MATCH (o:Ontology {id: $ontology_id})
            MERGE (t:Term {id: $id})
            ON CREATE SET t.created_at = datetime()
            SET t.name = $name,
                t.description = $description,
                t.domain = $domain,
                t.definition = $definition,
                t.updated_at = datetime()
            MERGE (o)-[:CONTAINS]->(t)
            RETURN t
        "#;

        let mut params = HashMap::new();
        params.insert("ontology_id", neo4rs::Node::from(ontology_id.to_string()));
        params.insert("id", neo4rs::Node::from(term.id.to_string()));
        params.insert("name", neo4rs::Node::from(term.name.clone()));
        params.insert("description", neo4rs::Node::from(term.description.clone().unwrap_or_default()));
        params.insert("domain", neo4rs::Node::from(term.domain.clone().unwrap_or_default()));
        params.insert("definition", neo4rs::Node::from(term.definition.clone().unwrap_or_default()));

        conn.run(query(create_term).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to save term: {}", e)))?;

        Ok(())
    }

    /// Get a term by ID
    async fn get_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> StorageResult<Term> {
        let conn = self.get_connection().await?;

        // Get the term node
        let get_term = r#"
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(t:Term {id: $term_id})
            RETURN t.name as name, t.description as description, 
                   t.domain as domain, t.definition as definition
        "#;

        let mut params = HashMap::new();
        params.insert("ontology_id", neo4rs::Node::from(ontology_id.to_string()));
        params.insert("term_id", neo4rs::Node::from(term_id.to_string()));

        let mut result = conn.execute(query(get_term).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to get term: {}", e)))?;

        if let Some(row) = result.next().await {
            let name: String = row.get("name").map_err(|e| StorageError::BackendError(format!("Failed to get name: {}", e)))?;
            let description: Option<String> = row.get("description").ok();
            let domain: Option<String> = row.get("domain").ok();
            let definition: Option<String> = row.get("definition").ok();

            let mut term = Term::new(name);
            term.id = *term_id;
            term.description = description;
            term.domain = domain;
            term.definition = definition;

            Ok(term)
        } else {
            Err(StorageError::NotFound(format!("Term not found: {}", term_id)))
        }
    }

    /// List terms for an ontology
    async fn list_terms(&self, ontology_id: &Uuid) -> Result<Vec<Term>, StorageError> {
        let query = "
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(t:Term)
            RETURN t
        ";

        let params = Self::create_params(vec![
            ("ontology_id", ontology_id.to_string()),
        ]);

        let result = self.get_connection().await?.execute(query::new(query).params(params)).await
            .map_err(|e| StorageError::QueryError(format!("Failed to list terms: {}", e)))?;

        let mut terms = Vec::new();
        let mut result = result;

        while let Some(row) = result.next().await {
            let node = row.get::<Node>("t")
                .map_err(|e| StorageError::BackendError(format!("Failed to get term node: {}", e)))?;

            let term = self.node_to_term(node).await?;
            terms.push(term);
        }

        Ok(terms)
    }

    /// Delete a term
    async fn delete_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> StorageResult<()> {
        let conn = self.get_connection().await?;

        // Delete the term and its relationships
        let delete_term = r#"
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(t:Term {id: $term_id})
            OPTIONAL MATCH (o)-[:CONTAINS]->(r:OntologyRelationship)-[:FROM|TO]->(t)
            DETACH DELETE t, r
        "#;

        let mut params = HashMap::new();
        params.insert("ontology_id", neo4rs::Node::from(ontology_id.to_string()));
        params.insert("term_id", neo4rs::Node::from(term_id.to_string()));

        conn.run(query(delete_term).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete term: {}", e)))?;

        Ok(())
    }

    /// Save a relationship to an ontology
    async fn save_relationship(&self, ontology_id: &Uuid, relationship: &Relationship) -> StorageResult<()> {
        let conn = self.get_connection().await?;

        // Create or update the relationship node
        let create_relationship = r#"
            MATCH (o:Ontology {id: $ontology_id})
            MATCH (source:Term {id: $source_id})
            MATCH (target:Term {id: $target_id})
            MERGE (r:OntologyRelationship {id: $id})
            ON CREATE SET r.created_at = datetime()
            SET r.type = $type,
                r.description = $description,
                r.strength = $strength,
                r.updated_at = datetime()
            MERGE (o)-[:CONTAINS]->(r)
            MERGE (r)-[:FROM]->(source)
            MERGE (r)-[:TO]->(target)
            RETURN r
        "#;

        let rel_type = match &relationship.relationship_type {
            RelationshipType::IsA => "is_a",
            RelationshipType::HasA => "has_a",
            RelationshipType::PartOf => "part_of",
            RelationshipType::RelatedTo => "related_to",
            RelationshipType::Custom(name) => name,
        };

        let mut params = HashMap::new();
        params.insert("ontology_id", neo4rs::Node::from(ontology_id.to_string()));
        params.insert("id", neo4rs::Node::from(relationship.id.to_string()));
        params.insert("source_id", neo4rs::Node::from(relationship.source_term_id.to_string()));
        params.insert("target_id", neo4rs::Node::from(relationship.target_term_id.to_string()));
        params.insert("type", neo4rs::Node::from(rel_type));
        params.insert("description", neo4rs::Node::from(relationship.description.clone().unwrap_or_default()));
        params.insert("strength", neo4rs::Node::from(relationship.strength));

        conn.run(query(create_relationship).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to save relationship: {}", e)))?;

        Ok(())
    }

    /// Get a relationship by ID
    async fn get_relationship(&self, ontology_id: &Uuid, relationship_id: &Uuid) -> StorageResult<Relationship> {
        let conn = self.get_connection().await?;

        // Get the relationship node and connected terms
        let get_relationship = r#"
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(r:OntologyRelationship {id: $rel_id})
            MATCH (r)-[:FROM]->(source:Term)
            MATCH (r)-[:TO]->(target:Term)
            RETURN r.type as type, r.description as description, r.strength as strength,
                   source.id as source_id, target.id as target_id
        "#;

        let mut params = HashMap::new();
        params.insert("ontology_id", neo4rs::Node::from(ontology_id.to_string()));
        params.insert("rel_id", neo4rs::Node::from(relationship_id.to_string()));

        let mut result = conn.execute(query(get_relationship).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to get relationship: {}", e)))?;

        if let Some(row) = result.next().await {
            let rel_type: String = row.get("type").map_err(|e| StorageError::BackendError(format!("Failed to get type: {}", e)))?;
            let description: Option<String> = row.get("description").ok();
            let strength: f64 = row.get("strength").unwrap_or(1.0);
            
            let source_id: String = row.get("source_id").map_err(|e| StorageError::BackendError(format!("Failed to get source_id: {}", e)))?;
            let source_id = Uuid::parse_str(&source_id).map_err(|e| StorageError::BackendError(format!("Invalid source UUID: {}", e)))?;
            
            let target_id: String = row.get("target_id").map_err(|e| StorageError::BackendError(format!("Failed to get target_id: {}", e)))?;
            let target_id = Uuid::parse_str(&target_id).map_err(|e| StorageError::BackendError(format!("Invalid target UUID: {}", e)))?;

            let relationship_type = match rel_type.as_str() {
                "is_a" => RelationshipType::IsA,
                "has_a" => RelationshipType::HasA,
                "part_of" => RelationshipType::PartOf,
                "related_to" => RelationshipType::RelatedTo,
                custom => RelationshipType::Custom(custom.to_string()),
            };

            let mut relationship = Relationship::with_strength(source_id, target_id, relationship_type, strength);
            relationship.id = *relationship_id;
            relationship.description = description;

            Ok(relationship)
        } else {
            Err(StorageError::NotFound(format!("Relationship not found: {}", relationship_id)))
        }
    }

    /// List relationships for an ontology
    async fn list_relationships(&self, ontology_id: &Uuid) -> Result<Vec<Relationship>, StorageError> {
        let query = "
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(r:OntologyRelationship)
            MATCH (s:Term)<-[:SOURCE]-(r)-[:TARGET]->(t:Term)
            RETURN r
        ";

        let params = Self::create_params(vec![
            ("ontology_id", ontology_id.to_string()),
        ]);

        let result = self.get_connection().await?.execute(query::new(query).params(params)).await
            .map_err(|e| StorageError::QueryError(format!("Failed to list relationships: {}", e)))?;

        let mut relationships = Vec::new();
        let mut result = result;

        while let Some(row) = result.next().await {
            let rel = row.get::<neo4rs::Relation>("r")
                .map_err(|e| StorageError::BackendError(format!("Failed to get relationship: {}", e)))?;

            let relationship = self.rel_to_relationship(rel, ontology_id).await?;
            relationships.push(relationship);
        }

        Ok(relationships)
    }

    /// Delete a relationship
    async fn delete_relationship(&self, ontology_id: &Uuid, relationship_id: &Uuid) -> StorageResult<()> {
        let conn = self.get_connection().await?;

        // Delete the relationship
        let delete_relationship = r#"
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(r:OntologyRelationship {id: $rel_id})
            DETACH DELETE r
        "#;

        let mut params = HashMap::new();
        params.insert("ontology_id", neo4rs::Node::from(ontology_id.to_string()));
        params.insert("rel_id", neo4rs::Node::from(relationship_id.to_string()));

        conn.run(query(delete_relationship).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete relationship: {}", e)))?;

        Ok(())
    }

    /// Find terms by name pattern
    async fn find_terms_by_name(&self, ontology_id: &Uuid, name_pattern: &str) -> StorageResult<Vec<Term>> {
        let conn = self.get_connection().await?;

        // Find terms matching the pattern
        let find_terms = r#"
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(t:Term)
            WHERE t.name =~ $pattern
            RETURN t.id as id
        "#;

        let mut params = HashMap::new();
        params.insert("ontology_id", neo4rs::Node::from(ontology_id.to_string()));
        params.insert("pattern", neo4rs::Node::from(format!("(?i).*{}.*", name_pattern)));

        let mut result = conn.execute(query(find_terms).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to find terms: {}", e)))?;

        let mut terms = Vec::new();
        while let Some(row) = result.next().await {
            let id: String = row.get("id").map_err(|e| StorageError::BackendError(format!("Failed to get id: {}", e)))?;
            let id = Uuid::parse_str(&id).map_err(|e| StorageError::BackendError(format!("Invalid UUID: {}", e)))?;
            
            // Get the full term
            match self.get_term(ontology_id, &id).await {
                Ok(term) => terms.push(term),
                Err(e) => eprintln!("Error getting term {}: {}", id, e),
            }
        }

        Ok(terms)
    }

    /// Find relationships by term
    async fn find_relationships_by_term(&self, ontology_id: &Uuid, term_id: &Uuid) -> StorageResult<Vec<Relationship>> {
        let query = "
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(r:Relationship)
            MATCH (t:Term {id: $term_id})
            WHERE (t)<-[:SOURCE]-(r) OR (t)<-[:TARGET]-(r)
            MATCH (s:Term)<-[:SOURCE]-(r)-[:TARGET]->(tar:Term)
            RETURN r
        ";

        let params = Self::create_params(vec![
            ("ontology_id", ontology_id.to_string()),
            ("term_id", term_id.to_string()),
        ]);

        let result = self.get_connection().await?.execute(query::new(query).params(params)).await
            .map_err(|e| StorageError::QueryError(format!("Failed to find relationships by term: {}", e)))?;

        let mut relationships = Vec::new();
        let mut result = result;

        while let Some(row) = result.next().await {
            let rel = row.get::<neo4rs::Relation>("r")
                .map_err(|e| StorageError::BackendError(format!("Failed to get relationship: {}", e)))?;

            let relationship = self.rel_to_relationship(rel, ontology_id).await?;
            relationships.push(relationship);
        }

        Ok(relationships)
    }

    /// Find relationships between terms
    async fn find_relationships_between_terms(
        &self,
        ontology_id: &Uuid,
        source_term_id: &Uuid,
        target_term_id: &Uuid,
    ) -> StorageResult<Vec<Relationship>> {
        let query = "
            MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(r:Relationship)
            MATCH (s:Term {id: $source_term_id})<-[:SOURCE]-(r)-[:TARGET]->(t:Term {id: $target_term_id})
            RETURN r
        ";

        let params = Self::create_params(vec![
            ("ontology_id", ontology_id.to_string()),
            ("source_term_id", source_term_id.to_string()),
            ("target_term_id", target_term_id.to_string()),
        ]);

        let result = self.get_connection().await?.execute(query::new(query).params(params)).await
            .map_err(|e| StorageError::QueryError(format!("Failed to find relationships between terms: {}", e)))?;

        let mut relationships = Vec::new();
        let mut result = result;

        while let Some(row) = result.next().await {
            let rel = row.get::<neo4rs::Relation>("r")
                .map_err(|e| StorageError::BackendError(format!("Failed to get relationship: {}", e)))?;

            let relationship = self.rel_to_relationship(rel, ontology_id).await?;
            relationships.push(relationship);
        }

        Ok(relationships)
    }

    /// Search for terms across all ontologies
    async fn search_terms(&self, name_pattern: &str) -> StorageResult<Vec<(Uuid, Term)>> {
        let conn = self.get_connection().await?;

        // Search for terms across all ontologies
        let search_terms = r#"
            MATCH (o:Ontology)-[:CONTAINS]->(t:Term)
            WHERE t.name =~ $pattern
            RETURN o.id as ontology_id, t.id as term_id
        "#;

        let mut params = HashMap::new();
        params.insert("pattern", neo4rs::Node::from(format!("(?i).*{}.*", name_pattern)));

        let mut result = conn.execute(query(search_terms).params(params))
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to search terms: {}", e)))?;

        let mut terms = Vec::new();
        while let Some(row) = result.next().await {
            let ontology_id: String = row.get("ontology_id").map_err(|e| StorageError::BackendError(format!("Failed to get ontology_id: {}", e)))?;
            let ontology_id = Uuid::parse_str(&ontology_id).map_err(|e| StorageError::BackendError(format!("Invalid ontology UUID: {}", e)))?;
            
            let term_id: String = row.get("term_id").map_err(|e| StorageError::BackendError(format!("Failed to get term_id: {}", e)))?;
            let term_id = Uuid::parse_str(&term_id).map_err(|e| StorageError::BackendError(format!("Invalid term UUID: {}", e)))?;
            
            // Get the full term
            match self.get_term(&ontology_id, &term_id).await {
                Ok(term) => terms.push((ontology_id, term)),
                Err(e) => eprintln!("Error getting term {}: {}", term_id, e),
            }
        }

        Ok(terms)
    }
} 