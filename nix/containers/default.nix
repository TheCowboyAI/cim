# Container images for CIM modules
{ pkgs, lib, ... }:

let
  cimLib = import ../lib { inherit lib pkgs; };
in
{
  # CIM Events container
  cim-events = cimLib.mkCimContainer {
    name = "cim-events";
    package = pkgs.cim-events;
    tag = "latest";
    
    config = {
      ExposedPorts = {
        "8080/tcp" = {};
      };
      Env = [
        "RUST_LOG=info"
        "NATS_URL=nats://nats:4222"
      ];
      Labels = {
        "org.opencontainers.image.description" = "CIM Events Service";
        "org.opencontainers.image.source" = "https://github.com/thecowboyai/cim";
      };
    };
  };
  
  # CIM Projections container
  cim-projections = cimLib.mkCimContainer {
    name = "cim-projections";
    package = pkgs.cim-projections;
    tag = "latest";
    
    config = {
      ExposedPorts = {
        "8081/tcp" = {};
      };
      Env = [
        "RUST_LOG=info"
        "EVENT_SOURCE=nats://nats:4222"
      ];
      Labels = {
        "org.opencontainers.image.description" = "CIM Projections Service";
        "org.opencontainers.image.source" = "https://github.com/thecowboyai/cim";
      };
    };
  };
  
  # Development container with all tools
  cim-dev = pkgs.dockerTools.buildImage {
    name = "cim-dev";
    tag = "latest";
    
    contents = with pkgs; [
      bashInteractive
      coreutils
      gnugrep
      gnused
      jq
      curl
      git
      
      # CIM packages
      cim-events
      cim-projections
      
      # Development tools
      natscli
      redis
      postgresql
    ];
    
    config = {
      Cmd = [ "/bin/bash" ];
      WorkingDir = "/workspace";
      Env = [
        "RUST_LOG=debug"
        "RUST_BACKTRACE=1"
      ];
    };
  };
}