use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// The status of an MCP operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MCPStatus {
    Success,
    Pending,
    Error,
}

/// The request structure for the MCP protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    /// Unique identifier for this request
    pub request_id: String,
    
    /// The operation to perform (e.g., "list_ontologies", "get_ontology", etc.)
    pub operation: String,
    
    /// Parameters for the operation
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Additional context for the request
    #[serde(default)]
    pub context: HashMap<String, serde_json::Value>,
}

impl MCPRequest {
    /// Create a new MCP request
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            operation: operation.into(),
            parameters: HashMap::new(),
            context: HashMap::new(),
        }
    }

    /// Add a parameter to the request
    pub fn with_param<T: Serialize>(mut self, key: &str, value: T) -> Result<Self, serde_json::Error> {
        let value = serde_json::to_value(value)?;
        self.parameters.insert(key.to_string(), value);
        Ok(self)
    }

    /// Add context to the request
    pub fn with_context<T: Serialize>(mut self, key: &str, value: T) -> Result<Self, serde_json::Error> {
        let value = serde_json::to_value(value)?;
        self.context.insert(key.to_string(), value);
        Ok(self)
    }
}

/// The response structure for the MCP protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// The request ID this response is for
    pub request_id: String,
    
    /// The status of the operation
    pub status: MCPStatus,
    
    /// The response data, if any
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    
    /// Error information, if any
    #[serde(default)]
    pub error: Option<MCPError>,
}

/// Error information for MCP responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    /// Error code
    pub code: String,
    
    /// Error message
    pub message: String,
    
    /// Additional details, if any
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

/// Represents an ontology in the CIM system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ontology {
    /// Unique identifier for the ontology
    pub id: String,
    
    /// Name of the ontology
    pub name: String,
    
    /// Description of the ontology
    pub description: String,
    
    /// Domain this ontology belongs to
    pub domain: String,
    
    /// When this ontology was created
    pub created_at: String,
    
    /// When this ontology was last updated
    pub updated_at: String,
    
    /// List of terms in this ontology
    #[serde(default)]
    pub terms: Vec<Term>,
    
    /// List of relationships in this ontology
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

/// Represents a term in an ontology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    /// Unique identifier for the term
    pub id: String,
    
    /// The ontology this term belongs to
    pub ontology_id: String,
    
    /// Name of the term
    pub name: String,
    
    /// Description of the term
    pub description: String,
    
    /// Additional properties for the term
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Represents a relationship between terms in an ontology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Unique identifier for the relationship
    pub id: String,
    
    /// The ontology this relationship belongs to
    pub ontology_id: String,
    
    /// Type of the relationship (e.g., "is_a", "part_of", etc.)
    pub relationship_type: String,
    
    /// Source term ID
    pub source_term_id: String,
    
    /// Target term ID
    pub target_term_id: String,
    
    /// Additional properties for the relationship
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
} 