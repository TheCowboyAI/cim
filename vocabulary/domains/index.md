# CIM Domain Vocabulary Index

## Domain Objects

### Nodes

```cypher
(:VocabularyIndex:Aggregate {
  domain: "Vocabulary",
  term: "VocabularyIndex",
  definition: "Central directory of all domain-specific vocabularies for the CIM system",
  taxonomy: "Documentation Taxonomy",
  usage_context: "Vocabulary navigation and domain discovery",
  code_reference: "vocabulary/domains/index.md"
})

(:DomainFile:Entity {
  domain: "Vocabulary",
  term: "DomainFile",
  definition: "A vocabulary file defining terms and concepts for a specific domain",
  taxonomy: "Documentation Taxonomy",
  usage_context: "Domain-specific term definition and standardization",
  code_reference: "vocabulary/domains/template.md"
})

(:VocabularyStandard:Entity {
  domain: "Vocabulary",
  term: "VocabularyStandard",
  definition: "The standardized format and rules for defining vocabulary terms",
  taxonomy: "Documentation Taxonomy",
  usage_context: "Ensuring consistency across domain vocabularies",
  code_reference: ".cursor/rules/vocabulary.mdc"
})

(:DomainTaxonomy:Entity {
  domain: "Vocabulary",
  term: "DomainTaxonomy",
  definition: "Hierarchical classification of terms within domains",
  taxonomy: "Documentation Taxonomy",
  usage_context: "Term categorization and hierarchical organization",
  code_reference: "vocabulary/taxonomies/domain_taxonomy.yaml"
})

(:DomainOntology:Entity {
  domain: "Vocabulary",
  term: "DomainOntology",
  definition: "Formal semantic representation of domains and their relationships",
  taxonomy: "Documentation Taxonomy",
  usage_context: "Semantic relationships and reasoning about domains",
  code_reference: "vocabulary/ontologies/cim_ontology.ttl"
})

(:DomainRelationship:Entity {
  domain: "Vocabulary",
  term: "DomainRelationship",
  definition: "Connections and dependencies between different domains",
  taxonomy: "Documentation Taxonomy",
  usage_context: "Cross-domain integration and boundary definition",
  code_reference: "vocabulary/knowledge/domain_relationships.md"
})
```

## Relationships

```cypher
// VocabularyIndex relationships
(:VocabularyIndex)-[:CONTAINS {type: "composition"}]->(:DomainFile)
(:VocabularyIndex)-[:REFERENCES {type: "link"}]->(:DomainTaxonomy)
(:VocabularyIndex)-[:REFERENCES {type: "link"}]->(:DomainOntology)
(:VocabularyIndex)-[:REFERENCES {type: "link"}]->(:DomainRelationship)

// DomainFile relationships
(:DomainFile)-[:FOLLOWS {type: "compliance"}]->(:VocabularyStandard)
(:DomainFile)-[:CONTRIBUTES_TO {type: "input"}]->(:DomainTaxonomy)
(:DomainFile)-[:CONTRIBUTES_TO {type: "input"}]->(:DomainOntology)

// Domain-specific files
(:DomainFile {name: "Information Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:InformationDomain)
(:DomainFile {name: "Knowledge Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:KnowledgeDomain)
(:DomainFile {name: "Security Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:SecurityDomain)
(:DomainFile {name: "Distribution Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:DistributionDomain)
(:DomainFile {name: "AI Integration Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:AIIntegrationDomain)
(:DomainFile {name: "Persistence Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:PersistenceDomain)
(:DomainFile {name: "Organization Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:OrganizationDomain)
(:DomainFile {name: "Agent Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:AgentDomain)
(:DomainFile {name: "Business Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:BusinessDomain)
(:DomainFile {name: "Environment Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:EnvironmentDomain)
(:DomainFile {name: "Governance Domain"})-[:DEFINES_TERMS_FOR {type: "scope"}]->(:GovernanceDomain)

// VocabularyStandard relationships
(:VocabularyStandard)-[:ENFORCES {type: "rule"}]->(:TermStructure)
(:VocabularyStandard)-[:ENFORCES {type: "rule"}]->(:PropertyRequirement)

// DomainTaxonomy relationships
(:DomainTaxonomy)-[:ORGANIZES {type: "classification"}]->(:TermCategory)
(:DomainTaxonomy)-[:REFERENCED_BY {type: "usage"}]->(:DomainFile)

// DomainOntology relationships
(:DomainOntology)-[:FORMALIZES {type: "representation"}]->(:DomainRelationship)
(:DomainOntology)-[:ENABLES {type: "capability"}]->(:SemanticReasoning)

// DomainRelationship relationships
(:DomainRelationship)-[:CONNECTS {type: "link"}]->(:DomainFile)
(:DomainRelationship)-[:DEFINES {type: "specification"}]->(:DomainBoundary)
```

## Taxonomies

### Documentation Taxonomy

```cypher
(:Taxonomy {name: "Documentation Taxonomy"})
-[:CONTAINS]->(:Category {name: "Vocabulary Components"})
-[:CONTAINS]->(:Term {name: "VocabularyIndex"})
-[:CONTAINS]->(:Term {name: "DomainFile"})
-[:CONTAINS]->(:Term {name: "VocabularyStandard"})

(:Taxonomy {name: "Documentation Taxonomy"})
-[:CONTAINS]->(:Category {name: "Semantic Components"})
-[:CONTAINS]->(:Term {name: "DomainTaxonomy"})
-[:CONTAINS]->(:Term {name: "DomainOntology"})
-[:CONTAINS]->(:Term {name: "DomainRelationship"})

(:Taxonomy {name: "Domain Categorization"})
-[:CONTAINS]->(:Category {name: "Primary Domains"})
-[:CONTAINS]->(:Term {name: "InformationDomain"})
-[:CONTAINS]->(:Term {name: "KnowledgeDomain"})
-[:CONTAINS]->(:Term {name: "BusinessDomain"})

(:Taxonomy {name: "Domain Categorization"})
-[:CONTAINS]->(:Category {name: "Support Domains"})
-[:CONTAINS]->(:Term {name: "SecurityDomain"})
-[:CONTAINS]->(:Term {name: "DistributionDomain"})
-[:CONTAINS]->(:Term {name: "PersistenceDomain"})
-[:CONTAINS]->(:Term {name: "OrganizationDomain"})
-[:CONTAINS]->(:Term {name: "AgentDomain"})
-[:CONTAINS]->(:Term {name: "EnvironmentDomain"})
-[:CONTAINS]->(:Term {name: "GovernanceDomain"})
-[:CONTAINS]->(:Term {name: "AIIntegrationDomain"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "Vocabulary navigation and domain discovery"})
-[:APPLIES_TO]->(:VocabularyIndex)
-[:REQUIRES]->(:DomainFile)
-[:PRODUCES]->(:DomainUnderstanding)

(:UsageContext {name: "Domain-specific term definition and standardization"})
-[:APPLIES_TO]->(:DomainFile)
-[:REQUIRES]->(:VocabularyStandard)
-[:PRODUCES]->(:StandardizedTerms)

(:UsageContext {name: "Ensuring consistency across domain vocabularies"})
-[:APPLIES_TO]->(:VocabularyStandard)
-[:REQUIRES]->(:TermStructure)
-[:PRODUCES]->(:ConsistentVocabulary)

(:UsageContext {name: "Term categorization and hierarchical organization"})
-[:APPLIES_TO]->(:DomainTaxonomy)
-[:REQUIRES]->(:TermCategory)
-[:PRODUCES]->(:OrganizedVocabulary)

(:UsageContext {name: "Semantic relationships and reasoning about domains"})
-[:APPLIES_TO]->(:DomainOntology)
-[:REQUIRES]->(:DomainRelationship)
-[:PRODUCES]->(:SemanticUnderstanding)

(:UsageContext {name: "Cross-domain integration and boundary definition"})
-[:APPLIES_TO]->(:DomainRelationship)
-[:REQUIRES]->(:DomainFile)
-[:PRODUCES]->(:IntegratedDomains)
```

## Code References

```cypher
(:CodeBase {path: "vocabulary/domains/index.md"})
-[:IMPLEMENTS]->(:VocabularyIndex)

(:CodeBase {path: "vocabulary/domains/template.md"})
-[:IMPLEMENTS]->(:DomainFile)

(:CodeBase {path: ".cursor/rules/vocabulary.mdc"})
-[:IMPLEMENTS]->(:VocabularyStandard)

(:CodeBase {path: "vocabulary/taxonomies/domain_taxonomy.yaml"})
-[:IMPLEMENTS]->(:DomainTaxonomy)

(:CodeBase {path: "vocabulary/ontologies/cim_ontology.ttl"})
-[:IMPLEMENTS]->(:DomainOntology)

(:CodeBase {path: "vocabulary/knowledge/domain_relationships.md"})
-[:IMPLEMENTS]->(:DomainRelationship)
```

