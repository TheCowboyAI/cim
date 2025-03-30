# Information Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:InformationEntity:Entity {
  domain: "Information",
  term: "Information Entity",
  definition: "A discrete unit of information that can be stored, retrieved, and processed within the CIM system",
  taxonomy: "Storage Taxonomy",
  usage_context: "Core building block for representing any piece of information in the system",
  code_reference: "TBD"
})

(:InformationState:ValueObject {
  domain: "Information",
  term: "Information State",
  definition: "The current representation and status of an Information Entity within the system",
  taxonomy: "Storage Taxonomy",
  usage_context: "Used to track and manage the lifecycle of information within the system",
  code_reference: "TBD"
})

(:ComposableUnit:Aggregate {
  domain: "Information",
  term: "Composable Unit",
  definition: "A self-contained piece of information that can be combined with other units to form larger information structures",
  taxonomy: "Storage Taxonomy",
  usage_context: "Fundamental building block for creating complex information structures",
  code_reference: "TBD"
})

(:InformationStorage:ValueObject {
  domain: "Information",
  term: "Information Storage",
  definition: "The physical or logical location where information is persistently stored",
  taxonomy: "Storage Taxonomy",
  usage_context: "Represents the actual storage mechanisms used by the system",
  code_reference: "TBD"
})

(:InformationAttribute:ValueObject {
  domain: "Information",
  term: "Information Attribute",
  definition: "A property or characteristic that describes an aspect of an Information Entity",
  taxonomy: "Information Structure",
  usage_context: "Defining and categorizing the properties of information entities",
  code_reference: "TBD"
})

(:InformationIntegrity:ValueObject {
  domain: "Information",
  term: "Information Integrity",
  definition: "The assurance that information has not been altered in an unauthorized manner",
  taxonomy: "Information Quality",
  usage_context: "Ensuring the accuracy and consistency of information",
  code_reference: "TBD"
})

(:StateChange:Event {
  domain: "Information",
  term: "State Change",
  definition: "An event that represents a transition in the state of an Information Entity",
  taxonomy: "Information Lifecycle",
  usage_context: "Tracking and responding to changes in information state",
  code_reference: "TBD"
})

(:InformationCapability:Service {
  domain: "Information",
  term: "Information Capability",
  definition: "A functional ability that can be applied to or used with Information Entities",
  taxonomy: "Information Processing",
  usage_context: "Defining the operations that can be performed on information",
  code_reference: "TBD"
})

(:UnitComposition:Service {
  domain: "Information",
  term: "Unit Composition",
  definition: "The process and rules for combining Composable Units into larger structures",
  taxonomy: "Information Structure",
  usage_context: "Creating complex information structures from simpler components",
  code_reference: "TBD"
})

(:PersistenceLayer:Service {
  domain: "Information",
  term: "Persistence Layer",
  definition: "System component responsible for durable storage of information",
  taxonomy: "Storage Taxonomy",
  usage_context: "Handling all aspects of permanent information storage",
  code_reference: "TBD"
})

(:StorageLayout:ValueObject {
  domain: "Information",
  term: "Storage Layout",
  definition: "The physical or logical organization of information within a storage system",
  taxonomy: "Storage Structure",
  usage_context: "Organizing information for efficient access and management",
  code_reference: "TBD"
})
```

## Relationships

```cypher
// Information Entity relationships
(:InformationEntity)-[:CONTAINS {type: "property"}]->(:InformationAttribute)
(:InformationEntity)-[:IS_A {type: "inheritance"}]->(:ComposableUnit)
(:InformationEntity)-[:MANAGES {type: "lifecycle"}]->(:InformationState)

// Information State relationships
(:InformationState)-[:PART_OF {type: "component"}]->(:InformationEntity)
(:InformationState)-[:VALIDATES {type: "quality"}]->(:InformationIntegrity)
(:InformationState)-[:TRIGGERS {type: "event"}]->(:StateChange)

// Composable Unit relationships
(:ComposableUnit)-[:CONTAINS {type: "structure"}]->(:InformationEntity)
(:ComposableUnit)-[:EXTENDS {type: "capability"}]->(:InformationCapability)
(:ComposableUnit)-[:CONFIGURES {type: "process"}]->(:UnitComposition)

// Information Storage relationships
(:InformationStorage)-[:PART_OF {type: "component"}]->(:PersistenceLayer)
(:InformationStorage)-[:CONTAINS {type: "content"}]->(:InformationEntity)
(:InformationStorage)-[:CONFIGURES {type: "structure"}]->(:StorageLayout)

// Information Attribute relationships
(:InformationAttribute)-[:PART_OF {type: "property"}]->(:InformationEntity)
(:InformationAttribute)-[:DEFINES {type: "characteristic"}]->(:InformationType)
(:InformationAttribute)-[:USED_BY {type: "process"}]->(:InformationProcessing)

// Information Integrity relationships
(:InformationIntegrity)-[:VALIDATED_BY {type: "process"}]->(:InformationState)
(:InformationIntegrity)-[:APPLIES_TO {type: "target"}]->(:InformationEntity)
(:InformationIntegrity)-[:ENFORCED_BY {type: "mechanism"}]->(:SecurityControl)

// State Change relationships
(:StateChange)-[:TRIGGERED_BY {type: "source"}]->(:InformationState)
(:StateChange)-[:AFFECTS {type: "target"}]->(:InformationEntity)
(:StateChange)-[:RECORDED_IN {type: "log"}]->(:ChangeHistory)

// Information Capability relationships
(:InformationCapability)-[:EXTENDED_BY {type: "enhancement"}]->(:ComposableUnit)
(:InformationCapability)-[:APPLIES_TO {type: "target"}]->(:InformationEntity)
(:InformationCapability)-[:IMPLEMENTS {type: "functionality"}]->(:InformationProcessing)

// Unit Composition relationships
(:UnitComposition)-[:CONFIGURED_BY {type: "source"}]->(:ComposableUnit)
(:UnitComposition)-[:CREATES {type: "result"}]->(:CompositeInformation)
(:UnitComposition)-[:FOLLOWS {type: "guideline"}]->(:CompositionRule)

// Persistence Layer relationships
(:PersistenceLayer)-[:CONTAINS {type: "component"}]->(:InformationStorage)
(:PersistenceLayer)-[:PROVIDES {type: "service"}]->(:PersistenceService)
(:PersistenceLayer)-[:MANAGES {type: "resource"}]->(:StorageResource)

// Storage Layout relationships
(:StorageLayout)-[:CONFIGURED_BY {type: "source"}]->(:InformationStorage)
(:StorageLayout)-[:ORGANIZES {type: "target"}]->(:InformationEntity)
(:StorageLayout)-[:OPTIMIZES {type: "goal"}]->(:AccessEfficiency)
```

## Taxonomies

### Information Structure Taxonomy

```cypher
(:Taxonomy {name: "Information Structure Taxonomy"})
-[:CONTAINS]->(:Category {name: "Information Components"})
-[:CONTAINS]->(:Category {name: "Information Properties"})
-[:CONTAINS]->(:Category {name: "Information Compositions"})

(:Category {name: "Information Components"})
-[:CONTAINS]->(:Term {name: "Information Entity"})
-[:CONTAINS]->(:Term {name: "Composable Unit"})
-[:CONTAINS]->(:Term {name: "Information Attribute"})

(:Category {name: "Information Properties"})
-[:CONTAINS]->(:Term {name: "Information State"})
-[:CONTAINS]->(:Term {name: "Information Type"})
-[:CONTAINS]->(:Term {name: "Information Integrity"})

(:Category {name: "Information Compositions"})
-[:CONTAINS]->(:Term {name: "Unit Composition"})
-[:CONTAINS]->(:Term {name: "Composite Information"})
-[:CONTAINS]->(:Term {name: "Composition Rule"})
```

### Storage Taxonomy

```cypher
(:Taxonomy {name: "Storage Taxonomy"})
-[:CONTAINS]->(:Category {name: "Storage Components"})
-[:CONTAINS]->(:Category {name: "Storage Structures"})
-[:CONTAINS]->(:Category {name: "Storage Services"})

(:Category {name: "Storage Components"})
-[:CONTAINS]->(:Term {name: "Information Storage"})
-[:CONTAINS]->(:Term {name: "Persistence Layer"})
-[:CONTAINS]->(:Term {name: "Storage Resource"})

(:Category {name: "Storage Structures"})
-[:CONTAINS]->(:Term {name: "Storage Layout"})
-[:CONTAINS]->(:Term {name: "Access Efficiency"})
-[:CONTAINS]->(:Term {name: "Storage Pattern"})

(:Category {name: "Storage Services"})
-[:CONTAINS]->(:Term {name: "Persistence Service"})
-[:CONTAINS]->(:Term {name: "Storage Management"})
-[:CONTAINS]->(:Term {name: "Storage Optimization"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "Information Representation"})
-[:APPLIES_TO]->(:InformationEntity)
-[:REQUIRES]->(:InformationAttribute)
-[:PRODUCES]->(:StructuredInformation)

(:UsageContext {name: "Information Lifecycle"})
-[:APPLIES_TO]->(:InformationState)
-[:REQUIRES]->(:StateChange)
-[:PRODUCES]->(:ManagedLifecycle)

(:UsageContext {name: "Information Composition"})
-[:APPLIES_TO]->(:ComposableUnit)
-[:REQUIRES]->(:UnitComposition)
-[:PRODUCES]->(:CompositeInformation)

(:UsageContext {name: "Information Storage"})
-[:APPLIES_TO]->(:InformationStorage)
-[:REQUIRES]->(:PersistenceLayer)
-[:PRODUCES]->(:PersistedInformation)

(:UsageContext {name: "Information Quality"})
-[:APPLIES_TO]->(:InformationIntegrity)
-[:REQUIRES]->(:ValidationProcess)
-[:PRODUCES]->(:VerifiedInformation)
```

## Code References

```cypher
(:CodeBase {path: "cim/src/information/entity"})
-[:IMPLEMENTS]->(:InformationEntity)

(:CodeBase {path: "cim/src/information/state"})
-[:IMPLEMENTS]->(:InformationState)

(:CodeBase {path: "cim/src/information/unit"})
-[:IMPLEMENTS]->(:ComposableUnit)

(:CodeBase {path: "cim/src/information/storage"})
-[:IMPLEMENTS]->(:InformationStorage)

(:CodeBase {path: "cim/src/information/attribute"})
-[:IMPLEMENTS]->(:InformationAttribute)

(:CodeBase {path: "cim/src/information/integrity"})
-[:IMPLEMENTS]->(:InformationIntegrity)

(:CodeBase {path: "cim/src/information/event"})
-[:IMPLEMENTS]->(:StateChange)

(:CodeBase {path: "cim/src/information/capability"})
-[:IMPLEMENTS]->(:InformationCapability)

(:CodeBase {path: "cim/src/information/composition"})
-[:IMPLEMENTS]->(:UnitComposition)

(:CodeBase {path: "cim/src/information/persistence"})
-[:IMPLEMENTS]->(:PersistenceLayer)

(:CodeBase {path: "cim/src/information/layout"})
-[:IMPLEMENTS]->(:StorageLayout)
```

