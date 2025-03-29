# Domain Vocabulary Organization

This directory contains the organized vocabulary for our system, structured by specific domains. Each domain has its own dedicated file with terms, definitions, and relationships following our vocabulary template.

## Domain Structure

### Communications Domain (`communications.md`)
- Message handling
- Domain content
- Subject-based routing
- Event and object storage

### Domain Configuration (`domain_config.md`)
- Domain modeling concepts
- Configuration management
- Domain events, commands, and queries

### AI and Strategy (`ai_strategy.md`)
- Conceptual spaces
- Game theory concepts
- AI agent components
- Decision-making strategies

### Infrastructure (`infrastructure.md`)
- Core platforms (NixOS, nats Jetstream, MCPs)
- Development tools
- Service components
- Security infrastructure

### Business Value (`business_value.md`)
- Value propositions
- Customer segments
- Resource management
- Inventory concepts

## Term Template

Each term in the domain files follows this structure:

```markdown
### Term: [Term Name]
- **Category**: [Domain Object|Technical Concept|Business Concept|Cross-Cutting]
- **Type**: [Aggregate|Entity|Value Object|Service|etc.]
- **Taxonomy**: [Primary taxonomy this belongs to]
- **Definition**: Clear, concise definition
- **Relationships**:
  * [Relationship Type]: Related terms
- **Usage Context**: Where/how this term is used
- **Code Reference**: Link to implementing code (if applicable)
```

## Maintenance Guidelines

1. **Term Addition**:
   - Categorize appropriately
   - Define clear relationships
   - Document usage context
   - Update knowledge graph

2. **Term Updates**:
   - Review existing relationships
   - Maintain consistency
   - Document changes
   - Update affected terms

3. **Term Removal**:
   - Document reason
   - Update related terms
   - Maintain graph integrity
   - Archive definition

## Knowledge Graph

Terms are interconnected through explicit relationships, forming a knowledge graph that helps understand:
- Term hierarchies
- Cross-domain relationships
- Usage patterns
- Implementation dependencies 