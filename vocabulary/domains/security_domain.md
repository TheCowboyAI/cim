# Security Domain Vocabulary

## Term: Authorization Context
- **Category**: Cross-Cutting
- **Type**: Service
- **Taxonomy**: Configuration Taxonomy
- **Definition**: Security context that determines access rights and permissions for information access
- **Relationships**:
  * Manages: Access Control
  * Validates: User Permissions
  * Configures: Security Policies
- **Usage Context**: Used in all operations that require secure access to information
- **Code Reference**: TBD

## Term: Information Integrity
- **Category**: Cross-Cutting
- **Type**: Service
- **Taxonomy**: Storage Taxonomy
- **Definition**: Mechanisms ensuring information remains accurate and unaltered throughout its lifecycle
- **Relationships**:
  * Validates: Information State
  * Triggers: Integrity Checks
  * Depends-On: Authorization Context
- **Usage Context**: Continuous verification of information authenticity and completeness
- **Code Reference**: TBD 