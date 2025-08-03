# Default package set for CIM
{ pkgs, lib, ... }:

let
  cimLib = import ../lib { inherit lib pkgs; };
in
{
  # Individual packages
  cim-events = pkgs.callPackage ./cim-events.nix { inherit cimLib; };
  cim-projections = pkgs.callPackage ./cim-projections.nix { inherit cimLib; };
  
  # Meta package that includes all CIM modules
  cim-all = pkgs.symlinkJoin {
    name = "cim-all";
    paths = [
      pkgs.cim-events
      pkgs.cim-projections
    ];
    meta = {
      description = "All CIM modules";
      homepage = "https://github.com/thecowboyai/cim";
      license = lib.licenses.asl20;
    };
  };
  
  # Development tools package
  cim-dev-tools = pkgs.buildEnv {
    name = "cim-dev-tools";
    paths = with pkgs; [
      # NATS tools
      nats-server
      natscli
      nats-top
      
      # Database tools
      postgresql
      redis
      
      # Monitoring
      prometheus
      grafana
      
      # Development utilities
      httpie
      jq
      yq
      grpcurl
    ];
  };
}