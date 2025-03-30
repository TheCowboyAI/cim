use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// A source represents the origin of ontological information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Unique identifier for the source
    pub id: Uuid,
    /// Type of the source
    pub source_type: SourceType,
    /// Location information for the source
    pub location: SourceLocation,
    /// Timestamp when the source was processed
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional metadata about the source
    pub metadata: HashMap<String, String>,
}

/// Types of sources for ontological information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceType {
    /// Information extracted from a file
    File,
    /// Information from a seed ontology
    SeedOntology,
    /// Information manually entered
    Manual,
    /// Information from an external API or service
    External,
    /// Information derived from analysis
    Derived,
}

/// Location information for a source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceLocation {
    /// File path and optional line range
    File {
        path: PathBuf,
        line_start: Option<usize>,
        line_end: Option<usize>,
    },
    /// URL and optional fragment
    Url {
        url: String,
        fragment: Option<String>,
    },
    /// Reference to another ontology
    Ontology {
        id: Uuid,
        name: String,
    },
    /// User or system that created the information
    User {
        name: String,
    },
    /// Information derived from other sources
    Derived {
        source_ids: Vec<Uuid>,
        description: String,
    },
}

impl Source {
    /// Create a new source from a file
    pub fn from_file(path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_type: SourceType::File,
            location: SourceLocation::File {
                path,
                line_start: None,
                line_end: None,
            },
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new source from a file with line range
    pub fn from_file_range(path: PathBuf, line_start: usize, line_end: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_type: SourceType::File,
            location: SourceLocation::File {
                path,
                line_start: Some(line_start),
                line_end: Some(line_end),
            },
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new source from a seed ontology
    pub fn from_seed_ontology(ontology_id: Uuid, ontology_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_type: SourceType::SeedOntology,
            location: SourceLocation::Ontology {
                id: ontology_id,
                name: ontology_name,
            },
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new source from a URL
    pub fn from_url(url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_type: SourceType::External,
            location: SourceLocation::Url {
                url,
                fragment: None,
            },
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new source from a user
    pub fn from_user(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_type: SourceType::Manual,
            location: SourceLocation::User { name },
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new derived source
    pub fn derived(source_ids: Vec<Uuid>, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_type: SourceType::Derived,
            location: SourceLocation::Derived {
                source_ids,
                description,
            },
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the source
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
} 