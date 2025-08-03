# NixOS module for CIM Events service
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.cim-events;
  cimLib = import ../lib { inherit lib pkgs; };
  
  configFile = pkgs.writeText "cim-events.toml" ''
    [server]
    host = "${cfg.host}"
    port = ${toString cfg.port}
    
    [nats]
    url = "${cfg.natsUrl}"
    ${optionalString (cfg.natsCredentials != null) ''
    credentials = "${cfg.natsCredentials}"
    ''}
    
    [jetstream]
    enabled = ${boolToString cfg.jetstream.enabled}
    bucket = "${cfg.jetstream.bucket}"
    
    [logging]
    level = "${cfg.logLevel}"
    format = "${cfg.logFormat}"
    
    ${optionalString cfg.ipfs.enabled ''
    [ipfs]
    enabled = true
    api_url = "${cfg.ipfs.apiUrl}"
    ''}
  '';
in
{
  options.services.cim-events = {
    enable = mkEnableOption "CIM Events service";
    
    package = mkOption {
      type = types.package;
      default = pkgs.cim-events;
      defaultText = literalExpression "pkgs.cim-events";
      description = "The cim-events package to use";
    };
    
    host = mkOption {
      type = types.str;
      default = "0.0.0.0";
      description = "Host to bind to";
    };
    
    port = mkOption {
      type = types.port;
      default = 8080;
      description = "Port to listen on";
    };
    
    natsUrl = mkOption {
      type = types.str;
      default = "nats://localhost:4222";
      description = "NATS server URL";
    };
    
    natsCredentials = mkOption {
      type = types.nullOr types.path;
      default = null;
      description = "Path to NATS credentials file";
    };
    
    jetstream = {
      enabled = mkOption {
        type = types.bool;
        default = true;
        description = "Enable JetStream";
      };
      
      bucket = mkOption {
        type = types.str;
        default = "cim-events";
        description = "JetStream bucket name";
      };
    };
    
    ipfs = {
      enabled = mkOption {
        type = types.bool;
        default = false;
        description = "Enable IPFS integration";
      };
      
      apiUrl = mkOption {
        type = types.str;
        default = "http://localhost:5001";
        description = "IPFS API URL";
      };
    };
    
    logLevel = mkOption {
      type = types.enum [ "trace" "debug" "info" "warn" "error" ];
      default = "info";
      description = "Log level";
    };
    
    logFormat = mkOption {
      type = types.enum [ "json" "pretty" "compact" ];
      default = "json";
      description = "Log format";
    };
    
    environmentFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      description = "Environment file for secrets";
    };
  };
  
  config = mkIf cfg.enable {
    # Ensure NATS is available
    systemd.services.cim-events = {
      description = "CIM Events Service";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ] 
        ++ optional config.services.nats.enable "nats.service";
      
      serviceConfig = {
        Type = "notify";
        ExecStart = "${cfg.package}/bin/cim-events --config ${configFile}";
        Restart = "on-failure";
        RestartSec = "5s";
        
        # Security hardening
        DynamicUser = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        NoNewPrivileges = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        RestrictAddressFamilies = [ "AF_INET" "AF_INET6" "AF_UNIX" ];
        RestrictNamespaces = true;
        LockPersonality = true;
        MemoryDenyWriteExecute = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        
        # State directory
        StateDirectory = "cim-events";
        StateDirectoryMode = "0750";
        
        # Optional environment file
        EnvironmentFile = lib.optional (cfg.environmentFile != null) cfg.environmentFile;
      };
      
      environment = {
        RUST_LOG = cfg.logLevel;
        RUST_BACKTRACE = "1";
      };
    };
    
    # Open firewall port
    networking.firewall.allowedTCPPorts = [ cfg.port ];
  };
}