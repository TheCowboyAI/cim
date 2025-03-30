# Business Value Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:ValueProposition:Aggregate {
  domain: "Business",
  term: "Value Proposition",
  definition: "A bundle of products and services that create value for specific customer segments by addressing their needs and pain points.",
  taxonomy: "Business Value",
  usage_context: "Business strategy and customer value delivery",
  code_reference: "cim/src/business/value"
})

(:CustomerSegment:Entity {
  domain: "Business",
  term: "Customer Segment",
  definition: "A distinct group of customers with common characteristics, needs, and behaviors that a business targets with specific value propositions.",
  taxonomy: "Business Value",
  usage_context: "Market targeting and customer analysis",
  code_reference: "cim/src/business/segment"
})

(:CustomerProfile:ValueObject {
  domain: "Business",
  term: "Customer Profile",
  definition: "A structured representation of a customer segment's jobs, pains, and gains that guides value proposition design.",
  taxonomy: "Business Value",
  usage_context: "Customer understanding and value design",
  code_reference: "cim/src/business/profile"
})

(:ValueMap:ValueObject {
  domain: "Business",
  term: "Value Map",
  definition: "A structured representation of how products and services create value by addressing customer pains and enabling gains.",
  taxonomy: "Business Value",
  usage_context: "Value proposition design and validation",
  code_reference: "cim/src/business/map"
})

(:Inventory:Aggregate {
  domain: "Business",
  term: "Inventory",
  definition: "A comprehensive catalog of all resources, configurations, and language elements available to the Domain.",
  taxonomy: "Resource Management",
  usage_context: "Resource management and tracking",
  code_reference: "cim/src/inventory"
})

(:Category:ValueObject {
  domain: "Business",
  term: "Category",
  definition: "A classification unit in Category Theory that helps organize and understand relationships between domain concepts.",
  taxonomy: "Resource Management",
  usage_context: "Domain organization and categorization",
  code_reference: "cim/src/inventory/category"
})

(:Resource:Entity {
  domain: "Business",
  term: "Resource",
  definition: "Any asset, capability, or component that contributes to the functioning of the Domain.",
  taxonomy: "Resource Management",
  usage_context: "Resource allocation and management",
  code_reference: "cim/src/inventory/resource"
})

(:ContentAddress:ValueObject {
  domain: "Business",
  term: "Content Address",
  definition: "A unique identifier for content based on its hash, providing immutable reference and version control.",
  taxonomy: "Resource Management",
  usage_context: "Content management and versioning",
  code_reference: "cim/src/inventory/content"
})

(:Shape:ValueObject {
  domain: "Business",
  term: "Shape",
  definition: "The structural form of a resource or concept defined by its Category and relationships.",
  taxonomy: "Resource Management",
  usage_context: "Resource structure and organization",
  code_reference: "cim/src/inventory/shape"
})

(:ResourceManagement:Service {
  domain: "Business",
  term: "ResourceManagement",
  definition: "A system service responsible for managing, allocating, and tracking resources across the domain.",
  taxonomy: "Resource Management",
  usage_context: "Resource lifecycle management",
  code_reference: "cim/src/business/resource_management"
})

(:CustomerSegmentation:Service {
  domain: "Business",
  term: "CustomerSegmentation",
  definition: "A service that analyzes and categorizes customers into distinct segments based on common characteristics and needs.",
  taxonomy: "Business Value",
  usage_context: "Customer analysis and targeting",
  code_reference: "cim/src/business/segmentation"
})

(:InventoryManagement:Service {
  domain: "Business",
  term: "InventoryManagement",
  definition: "A service that maintains and manages the comprehensive catalog of domain resources and their relationships.",
  taxonomy: "Resource Management",
  usage_context: "Inventory tracking and management",
  code_reference: "cim/src/business/inventory_management"
})
```

## Relationships

```cypher
// Value Proposition relationships
(:ValueProposition)-[:CONTAINS {type: "offering"}]->(:Product)
(:ValueProposition)-[:CONTAINS {type: "offering"}]->(:Service)
(:ValueProposition)-[:TARGETS {type: "audience"}]->(:CustomerSegment)
(:ValueProposition)-[:ADDRESSES {type: "solution"}]->(:CustomerNeed)

// Customer Segment relationships
(:CustomerSegment)-[:HAS {type: "component"}]->(:CustomerProfile)
(:CustomerSegment)-[:RECEIVES {type: "offering"}]->(:ValueProposition)
(:CustomerSegment)-[:CONTAINS {type: "need"}]->(:CustomerJob)

// Customer Profile relationships
(:CustomerProfile)-[:PART_OF {type: "component"}]->(:CustomerSegment)
(:CustomerProfile)-[:CONTAINS {type: "element"}]->(:Job)
(:CustomerProfile)-[:CONTAINS {type: "element"}]->(:Pain)
(:CustomerProfile)-[:CONTAINS {type: "element"}]->(:Gain)
(:CustomerProfile)-[:GUIDES {type: "design"}]->(:ValueMap)

// Value Map relationships
(:ValueMap)-[:CONTAINS {type: "element"}]->(:PainReliever)
(:ValueMap)-[:CONTAINS {type: "element"}]->(:GainCreator)
(:ValueMap)-[:MAPS_TO {type: "alignment"}]->(:CustomerProfile)
(:ValueMap)-[:PART_OF {type: "component"}]->(:ValueProposition)

// Inventory relationships
(:Inventory)-[:CONTAINS {type: "asset"}]->(:Resource)
(:Inventory)-[:CONTAINS {type: "settings"}]->(:Configuration)
(:Inventory)-[:MANAGES {type: "asset"}]->(:DomainAsset)
(:Inventory)-[:IMPLEMENTS {type: "function"}]->(:ResourceTracking)

// Category relationships
(:Category)-[:CONTAINS {type: "element"}]->(:Object)
(:Category)-[:CONTAINS {type: "relation"}]->(:Morphism)
(:Category)-[:PART_OF {type: "component"}]->(:Inventory)
(:Category)-[:IMPLEMENTS {type: "function"}]->(:Classification)

// Resource relationships
(:Resource)-[:PART_OF {type: "component"}]->(:Inventory)
(:Resource)-[:HAS {type: "settings"}]->(:Configuration)
(:Resource)-[:USED_BY {type: "consumer"}]->(:Domain)

// Content Address relationships
(:ContentAddress)-[:IDENTIFIES {type: "reference"}]->(:Content)
(:ContentAddress)-[:USED_BY {type: "system"}]->(:ObjectStore)
(:ContentAddress)-[:ENABLES {type: "feature"}]->(:VersionControl)

// Shape relationships
(:Shape)-[:DEFINED_BY {type: "structure"}]->(:Category)
(:Shape)-[:PART_OF {type: "component"}]->(:Resource)
(:Shape)-[:GUIDES {type: "function"}]->(:Organization)

// Resource Management relationships
(:ResourceManagement)-[:MANAGES {type: "asset"}]->(:Resource)
(:ResourceManagement)-[:MANAGES {type: "catalog"}]->(:Inventory)
(:ResourceManagement)-[:IMPLEMENTS {type: "function"}]->(:ResourceAllocation)
(:ResourceManagement)-[:USES {type: "tool"}]->(:ContentAddress)

// Customer Segmentation relationships
(:CustomerSegmentation)-[:CREATES {type: "output"}]->(:CustomerSegment)
(:CustomerSegmentation)-[:USES {type: "input"}]->(:CustomerProfile)
(:CustomerSegmentation)-[:IMPLEMENTS {type: "function"}]->(:MarketAnalysis)

// Inventory Management relationships
(:InventoryManagement)-[:MANAGES {type: "catalog"}]->(:Inventory)
(:InventoryManagement)-[:MANAGES {type: "structure"}]->(:Category)
(:InventoryManagement)-[:IMPLEMENTS {type: "function"}]->(:ResourceTracking)
(:InventoryManagement)-[:USES {type: "tool"}]->(:Shape)
```

## Taxonomies

### Business Value Taxonomy

```cypher
(:Taxonomy {name: "Business Value Taxonomy"})
-[:CONTAINS]->(:Category {name: "Customer Understanding"})
-[:CONTAINS]->(:Category {name: "Value Creation"})
-[:CONTAINS]->(:Category {name: "Market Analysis"})

(:Category {name: "Customer Understanding"})
-[:CONTAINS]->(:Term {name: "Customer Segment"})
-[:CONTAINS]->(:Term {name: "Customer Profile"})
-[:CONTAINS]->(:Term {name: "CustomerSegmentation"})

(:Category {name: "Value Creation"})
-[:CONTAINS]->(:Term {name: "Value Proposition"})
-[:CONTAINS]->(:Term {name: "Value Map"})
-[:CONTAINS]->(:Term {name: "Pain Reliever"})
-[:CONTAINS]->(:Term {name: "Gain Creator"})

(:Category {name: "Market Analysis"})
-[:CONTAINS]->(:Term {name: "Customer Job"})
-[:CONTAINS]->(:Term {name: "Customer Need"})
-[:CONTAINS]->(:Term {name: "Market Segment"})
```

### Resource Management Taxonomy

```cypher
(:Taxonomy {name: "Resource Management Taxonomy"})
-[:CONTAINS]->(:Category {name: "Resource Cataloging"})
-[:CONTAINS]->(:Category {name: "Resource Structure"})
-[:CONTAINS]->(:Category {name: "Resource Services"})

(:Category {name: "Resource Cataloging"})
-[:CONTAINS]->(:Term {name: "Inventory"})
-[:CONTAINS]->(:Term {name: "Resource"})
-[:CONTAINS]->(:Term {name: "Content Address"})

(:Category {name: "Resource Structure"})
-[:CONTAINS]->(:Term {name: "Category"})
-[:CONTAINS]->(:Term {name: "Shape"})
-[:CONTAINS]->(:Term {name: "Configuration"})

(:Category {name: "Resource Services"})
-[:CONTAINS]->(:Term {name: "ResourceManagement"})
-[:CONTAINS]->(:Term {name: "InventoryManagement"})
-[:CONTAINS]->(:Term {name: "Resource Tracking"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "Business Strategy"})
-[:APPLIES_TO]->(:ValueProposition)
-[:REQUIRES]->(:CustomerSegment)
-[:PRODUCES]->(:BusinessValue)

(:UsageContext {name: "Customer Analysis"})
-[:APPLIES_TO]->(:CustomerSegmentation)
-[:REQUIRES]->(:CustomerProfile)
-[:PRODUCES]->(:CustomerSegment)

(:UsageContext {name: "Resource Management"})
-[:APPLIES_TO]->(:ResourceManagement)
-[:REQUIRES]->(:Inventory)
-[:PRODUCES]->(:ResourceAllocation)

(:UsageContext {name: "Inventory Tracking"})
-[:APPLIES_TO]->(:InventoryManagement)
-[:REQUIRES]->(:Category)
-[:PRODUCES]->(:ResourceTracking)

(:UsageContext {name: "Value Design"})
-[:APPLIES_TO]->(:ValueMap)
-[:REQUIRES]->(:CustomerProfile)
-[:PRODUCES]->(:ValueProposition)
```

## Code References

```cypher
(:CodeBase {path: "cim/src/business/value"})
-[:IMPLEMENTS]->(:ValueProposition)

(:CodeBase {path: "cim/src/business/segment"})
-[:IMPLEMENTS]->(:CustomerSegment)

(:CodeBase {path: "cim/src/business/profile"})
-[:IMPLEMENTS]->(:CustomerProfile)

(:CodeBase {path: "cim/src/business/map"})
-[:IMPLEMENTS]->(:ValueMap)

(:CodeBase {path: "cim/src/inventory"})
-[:IMPLEMENTS]->(:Inventory)

(:CodeBase {path: "cim/src/inventory/category"})
-[:IMPLEMENTS]->(:Category)

(:CodeBase {path: "cim/src/inventory/resource"})
-[:IMPLEMENTS]->(:Resource)

(:CodeBase {path: "cim/src/inventory/content"})
-[:IMPLEMENTS]->(:ContentAddress)

(:CodeBase {path: "cim/src/inventory/shape"})
-[:IMPLEMENTS]->(:Shape)

(:CodeBase {path: "cim/src/business/resource_management"})
-[:IMPLEMENTS]->(:ResourceManagement)

(:CodeBase {path: "cim/src/business/segmentation"})
-[:IMPLEMENTS]->(:CustomerSegmentation)

(:CodeBase {path: "cim/src/business/inventory_management"})
-[:IMPLEMENTS]->(:InventoryManagement)
```

