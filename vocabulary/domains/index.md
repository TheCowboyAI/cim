# CIM Domain Vocabularies

This directory contains the domain-specific vocabularies for the CIM system, following the structure defined in @vocabulary.mdc.

## Domain Files
- [Information Domain](information_domain.md)
- [Knowledge Domain](knowledge_domain.md)
- [Security Domain](security_domain.md)
- [Distribution Domain](distribution_domain.md)
- [AI Integration Domain](ai_integration_domain.md)
- [Persistence Domain](persistence_domain.md)
- [Organization Domain](organization_domain.md)
- [Agent Domain](agent_domain.md)
- [Business Domain](business_domain.md)
- [Environment Domain](environment_domain.md)
- [Governance Domain](governance_domain.md)

## Maintenance
Each domain vocabulary file follows the standard template from @vocabulary.mdc:

```markdown
## Term: [Term Name]
- **Category**: [Domain Object|Technical Concept|Business Concept|Cross-Cutting]
- **Type**: [Aggregate|Entity|Value Object|Service|etc.]
- **Taxonomy**: [Primary taxonomy this belongs to]
- **Definition**: Clear, concise definition
- **Relationships**:
  * [Relationship Type]: Related terms
- **Usage Context**: Where/how this term is used
- **Code Reference**: Link to implementing code (if applicable)
```

## Related Files
- [Domain Taxonomy](../taxonomies/domain_taxonomy.yaml)
- [Domain Ontology](../ontologies/cim_ontology.ttl)
- [Domain Relationships](../knowledge/domain_relationships.md) 