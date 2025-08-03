# Development environment configuration
{ config, pkgs, lib, ... }:

{
  # Enable all CIM services
  services.cim-events = {
    enable = true;
    host = "127.0.0.1";
    port = 8080;
    logLevel = "debug";
    logFormat = "pretty";
    
    natsUrl = "nats://localhost:4222";
    
    jetstream = {
      enabled = true;
      bucket = "cim-events-dev";
    };
  };
  
  services.cim-projections = {
    enable = true;
    host = "127.0.0.1";
    port = 8081;
    logLevel = "debug";
    logFormat = "pretty";
    
    eventSource = "nats://localhost:4222";
    startFrom = "beginning";
    enabledStores = [ "memory" "postgres" ];
    
    postgres = {
      url = "postgres://cim:cim@localhost/cim_dev";
      maxConnections = 5;
    };
  };
  
  # Enable supporting services
  services.nats = {
    enable = true;
    jetstream = true;
  };
  
  services.postgresql = {
    enable = true;
    ensureDatabases = [ "cim_dev" ];
    ensureUsers = [{
      name = "cim";
      ensurePermissions = {
        "DATABASE cim_dev" = "ALL PRIVILEGES";
      };
    }];
  };
  
  # Development tools
  environment.systemPackages = with pkgs; [
    natscli
    httpie
    jq
    postgresql
  ];
}