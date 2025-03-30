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
        content = builtins.readFile ../../.cursor/rules/code-policy.mdc;
      };

      "design-patterns" = {
        name = "design-patterns";
        description = "Applies general rules across all file types to maintain design patterns, and domain consistency";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/design-patterns.mdc;
      };

      "ddd-patterns" = {
        name = "ddd-patterns";
        description = "Rules for implementing Domain-Driven Design patterns in code";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/ddd-patterns.mdc;
      };

      "project-memory" = {
        name = "project-memory";
        description = "Rules for maintaining project memory bank and documentation structure";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/project-memory.mdc;
      };

      "vocabulary" = {
        name = "vocabulary";
        description = "Rules for maintaining consistent domain vocabulary and ubiquitous language";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/vocabulary.mdc;
      };

      "mcp" = {
        name = "mcp";
        description = "Model Context Protocol (MCP) Rules for Creating and Debugging MCPs";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/mcp.mdc;
      };

      "research" = {
        name = "research";
        description = "Research methodology and documentation rules";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/research.mdc;
      };

      "nixmcp" = {
        name = "nixmcp";
        description = "NixMCP Rules for NixOS Package and Option Lookups";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/nixmcp.mdc;
      };

      "nix-flakes" = {
        name = "nix-flakes";
        description = "Rules for implementing Nix flakes with best practices";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/nix-flakes.mdc;
      };

      "nixos" = {
        name = "nixos";
        description = "Nixos Foundational Rules";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/nixos.mdc;
      };

      "rust" = {
        name = "rust";
        description = "Rules for Rust projects and toolchain configuration";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/rust.mdc;
      };

      "ruler" = {
        name = "ruler";
        description = "Rules and patterns for creating Cursor rules";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/ruler.mdc;
      };

      "mermaid" = {
        name = "mermaid";
        description = "Rules for creating and using Mermaid diagrams";
        enabled = true;
        content = builtins.readFile ../../.cursor/rules/mermaid.mdc;
      };
    };
  };
}
