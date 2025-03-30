pub mod client;
pub mod domain;
pub mod error;

pub use client::MCPClient;
pub use domain::{MCPRequest, MCPResponse, MCPStatus, Ontology, Term, Relationship};
pub use error::{Error, Result}; 