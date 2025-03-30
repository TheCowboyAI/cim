# Cursor Rules Configuration
#
# This file defines the specific cursor rules for the CIM project.
# Each rule is defined as an attribute in the rules set.
{lib, ...}: {
  cursor.rules = {
    enable = true;

    rules = {
      "code-policy" = {
        name = "code-policy";
        description = "General coding rules for maintaining code quality and consistency";
        enabled = true;
        # The content would be the actual content of the .cursor/rules/code-policy.mdc file
        content = ''
          # Code Policy Rules

          These rules define the standard code policies for the CIM project:

          ## Formatting
          - Use consistent indentation (2 spaces for most languages)
          - Limit line length to 100 characters
          - Use meaningful names for variables, functions, and classes
          - Follow language-specific style guides

          ## Documentation
          - Document all public APIs
          - Include usage examples for complex functionality
          - Keep documentation up-to-date with code changes
          - Use standardized doc comments (e.g., rustdoc for Rust)

          ## Best Practices
          - Write comprehensive tests
          - Minimize code duplication
          - Handle errors appropriately
          - Use immutable data structures when possible
          - Limit function and method complexity
        '';
      };

      "design-patterns" = {
        name = "design-patterns";
        description = "Applies general rules across all file types to maintain design patterns, and domain consistency";
        enabled = true;
        content = ''
          # Design Patterns Rules

          - APPROVED design patterns for writing code:
            - SOLID principles
            - DRY principle
            - YAGNI principle
            - KISS principle
            - TDD (Test Driven Development)
            - BDD (Behavior Driven Development)
            - DDD (Domain Driven Design)
            - EDA (Event Driven Architecture)
            - CQRS (Command Query Responsibility Segregation)
            - Reactive Programming
            - Functional Programming
            - Strategy
            - Strategy Pattern
            - Observer Pattern
            - ECS (Entity Component System)
            - TEA (The Elm Architecture)
            - Imperative Style: For low-level control over hardware or performance-critical code.
            - Functional Style: For data transformations using immutability and composable functions.
            - Trait-Oriented Design: A unique feature of Rust that enables polymorphism without inheritance by using traits for shared behavior across types
            - Declarative Style: For describing System Configurations.

          **REJECTED PATTERNS**
          DO NOT USE OTHER PATTERNS INTENTIONALLY
        '';
      };

      "ddd-patterns" = {
        name = "ddd-patterns";
        description = "Rules for implementing Domain-Driven Design patterns in code";
        enabled = true;
        content = ''
          # Domain-Driven Design Rules

          ## Ubiquitous Language
          - Use consistent terminology across code, documentation, and team communication
          - Match class and function names to domain language
          - Reflect domain concepts directly in code structure

          ## Bounded Contexts
          - Define clear boundaries between different domain areas
          - Create explicit context maps for relationships between bounded contexts
          - Use appropriate integration patterns between contexts

          ## Aggregates
          - Design aggregates around invariants and transaction boundaries
          - Keep aggregates small and focused
          - Use aggregate roots to control access to aggregate elements

          ## Entities and Value Objects
          - Distinguish between entities (defined by identity) and value objects (defined by attributes)
          - Make value objects immutable
          - Implement equality for value objects based on all attributes

          ## Repositories
          - Define repository interfaces in the domain layer
          - Keep repository methods focused on domain concepts
          - Return fully-constructed aggregates

          ## Domain Events
          - Use events to represent state changes within the domain
          - Design events as immutable records of what happened
          - Use event sourcing where appropriate

          ## Domain Services
          - Create services for operations that don't belong to a specific entity
          - Keep services stateless
          - Use services to coordinate complex operations across multiple aggregates
        '';
      };

      "project-memory" = {
        name = "project-memory";
        description = "Rules for maintaining project memory bank and documentation structure";
        enabled = true;
        content = ''
          # Project Memory Rules

          ## Documentation Structure
          - Organize documentation by domain concept rather than by file structure
          - Maintain a glossary of domain terms
          - Keep decision records for significant architectural choices
          - Document the "why" behind design decisions

          ## Knowledge Preservation
          - Record exploration paths, even unsuccessful ones
          - Document alternative approaches considered
          - Maintain historical context of system evolution

          ## Accessibility
          - Make documentation easily discoverable
          - Use consistent formatting and structure
          - Link related concepts together
          - Provide multiple entry points based on reader's needs
        '';
      };

      "vocabulary" = {
        name = "vocabulary";
        description = "Rules for maintaining consistent domain vocabulary and ubiquitous language";
        enabled = true;
        content = ''
          # Vocabulary Rules

          ## Term Definition
          - Define all domain terms clearly and precisely
          - Link terms to their domain context
          - Specify relationships between related terms
          - Include examples of proper usage

          ## Consistency
          - Use the same term for the same concept everywhere
          - Avoid synonyms and ambiguous terminology
          - Make distinctions in language when concepts differ
          - Update code when terminology changes

          ## Evolution
          - Track changes to terminology over time
          - Document reasons for vocabulary changes
          - Maintain backward references for historical terms
        '';
      };

      "mcp" = {
        name = "mcp";
        description = "Model Context Protocol (MCP) Rules for Creating and Debugging MCPs";
        enabled = true;
        content = ''
          # MCP Rules

          ## Protocol Design
          - Use consistent message structures
          - Define clear error handling mechanisms
          - Ensure backward compatibility
          - Document all message types and fields

          ## Implementation
          - Handle all error cases explicitly
          - Use strong typing for messages where possible
          - Include request/response correlation IDs
          - Implement proper logging for debugging
        '';
      };

      "research" = {
        name = "research";
        description = "Research methodology and documentation rules";
        enabled = true;
        content = ''
          # Research Rules

          ## Methodology
          - Define clear research questions before beginning
          - Document assumptions and constraints
          - Use rigorous validation methods
          - Cite sources for external information

          ## Documentation
          - Structure research documents consistently
          - Include executive summaries for complex topics
          - Separate findings from interpretations
          - Document future research directions
        '';
      };
    };
  };
}
