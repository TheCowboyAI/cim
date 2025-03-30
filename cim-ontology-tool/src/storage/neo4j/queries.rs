//! Cypher queries for Neo4j operations

// Ontology Queries
pub const CREATE_OR_UPDATE_ONTOLOGY: &str = r#"
MERGE (o:Ontology {id: $id})
SET o.name = $name,
    o.description = $description,
    o.metadata = $metadata
RETURN o
"#;

pub const GET_ONTOLOGY_BY_ID: &str = r#"
MATCH (n:Ontology {id: $id})
RETURN n
"#;

pub const LIST_ONTOLOGIES: &str = r#"
MATCH (n:Ontology)
OPTIONAL MATCH (n)-[:CONTAINS]->(t:Term)
OPTIONAL MATCH (n)-[:CONTAINS]->(r:Relationship)
WITH n, COUNT(DISTINCT t) AS termCount, COUNT(DISTINCT r) AS relationshipCount
RETURN n, termCount, relationshipCount
"#;

pub const DELETE_ONTOLOGY: &str = r#"
MATCH (o:Ontology {id: $id})
OPTIONAL MATCH (o)-[:CONTAINS]->(t)
DETACH DELETE o, t
"#;

// Term Queries
pub const CREATE_OR_UPDATE_TERM: &str = r#"
MATCH (o:Ontology {id: $ontology_id})
MERGE (t:Term {id: $term_id})
SET t.name = $name,
    t.definition = $definition,
    t.domain = $domain,
    t.metadata = $metadata
MERGE (o)-[:CONTAINS]->(t)
RETURN t
"#;

pub const GET_TERM_BY_ID: &str = r#"
MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(n:Term {id: $term_id})
RETURN n
"#;

pub const GET_TERMS_FOR_ONTOLOGY: &str = r#"
MATCH (o:Ontology {id: $id})-[:CONTAINS]->(n:Term)
RETURN n
"#;

pub const SEARCH_TERMS_BY_NAME: &str = r#"
MATCH (n:Term)
WHERE n.name CONTAINS $name_pattern
RETURN n
"#;

pub const FIND_TERMS_BY_DOMAIN: &str = r#"
MATCH (n:Term)
WHERE n.domain = $domain
RETURN n
"#;

// Relationship Queries
pub const CREATE_OR_UPDATE_RELATIONSHIP: &str = r#"
MATCH (o:Ontology {id: $ontology_id})
MATCH (source:Term {id: $source_term_id})
MATCH (target:Term {id: $target_term_id})
MERGE (r:Relationship {id: $relationship_id})
SET r.type = $relationship_type,
    r.strength = $strength,
    r.metadata = $metadata
MERGE (o)-[:CONTAINS]->(r)
MERGE (r)-[:SOURCE]->(source)
MERGE (r)-[:TARGET]->(target)
RETURN r
"#;

pub const GET_RELATIONSHIP_BY_ID: &str = r#"
MATCH (o:Ontology {id: $ontology_id})-[:CONTAINS]->(n:Relationship {id: $relationship_id})
MATCH (n)-[:SOURCE]->(source:Term)
MATCH (n)-[:TARGET]->(target:Term)
RETURN n, source.id AS source_id, target.id AS target_id
"#;

pub const GET_RELATIONSHIPS_FOR_ONTOLOGY: &str = r#"
MATCH (o:Ontology {id: $id})-[:CONTAINS]->(n:Relationship)
MATCH (n)-[:SOURCE]->(source:Term)
MATCH (n)-[:TARGET]->(target:Term)
RETURN n, source.id AS source_id, target.id AS target_id
"#;

pub const FIND_RELATIONSHIPS_BY_TYPE: &str = r#"
MATCH (n:Relationship)
WHERE n.type = $relationship_type
MATCH (n)-[:SOURCE]->(source:Term)
MATCH (n)-[:TARGET]->(target:Term)
RETURN n, source.id AS source_id, target.id AS target_id
"#; 