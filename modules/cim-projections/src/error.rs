use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectionError {
    #[error("Store error: {0}")]
    Store(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Event processing error: {0}")]
    EventProcessing(String),
    
    #[error("Position tracking error: {0}")]
    PositionTracking(String),
    
    #[error("Query error: {0}")]
    Query(String),
    
    #[error("Projection not found: {0}")]
    ProjectionNotFound(String),
    
    #[error("Concurrent modification detected")]
    ConcurrentModification,
    
    #[error("Database error: {0}")]
    Database(String),
}

pub type Result<T> = std::result::Result<T, ProjectionError>;