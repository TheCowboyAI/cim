# NixOS module for CIM Projections service
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.cim-projections;
  cimLib = import ../lib { inherit lib pkgs; };
  
  configFile = pkgs.writeText "cim-projections.toml" ''
    [server]
    host = "${cfg.host}"
    port = ${toString cfg.port}
    
    [events]
    source = "${cfg.eventSource}"
    start_from = "${cfg.startFrom}"
    
    [stores]
    ${concatStringsSep "\n" (map (store: ''
    [[stores.${store}]]
    enabled = true
    '') cfg.enabledStores)}
    
    ${optionalString (elem "postgres" cfg.enabledStores) ''
    [postgres]
    url = "${cfg.postgres.url}"
    max_connections = ${toString cfg.postgres.maxConnections}
    ''}
    
    ${optionalString (elem "redis" cfg.enabledStores) ''
    [redis]
    url = "${cfg.redis.url}"
    ''}
    
    [logging]
    level = "${cfg.logLevel}"
    format = "${cfg.logFormat}"
  '';
in
{
  options.services.cim-projections = {
    enable = mkEnableOption "CIM Projections service";
    
    package = mkOption {
      type = types.package;
      default = pkgs.cim-projections;
      defaultText = literalExpression "pkgs.cim-projections";
      description = "The cim-projections package to use";
    };
    
    host = mkOption {
      type = types.str;
      default = "0.0.0.0";
      description = "Host to bind to";
    };
    
    port = mkOption {
      type = types.port;
      default = 8081;
      description = "Port to listen on";
    };
    
    eventSource = mkOption {
      type = types.str;
      default = "nats://localhost:4222";
      description = "Event source URL (NATS or cim-events endpoint)";
    };
    
    startFrom = mkOption {
      type = types.enum [ "beginning" "end" "stored" ];
      default = "stored";
      description = "Where to start processing events from";
    };
    
    enabledStores = mkOption {
      type = types.listOf (types.enum [ "memory" "postgres" "redis" ]);
      default = [ "memory" ];
      description = "Projection stores to enable";
    };
    
    postgres = {
      url = mkOption {
        type = types.str;
        default = "postgres://localhost/cim_projections";
        description = "PostgreSQL connection URL";
      };
      
      maxConnections = mkOption {
        type = types.int;
        default = 10;
        description = "Maximum PostgreSQL connections";
      };
    };
    
    redis = {
      url = mkOption {
        type = types.str;
        default = "redis://localhost";
        description = "Redis connection URL";
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
    # Ensure dependencies are available
    systemd.services.cim-projections = {
      description = "CIM Projections Service";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ]
        ++ optional config.services.cim-events.enable "cim-events.service"
        ++ optional (elem "postgres" cfg.enabledStores && config.services.postgresql.enable) "postgresql.service"
        ++ optional (elem "redis" cfg.enabledStores && config.services.redis.servers != {}) "redis.service";
      
      serviceConfig = {
        Type = "notify";
        ExecStart = "${cfg.package}/bin/cim-projections --config ${configFile}";
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
        StateDirectory = "cim-projections";
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