### Term: Ubiquitous Language
- **Category**: Cross-Cutting
- **Type**: Concept
- **Taxonomy**: Communication
- **Definition**: A shared language between stakeholders and developers that reflects the domain model and is used consistently throughout the system.
- **Relationships**:
  * Part-Of: Domain-Driven Design
  * Used-By: All Domains
  * Defines: Domain Terms
- **Usage Context**: Communication and documentation across all system aspects
- **Code Reference**: `cim/src/common/language`

### Term: Geometric Representation
- **Category**: Cross-Cutting
- **Type**: Concept
- **Taxonomy**: Visualization
- **Definition**: A mathematical representation of domain concepts in a multidimensional space, enabling analysis and comparison.
- **Relationships**:
  * Used-By: Conceptual Space
  * Contains: Quality Dimensions
  * Enables: Data Analysis
- **Usage Context**: Data visualization and analysis across domains
- **Code Reference**: `cim/src/common/geometry`

### Term: Prototype
- **Category**: Cross-Cutting
- **Type**: Value Object
- **Taxonomy**: Classification
- **Definition**: A representative example of a category in a conceptual space, used for classification and comparison.
- **Relationships**:
  * Part-Of: Conceptual Space
  * Defines: Category
  * Used-For: Classification
- **Usage Context**: Category definition and object classification
- **Code Reference**: `cim/src/common/prototype`

### Term: Region
- **Category**: Cross-Cutting
- **Type**: Value Object
- **Taxonomy**: Classification
- **Definition**: A defined area in a conceptual space representing a specific category or concept.
- **Relationships**:
  * Part-Of: Conceptual Space
  * Contains: Prototypes
  * Defines: Categories
- **Usage Context**: Category definition and spatial analysis
- **Code Reference**: `cim/src/common/region` 