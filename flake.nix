{
  description = "CIM - The Composable Information Machine";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # CIM domain modules
    cim-domain-nix = {
      url = "github:thecowboyai/cim-domain-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cim-domain-git = {
      url = "github:thecowboyai/cim-domain-git";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # Standalone CIM modules
    cim-network = {
      url = "github:TheCowboyAI/cim-network";
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
    cim-network,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        
        # Import CIM packages
        cimPackages = import ./nix/packages { inherit pkgs lib; };

      in {
        # Package outputs
        packages = cimPackages // {
          default = cimPackages.cim-all;
          
          # Re-export domain packages
          inherit (cim-domain-nix.packages.${system}) cim-domain-nix;
          inherit (cim-domain-git.packages.${system}) cim-domain-git;
          inherit (cim-network.packages.${system}) cim-network;
        };
        
        # Container images
        containers = import ./nix/containers { inherit pkgs lib; };
        
        # NixOS Module
        nixosModules = {
          default = import ./nix/modules;
          cim-events = import ./nix/modules/cim-events.nix;
          cim-projections = import ./nix/modules/cim-projections.nix;
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            cargo-edit
            cargo-expand
            cargo-udeps
            cargo-whatfeatures
            openssl
            openssl.dev
            pkg-config
            zlib.dev
            nodejs
            direnv
            zsh
            git
            act
            starship
            
            # CIM domain tools
            cim-domain-nix.packages.${system}.default
            cim-domain-git.packages.${system}.default
            
            # Additional Nix tools
            nix-prefetch-git
            nixpkgs-fmt
            nixos-generators
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          
          shellHook = ''
            echo "🚀 Welcome to CIM development environment!"
            echo ""
            echo "Available commands:"
            echo "  nix build          - Build all CIM packages"
            echo "  nix run .#<pkg>    - Run a specific package"
            echo "  nix develop        - Enter development shell"
            echo ""
            echo "Nix integration available via:"
            echo "  cim-domain-nix     - Nix domain modeling"
            echo "  cim-domain-git     - Git integration"
            echo ""
          '';
        };
        
        # Additional development shells
        devShells.integration = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            # Integration test dependencies
            nats-server
            postgresql
            redis
            docker-compose
          ];
          
          shellHook = ''
            echo "Integration testing environment"
            echo "Starting local services..."
            docker-compose -f tests/docker-compose.yml up -d
          '';
        };
      }
    )
    // {
      # Flake-level attributes
      nixosConfigurations = {
        # Development VM
        cim-dev = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            self.nixosModules.default
            ./nix/environments/development.nix
          ];
        };
        
        # Production configuration template
        cim-prod = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            self.nixosModules.default
            ./nix/environments/production.nix
          ];
        };
      };
      
      # Deployment templates
      templates = {
        cim-module = {
          path = ./templates/cim-module;
          description = "Template for new CIM modules";
        };
      };
    };
}
