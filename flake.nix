{
  description = "CIM Ontology Tool - A suite for extracting, managing, and analyzing ontologies";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
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

        # Define the main package
        cim-ontology-tool = pkgs.rustPlatform.buildRustPackage {
          pname = "cim-ontology-tool";
          version = "0.1.0";

          src = ./cim-ontology-tool;

          cargoLock = {
            lockFile = ./cim-ontology-tool/Cargo.lock;
          };

          buildInputs = with pkgs; [
            openssl
            pkg-config
          ];

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
          ];

          meta = with pkgs.lib; {
            description = "An ontology extraction tool for Composable Information Machines (CIMs)";
            homepage = "https://github.com/thecowboyai/cim";
            license = licenses.mit;
            maintainers = [];
          };
        };
      in {
        # Package outputs
        packages = {
          cim-ontology-tool = cim-ontology-tool;
          default = cim-ontology-tool;
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
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        # Check for CI
        checks = {
          inherit cim-ontology-tool;
        };

        # Make the binary available
        apps.default = flake-utils.lib.mkApp {
          drv = cim-ontology-tool;
        };
      }
    );
}
