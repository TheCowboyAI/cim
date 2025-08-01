{ pkgs, lib, config, ... }:

with lib;

let
  cfg = config.cim.network;
  
  # Fetch cim-network from GitHub
  cim-network-src = pkgs.fetchFromGitHub {
    owner = "TheCowboyAI";
    repo = "cim-network";
    rev = "main"; # You can pin to a specific commit later
    sha256 = lib.fakeSha256; # This will fail initially, providing the correct hash
  };

  # Build cim-network package
  cim-network = pkgs.rustPlatform.buildRustPackage {
    pname = "cim-network";
    version = "0.1.0";
    
    src = cim-network-src;
    
    cargoSha256 = lib.fakeSha256; # This will fail initially, providing the correct hash
    
    buildInputs = with pkgs; [
      openssl
      pkg-config
    ];
    
    nativeBuildInputs = with pkgs; [
      pkg-config
    ];
    
    # Skip tests during build as they require NATS
    doCheck = false;
  };

in {
  options.cim.network = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable CIM network module";
    };
    
    package = mkOption {
      type = types.package;
      default = cim-network;
      description = "CIM network package to use";
    };
    
    config = mkOption {
      type = types.attrs;
      default = {};
      description = "CIM network configuration";
    };
  };
  
  config = mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
    
    # Add any systemd services or other configurations here
  };
}