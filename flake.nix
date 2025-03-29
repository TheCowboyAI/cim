{
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
        stdenv = pkgs.clangStdenv;
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
        with pkgs; {
          environment.systemPackages = with pkgs; [
            rust-analyzer
            cargo-edit
            cargo-expand
            cargo-udeps
            cargo-whatfeatures
            trunk
            direnv
            zsh
            git
            act
            starship
          ];

          modules = [
            nix-ld.nixosModules.nix-ld
            {programs.nix-ld.dev.enable = true;}
          ];

          devShells.default = mkShell {
            buildInputs = [
              rustToolchain
              cargo-leptos
              cargo-generate
              cargo-make
              cacert
              openssl
              openssl.dev
              pkg-config
              zlib.dev
              nodejs
            ];

            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        }
    );
}
