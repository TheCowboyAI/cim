//! Utilities module
//!
//! This module provides utility functions and types for the ontology tool.

/// Check if a string is a valid UUID
pub fn is_valid_uuid(s: &str) -> bool {
    uuid::Uuid::parse_str(s).is_ok()
}

/// Get the current timestamp in ISO 8601 format
pub fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
} 