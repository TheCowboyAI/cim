# Common Nix functions and utilities for CIM
{ lib, pkgs, ... }:

rec {
  # Create a CIM module package
  mkCimPackage = {
    name,
    src,
    cargoToml ? src + "/Cargo.toml",
    buildInputs ? [],
    nativeBuildInputs ? [],
    features ? [],
    ...
  }@args: 
    let
      manifest = builtins.fromTOML (builtins.readFile cargoToml);
      version = manifest.package.version or "0.1.0";
    in
    pkgs.rustPlatform.buildRustPackage (lib.recursiveUpdate {
      pname = name;
      inherit version src;
      
      nativeBuildInputs = [ pkgs.pkg-config ] ++ nativeBuildInputs;
      buildInputs = [ pkgs.openssl ] ++ buildInputs;
      
      buildFeatures = features;
      
      # For development, use fake hash
      cargoSha256 = lib.fakeSha256;
      
      meta = with lib; {
        description = manifest.package.description or "A CIM module";
        homepage = manifest.package.repository or "https://github.com/thecowboyai/cim";
        license = licenses.asl20;
        maintainers = [ ];
      };
    } (removeAttrs args [ "name" "features" ]));

  # Create a CIM development shell
  mkCimShell = {
    name ? "cim-dev",
    modules ? [],
    extraPackages ? [],
    extraShellHook ? "",
    ...
  }@args:
    pkgs.mkShell {
      name = "${name}-shell";
      
      buildInputs = with pkgs; [
        # Rust toolchain
        rustc
        cargo
        rustfmt
        rust-analyzer
        clippy
        
        # Build dependencies
        pkg-config
        openssl
        openssl.dev
        zlib.dev
        
        # Development tools
        cargo-edit
        cargo-expand
        cargo-udeps
        cargo-watch
        cargo-outdated
        
        # CIM-specific tools
        jq
        yq
        direnv
        
        # Service dependencies
        nats-server
        natscli
      ] ++ extraPackages;
      
      shellHook = ''
        echo "🚀 Welcome to ${name} development environment!"
        echo ""
        echo "Available CIM modules: ${lib.concatStringsSep ", " modules}"
        echo ""
        echo "Useful commands:"
        echo "  cargo build    - Build the project"
        echo "  cargo test     - Run tests"
        echo "  cargo watch    - Watch for changes"
        echo "  nats-server    - Start local NATS server"
        echo ""
        ${extraShellHook}
      '';
      
      RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      RUST_LOG = "debug";
    };

  # Create a CIM service module
  mkCimService = {
    name,
    package,
    description ? "CIM ${name} service",
    extraConfig ? {},
    extraSystemdConfig ? {},
    ...
  }: { config, lib, pkgs, ... }:
    with lib;
    let
      cfg = config.services."cim-${name}";
    in {
      options.services."cim-${name}" = {
        enable = mkEnableOption description;
        
        package = mkOption {
          type = types.package;
          default = package;
          description = "Package to use for ${name}";
        };
        
        configFile = mkOption {
          type = types.nullOr types.path;
          default = null;
          description = "Path to configuration file";
        };
        
        extraArgs = mkOption {
          type = types.listOf types.str;
          default = [];
          description = "Extra command line arguments";
        };
        
        environmentFile = mkOption {
          type = types.nullOr types.path;
          default = null;
          description = "Environment file for secrets";
        };
      } // extraConfig;
      
      config = mkIf cfg.enable {
        systemd.services."cim-${name}" = lib.recursiveUpdate {
          description = description;
          wantedBy = [ "multi-user.target" ];
          after = [ "network.target" ];
          
          serviceConfig = {
            Type = "notify";
            ExecStart = "${cfg.package}/bin/${name} ${lib.concatStringsSep " " cfg.extraArgs}";
            Restart = "on-failure";
            RestartSec = "5s";
            
            # Security hardening
            DynamicUser = true;
            ProtectSystem = "strict";
            ProtectHome = true;
            NoNewPrivileges = true;
            PrivateTmp = true;
            
            # Optional environment file
            EnvironmentFile = lib.optional (cfg.environmentFile != null) cfg.environmentFile;
          };
          
          environment = {
            RUST_LOG = "info";
          } // lib.optionalAttrs (cfg.configFile != null) {
            CONFIG_FILE = cfg.configFile;
          };
        } extraSystemdConfig;
      };
    };

  # Create a CIM container image
  mkCimContainer = {
    name,
    package,
    tag ? "latest",
    contents ? [],
    config ? {},
    ...
  }@args:
    pkgs.dockerTools.buildImage {
      inherit name tag;
      
      contents = [ package ] ++ contents;
      
      config = lib.recursiveUpdate {
        Cmd = [ "/bin/${name}" ];
        WorkingDir = "/";
        Env = [
          "RUST_LOG=info"
          "RUST_BACKTRACE=1"
        ];
      } config;
    };

  # Kubernetes manifest helpers
  mkK8sDeployment = {
    name,
    namespace ? "cim",
    image,
    replicas ? 1,
    env ? [],
    resources ? {},
    ...
  }: {
    apiVersion = "apps/v1";
    kind = "Deployment";
    metadata = {
      inherit name namespace;
      labels.app = name;
    };
    spec = {
      inherit replicas;
      selector.matchLabels.app = name;
      template = {
        metadata.labels.app = name;
        spec.containers = [{
          inherit name image env resources;
          
          livenessProbe = {
            httpGet = {
              path = "/health";
              port = 8080;
            };
            initialDelaySeconds = 30;
            periodSeconds = 10;
          };
          
          readinessProbe = {
            httpGet = {
              path = "/ready";
              port = 8080;
            };
            initialDelaySeconds = 5;
            periodSeconds = 5;
          };
        }];
      };
    };
  };

  mkK8sService = {
    name,
    namespace ? "cim",
    port ? 8080,
    targetPort ? 8080,
    type ? "ClusterIP",
    ...
  }: {
    apiVersion = "v1";
    kind = "Service";
    metadata = {
      inherit name namespace;
      labels.app = name;
    };
    spec = {
      inherit type;
      selector.app = name;
      ports = [{
        inherit port targetPort;
        protocol = "TCP";
      }];
    };
  };

  # Configuration validation
  validateCimConfig = config: 
    let
      assertions = [
        {
          assertion = config ? version;
          message = "Configuration must specify a version";
        }
        {
          assertion = lib.versionAtLeast config.version "0.1.0";
          message = "Configuration version must be at least 0.1.0";
        }
      ];
      
      failedAssertions = lib.filter (a: !a.assertion) assertions;
    in
    if failedAssertions == []
    then config
    else throw "Configuration validation failed:\n${
      lib.concatMapStringsSep "\n" (a: "- ${a.message}") failedAssertions
    }";
}