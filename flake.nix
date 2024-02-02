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
    llm-git = {
      url = "github:rustformers/llm";
      inputs.nixpkgs.follows = "nixpkgs";      
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, llm-git }:  
    flake-utils.lib.eachDefaultSystem 
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
            config.allowUnfree = true;
          };
          stdenv = pkgs.clangStdenv;
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
       in
        with pkgs;
        {
          llm = llm-git.packages."${system}".llm;

          environment.systemPackages = with pkgs; [
              llm
              rust-analyzer
              vscode-extensions.vadimcn.vscode-lldb
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
              tailwindcss
              sass
            ];

          modules = [
            nix-ld.nixosModules.nix-ld
            { programs.nix-ld.dev.enable = true; }
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
              wasm-pack
              nodejs_18.out
              nodePackages.webpack
              nodePackages.webpack-cli
              nodePackages.npm
              sass
              tailwindcss
            ];
            
            RUST_SRC_PATH = rustPlatform.rustLibSrc;

        };
        }
      );
}
