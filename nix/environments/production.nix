# Production environment configuration
{ config, pkgs, lib, ... }:

{
  # Production CIM services configuration
  services.cim-events = {
    enable = true;
    host = "0.0.0.0";
    port = 8080;
    logLevel = "info";
    logFormat = "json";
    
    # Use environment file for sensitive configuration
    environmentFile = "/run/secrets/cim-events-env";
    
    jetstream = {
      enabled = true;
      bucket = "cim-events-prod";
    };
    
    # Enable IPFS for immutable event storage
    ipfs = {
      enabled = true;
      apiUrl = "http://ipfs:5001";
    };
  };
  
  services.cim-projections = {
    enable = true;
    host = "0.0.0.0";
    port = 8081;
    logLevel = "info";
    logFormat = "json";
    
    environmentFile = "/run/secrets/cim-projections-env";
    
    eventSource = "http://cim-events:8080";
    startFrom = "stored";
    enabledStores = [ "postgres" "redis" ];
    
    postgres = {
      # URL will be provided via environment file
      maxConnections = 20;
    };
  };
  
  # Security hardening
  security.apparmor.enable = true;
  security.audit.enable = true;
  
  # Monitoring
  services.prometheus = {
    enable = true;
    scrapeConfigs = [
      {
        job_name = "cim-events";
        static_configs = [{
          targets = [ "localhost:8080" ];
        }];
      }
      {
        job_name = "cim-projections";
        static_configs = [{
          targets = [ "localhost:8081" ];
        }];
      }
    ];
  };
  
  # Backup configuration
  services.postgresqlBackup = {
    enable = true;
    databases = [ "cim_prod" ];
    location = "/var/backup/postgresql";
    startAt = "*-*-* 01:00:00";
  };
}