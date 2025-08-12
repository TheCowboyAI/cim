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
            # mdbook      # Commented out - build from source if needed
            plantuml
            graphviz
            pandoc
            # asciidoctor # Commented out - use pandoc instead
            
            # Diagram and visualization
            # mermaid-cli  # Has dependency issues
            # drawio       # GUI app, not suitable for shell
            
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
            # docker-compose  # Commented out - requires Docker daemon
            # kubectl         # Commented out - not needed for blueprint
            # k9s            # Commented out - not needed for blueprint
            
            # Development utilities
            direnv
            starship
            bat
            ripgrep
            fd
            
            # Text editors with LSP support
            # zed-editor # Not in nixpkgs stable yet - install separately
            helix        # Terminal editor with LSP (configurable keybindings)
            micro        # Simple terminal editor as fallback
            
            # Language support for examples
            # rustToolchain  # Already defined but not needed for blueprint
            nodejs
            python3
            # go  # Commented out - add if needed
          ];

          shellHook = ''
            echo "╔═══════════════════════════════════════════════════════════════╗"
            echo "║  🧬 Welcome to the CIM Architectural Blueprint Environment  ║"
            echo "╚═══════════════════════════════════════════════════════════════╝"
            echo ""
            echo "This repository serves as the DNA for building domain-specific CIMs."
            echo ""
            echo "📚 Key Resources:"
            echo "  • doc/ - Architectural documentation and design principles"
            echo "  • examples/ - Example CIM assemblies and patterns"
            echo "  • nix/ - Nix expressions for CIM module composition"
            echo "  • .claude/ - AI assistant patterns for CIM development"
            echo ""
            echo "🛠️  Available Tools:"
            echo "  • hx (helix) - Modern terminal editor with LSP"
            echo "  • micro - Simple terminal editor (ctrl-q to quit)"
            echo "  • ./scripts/zed - Launch Zed editor (GUI)"
            echo "  • plantuml - Create architectural diagrams"
            echo "  • nix flake - Explore CIM module flakes"
            echo "  • gh - Browse CIM ecosystem repositories"
            echo ""
            echo "🚀 Getting Started:"
            echo "  1. Review doc/cim_comprehensive_manual.md"
            echo "  2. Explore doc/design/ for architectural patterns"
            echo "  3. Check examples/ for domain implementation examples"
            echo "  4. Visit github.com/thecowboyai/cim-start to begin building"
            echo ""
            echo "Remember: CIM is not just code - it's a way of thinking about"
            echo "         business domains as composable information systems."
            echo ""
            
            # Create helix config directory if it doesn't exist
            mkdir -p ~/.config/helix
            
            # Check if user wants VS Code keybindings for helix
            if [ ! -f ~/.config/helix/config.toml ]; then
              echo ""
              echo "💡 Editor Tips:"
              echo "   • Zed: Use './scripts/zed' or 'nix run nixpkgs#zed-editor'"
              echo "   • For VS Code keybindings in Helix:"
              echo "     cp examples/helix-vscode-config.toml ~/.config/helix/config.toml"
            fi
          '';
        };
      }
    );
}
