# Organizations Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:Organization:Aggregate {
  domain: "Business",
  definition: "A structured entity with defined roles, responsibilities, and relationships within the system"
})

(:Role:ValueObject {
  domain: "Business",
  definition: "A defined set of responsibilities and permissions within an organization"
})

(:Team:Aggregate {
  domain: "Business",
  definition: "A group of members working together towards common objectives"
})

(:Member:Entity {
  domain: "Business",
  definition: "An individual or agent associated with an organization"
})

(:Responsibility:ValueObject {
  domain: "Business",
  definition: "A specific duty or obligation assigned to a role or team"
})

(:Communication:Service {
  domain: "Business",
  definition: "The system facilitating information exchange within and between organizations"
})
```

## Relationships

```cypher
// Organization relationships
(:Organization)-[:DEFINES {type: "structure"}]->(:Role)
(:Organization)-[:CONTAINS {type: "unit"}]->(:Team)
(:Organization)-[:INCLUDES {type: "member"}]->(:Member)
(:Organization)-[:ASSIGNS {type: "duty"}]->(:Responsibility)
(:Organization)-[:USES {type: "service"}]->(:Communication)

// Role relationships
(:Role)-[:BELONGS_TO {type: "structure"}]->(:Organization)
(:Role)-[:ASSIGNED_TO {type: "member"}]->(:Member)
(:Role)-[:HAS {type: "duty"}]->(:Responsibility)
(:Role)-[:REQUIRES {type: "permission"}]->(:Permission)

// Team relationships
(:Team)-[:PART_OF {type: "unit"}]->(:Organization)
(:Team)-[:INCLUDES {type: "member"}]->(:Member)
(:Team)-[:RESPONSIBLE_FOR {type: "duty"}]->(:Responsibility)
(:Team)-[:USES {type: "service"}]->(:Communication)

// Member relationships
(:Member)-[:BELONGS_TO {type: "member"}]->(:Organization)
(:Member)-[:HAS {type: "role"}]->(:Role)
(:Member)-[:PART_OF {type: "unit"}]->(:Team)
(:Member)-[:COMMUNICATES_VIA {type: "service"}]->(:Communication)
```

## Taxonomies

### Organization Processing

```cypher
(:Taxonomy {name: "OrganizationProcessing"})
-[:CONTAINS]->(:Category {name: "OrganizationOperations"})
-[:CONTAINS]->(:Operation {name: "RoleManagement"})
-[:CONTAINS]->(:Operation {name: "TeamCoordination"})
-[:CONTAINS]->(:Operation {name: "MemberAdministration"})

(:Category {name: "CommunicationManagement"})
-[:CONTAINS]->(:Operation {name: "InternalCommunication"})
-[:CONTAINS]->(:Operation {name: "ExternalCommunication"})
-[:CONTAINS]->(:Operation {name: "CollaborationTools"})

(:Category {name: "ResponsibilityManagement"})
-[:CONTAINS]->(:Operation {name: "DutyAssignment"})
-[:CONTAINS]->(:Operation {name: "TaskTracking"})
-[:CONTAINS]->(:Operation {name: "PerformanceMonitoring"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "OrganizationManagement"})
-[:APPLIES_TO]->(:Organization)
-[:REQUIRES]->(:Role)
-[:PRODUCES]->(:Structure)

(:UsageContext {name: "TeamCollaboration"})
-[:APPLIES_TO]->(:Team)
-[:REQUIRES]->(:Communication)
-[:PRODUCES]->(:Output)

(:UsageContext {name: "MemberEngagement"})
-[:APPLIES_TO]->(:Member)
-[:REQUIRES]->(:Role)
-[:PRODUCES]->(:Performance)
```

## Code References

```cypher
(:CodeBase {path: "notes/organizations/readme.md"})
-[:IMPLEMENTS]->(:Organization)

(:CodeBase {path: "notes/organizations/roles.md"})
-[:IMPLEMENTS]->(:Role)

(:CodeBase {path: "notes/organizations/teams.md"})
-[:IMPLEMENTS]->(:Team)

(:CodeBase {path: "notes/organizations/communication.md"})
-[:IMPLEMENTS]->(:Communication)
``` 