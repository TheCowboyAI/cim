# Organization Domain Vocabulary

## Term: Operator
- **Category**: Business Concept
- **Type**: Entity
- **Taxonomy**: Configuration Taxonomy
- **Definition**: Organization responsible for operating a CIM instance
- **Relationships**:
  * Manages: Tenants
  * Configures: Policies
  * Contains: Accounts
- **Usage Context**: Primary administrative entity for CIM operations
- **Code Reference**: TBD

## Term: Account
- **Category**: Business Concept
- **Type**: Entity
- **Taxonomy**: Configuration Taxonomy
- **Definition**: A group or individual identity within the CIM system
- **Relationships**:
  * Part-Of: Operator
  * Contains: Users
  * Has: Permissions
- **Usage Context**: Access control and resource management
- **Code Reference**: TBD 