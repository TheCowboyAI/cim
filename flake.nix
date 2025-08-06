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
    # Standalone CIM modules (commented until they have flake.nix)
    # cim-network = {
    #   url = "github:TheCowboyAI/cim-network";
    #   inputs.nixpkgs.follows = "nixpkgs";
    # };
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
        # Package outputs (simplified for now)
        packages = {
          # Will add packages once modules are ready
        };
        
        # Container images (disabled for now)
        # containers = import ./nix/containers { inherit pkgs lib; };
        
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
            
            # CIM domain tools (add when their builds are fixed)
            # cim-domain-nix.packages.${system}.default
            # cim-domain-git.packages.${system}.default
            
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
      # Flake-level attributes (simplified for now)
      # nixosConfigurations = { };
      # templates = { };
    };
}
