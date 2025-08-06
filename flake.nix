{
  description = "CIM - The Composable Information Machine: Architectural Blueprint & Assembly Guide for Domain-Specific Business Systems";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # CIM ecosystem references - these are examples of CIM modules
    # that can be composed together to build domain-specific systems
    cim-domain-nix = {
      url = "github:thecowboyai/cim-domain-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cim-domain-git = {
      url = "github:thecowboyai/cim-domain-git";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    cim-domain-nix,
    cim-domain-git,
    # cim-network,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
        
        lib = pkgs.lib;

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      in {

        # Development shell for CIM research, design, and assembly
        devShells.default = pkgs.mkShell {
          name = "cim-architect";
          
          buildInputs = with pkgs; [
            # Documentation and design tools
            mdbook
            plantuml
            graphviz
            pandoc
            asciidoctor
            
            # Diagram and visualization
            mermaid-cli
            drawio
            
            # Nix ecosystem tools for CIM assembly
            nix
            nixpkgs-fmt
            nixos-generators
            nix-prefetch-git
            nix-tree
            nix-diff
            
            # Git for version control
            git
            gh
            
            # JSON/YAML tools for configuration
            jq
            yq
            
            # Container tools for deployment strategies
            docker-compose
            kubectl
            k9s
            
            # Development utilities
            direnv
            starship
            bat
            ripgrep
            fd
            
            # Language support for examples
            rustToolchain
            nodejs
            python3
            go
          ];

          shellHook = ''
            ${"\033[1;36m"}╔═══════════════════════════════════════════════════════════════╗${"\033[0m"}
            ${"\033[1;36m"}║${"\033[0m"}  ${"\033[1;33m"}🧬 Welcome to the CIM Architectural Blueprint Environment${"\033[0m"}  ${"\033[1;36m"}║${"\033[0m"}
            ${"\033[1;36m"}╚═══════════════════════════════════════════════════════════════╝${"\033[0m"}
            
            ${"\033[1;32m"}This repository serves as the DNA for building domain-specific CIMs.${"\033[0m"}
            
            ${"\033[1;34m"}📚 Key Resources:${"\033[0m"}
              • ${"\033[0;36m"}doc/${"\033[0m"} - Architectural documentation and design principles
              • ${"\033[0;36m"}examples/${"\033[0m"} - Example CIM assemblies and patterns
              • ${"\033[0;36m"}nix/${"\033[0m"} - Nix expressions for CIM module composition
              • ${"\033[0;36m"}.claude/${"\033[0m"} - AI assistant patterns for CIM development
            
            ${"\033[1;34m"}🛠️  Available Tools:${"\033[0m"}
              • ${"\033[0;33m"}mdbook${"\033[0m"} - Build the CIM handbook
              • ${"\033[0;33m"}plantuml${"\033[0m"} - Create architectural diagrams
              • ${"\033[0;33m"}nix flake${"\033[0m"} - Explore CIM module flakes
              • ${"\033[0;33m"}gh${"\033[0m"} - Browse CIM ecosystem repositories
            
            ${"\033[1;34m"}🚀 Getting Started:${"\033[0m"}
              1. Review ${"\033[0;36m"}doc/cim_comprehensive_manual.md${"\033[0m"}
              2. Explore ${"\033[0;36m"}doc/design/${"\033[0m"} for architectural patterns
              3. Check ${"\033[0;36m"}examples/${"\033[0m"} for domain implementation examples
              4. Visit ${"\033[0;36m"}github.com/thecowboyai/cim-start${"\033[0m"} to begin building
            
            ${"\033[1;35m"}Remember: CIM is not just code - it's a way of thinking about${"\033[0m"}
            ${"\033[1;35m"}         business domains as composable information systems.${"\033[0m"}
          '';
        };
      }
    );
}
