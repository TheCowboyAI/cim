use std::collections::HashMap;
use std::time::Duration;

use async_nats::{Client as NatsClient, ConnectOptions};
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::time::timeout;
use tracing::{debug, error, info, trace};
use uuid::Uuid;

use crate::domain::{MCPRequest, MCPResponse, MCPStatus, Ontology, Relationship, Term};
use crate::error::{Error, Result};

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_SUBJECT_PREFIX: &str = "mcp";

/// MCP client for communicating with CIM Ontology Tool over NATS
pub struct MCPClient {
    /// The underlying NATS client
    nats: NatsClient,
    
    /// Default timeout for requests
    timeout_secs: u64,
    
    /// The subject prefix to use for MCP operations
    subject_prefix: String,
    
    /// Authentication token, if required
    auth_token: Option<String>,
}

impl MCPClient {
    /// Create a new MCP client with the given NATS connection
    pub async fn new(nats_url: &str) -> Result<Self> {
        let nats = async_nats::connect(nats_url).await?;
        Ok(Self {
            nats,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            subject_prefix: DEFAULT_SUBJECT_PREFIX.to_string(),
            auth_token: None,
        })
    }

    /// Create a new MCP client with custom connection options
    pub async fn with_options(nats_url: &str, options: ConnectOptions) -> Result<Self> {
        let nats = options.connect(nats_url).await?;
        Ok(Self {
            nats,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            subject_prefix: DEFAULT_SUBJECT_PREFIX.to_string(),
            auth_token: None,
        })
    }

    /// Set authentication token for MCP requests
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    /// Set default timeout for requests
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Set subject prefix for MCP operations
    pub fn with_subject_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.subject_prefix = prefix.into();
        self
    }

    /// Send a request and wait for a response
    pub async fn send_request<T: DeserializeOwned>(
        &self,
        operation: impl Into<String>,
        parameters: HashMap<String, Value>,
    ) -> Result<T> {
        let request = MCPRequest {
            request_id: Uuid::new_v4().to_string(),
            operation: operation.into(),
            parameters,
            context: self.create_context(),
        };

        let subject = format!("{}.request", self.subject_prefix);
        let payload = serde_json::to_vec(&request)?;
        
        debug!(?request, "Sending MCP request");
        
        let timeout_duration = Duration::from_secs(self.timeout_secs);
        let response = timeout(
            timeout_duration,
            self.nats.request(subject, payload.into()),
        )
        .await
        .map_err(|_| Error::Timeout(self.timeout_secs))?;

        let response_payload = response.payload.to_vec();
        trace!(response = ?String::from_utf8_lossy(&response_payload), "Received MCP response");
        
        let mcp_response: MCPResponse = serde_json::from_slice(&response_payload)?;
        
        match mcp_response.status {
            MCPStatus::Success => {
                if let Some(data) = mcp_response.data {
                    Ok(serde_json::from_value(data)?)
                } else {
                    Err(Error::NoResponse)
                }
            }
            MCPStatus::Error => {
                if let Some(error) = mcp_response.error {
                    Err(Error::MCPError {
                        code: error.code,
                        message: error.message,
                    })
                } else {
                    Err(Error::InvalidResponse("Error status with no error details".into()))
                }
            }
            MCPStatus::Pending => {
                Err(Error::InvalidResponse("Received pending status".into()))
            }
        }
    }

    /// List all ontologies
    pub async fn list_ontologies(&self) -> Result<Vec<Ontology>> {
        self.send_request("list_ontologies", HashMap::new()).await
    }
    
    /// Get a specific ontology by ID
    pub async fn get_ontology(&self, id: &str) -> Result<Ontology> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), Value::String(id.to_string()));
        self.send_request("get_ontology", params).await
    }

    /// Create a new ontology
    pub async fn create_ontology(
        &self,
        name: &str,
        description: &str,
        domain: &str,
    ) -> Result<Ontology> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), Value::String(name.to_string()));
        params.insert("description".to_string(), Value::String(description.to_string()));
        params.insert("domain".to_string(), Value::String(domain.to_string()));
        
        self.send_request("create_ontology", params).await
    }

    /// Update an existing ontology
    pub async fn update_ontology(
        &self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        domain: Option<&str>,
    ) -> Result<Ontology> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), Value::String(id.to_string()));
        
        if let Some(name) = name {
            params.insert("name".to_string(), Value::String(name.to_string()));
        }
        
        if let Some(description) = description {
            params.insert("description".to_string(), Value::String(description.to_string()));
        }
        
        if let Some(domain) = domain {
            params.insert("domain".to_string(), Value::String(domain.to_string()));
        }
        
        self.send_request("update_ontology", params).await
    }

    /// Delete an ontology
    pub async fn delete_ontology(&self, id: &str) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), Value::String(id.to_string()));
        
        self.send_request::<Value>("delete_ontology", params).await?;
        Ok(())
    }

    /// Create a new term in an ontology
    pub async fn create_term(
        &self,
        ontology_id: &str,
        name: &str,
        description: &str,
        properties: Option<HashMap<String, Value>>,
    ) -> Result<Term> {
        let mut params = HashMap::new();
        params.insert("ontology_id".to_string(), Value::String(ontology_id.to_string()));
        params.insert("name".to_string(), Value::String(name.to_string()));
        params.insert("description".to_string(), Value::String(description.to_string()));
        
        if let Some(props) = properties {
            params.insert("properties".to_string(), Value::Object(serde_json::Map::from_iter(props.into_iter())));
        }
        
        self.send_request("create_term", params).await
    }

    /// Create a relationship between terms
    pub async fn create_relationship(
        &self,
        ontology_id: &str,
        relationship_type: &str,
        source_term_id: &str,
        target_term_id: &str,
        properties: Option<HashMap<String, Value>>,
    ) -> Result<Relationship> {
        let mut params = HashMap::new();
        params.insert("ontology_id".to_string(), Value::String(ontology_id.to_string()));
        params.insert("relationship_type".to_string(), Value::String(relationship_type.to_string()));
        params.insert("source_term_id".to_string(), Value::String(source_term_id.to_string()));
        params.insert("target_term_id".to_string(), Value::String(target_term_id.to_string()));
        
        if let Some(props) = properties {
            params.insert("properties".to_string(), Value::Object(serde_json::Map::from_iter(props.into_iter())));
        }
        
        self.send_request("create_relationship", params).await
    }

    // Subscribe to MCP events
    pub async fn subscribe_events(&self) -> Result<impl Stream<Item = MCPResponse>> {
        let subject = format!("{}.events.>", self.subject_prefix);
        let subscription = self.nats.subscribe(subject).await?;
        
        Ok(subscription.map(|msg| {
            match serde_json::from_slice::<MCPResponse>(&msg.payload) {
                Ok(response) => response,
                Err(e) => {
                    error!(error = ?e, "Failed to deserialize MCP event");
                    MCPResponse {
                        request_id: "error".to_string(),
                        status: MCPStatus::Error,
                        data: None,
                        error: Some(crate::domain::MCPError {
                            code: "parse_error".to_string(),
                            message: e.to_string(),
                            details: None,
                        }),
                    }
                }
            }
        }))
    }

    // Helper to create context for requests
    fn create_context(&self) -> HashMap<String, Value> {
        let mut context = HashMap::new();
        
        // Add auth token if present
        if let Some(token) = &self.auth_token {
            context.insert("auth_token".to_string(), Value::String(token.clone()));
        }
        
        // Add client version
        context.insert(
            "client_version".to_string(),
            Value::String(env!("CARGO_PKG_VERSION").to_string()),
        );
        
        context
    }
} 