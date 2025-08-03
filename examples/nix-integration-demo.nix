# Example demonstrating CIM Nix integration
{ pkgs ? import <nixpkgs> {} }:

let
  # Import CIM flake
  cim = builtins.getFlake (toString ../.);
  
  # Get system-specific outputs
  system = pkgs.system;
  cimPkgs = cim.packages.${system};
in
{
  # Example 1: Simple development environment
  devEnv = pkgs.mkShell {
    buildInputs = [
      cimPkgs.cim-events
      cimPkgs.cim-projections
      pkgs.nats-server
    ];
    
    shellHook = ''
      echo "CIM development environment loaded"
      echo "Starting NATS server..."
      nats-server -js &
      NATS_PID=$!
      
      echo "Starting CIM services..."
      cim-events &
      EVENTS_PID=$!
      
      cim-projections &
      PROJ_PID=$!
      
      echo "Services started:"
      echo "  NATS: PID $NATS_PID"
      echo "  CIM Events: PID $EVENTS_PID"
      echo "  CIM Projections: PID $PROJ_PID"
      
      # Cleanup on exit
      trap "kill $NATS_PID $EVENTS_PID $PROJ_PID" EXIT
    '';
  };
  
  # Example 2: Docker Compose equivalent in Nix
  services = {
    nats = {
      image = pkgs.dockerTools.pullImage {
        imageName = "nats";
        imageDigest = "sha256:..."; # Use actual digest
        sha256 = "..."; # Use actual sha256
      };
      ports = [ "4222:4222" "8222:8222" ];
      command = [ "-js" "-m" "8222" ];
    };
    
    cim-events = {
      image = cim.containers.${system}.cim-events;
      depends_on = [ "nats" ];
      environment = {
        NATS_URL = "nats://nats:4222";
        RUST_LOG = "debug";
      };
      ports = [ "8080:8080" ];
    };
    
    cim-projections = {
      image = cim.containers.${system}.cim-projections;
      depends_on = [ "nats" "cim-events" ];
      environment = {
        EVENT_SOURCE = "http://cim-events:8080";
        RUST_LOG = "debug";
      };
      ports = [ "8081:8081" ];
    };
  };
  
  # Example 3: NixOS VM for testing
  testVm = pkgs.nixosTest {
    name = "cim-integration-test";
    
    nodes = {
      server = { config, pkgs, ... }: {
        imports = [ cim.nixosModules.default ];
        
        services.nats = {
          enable = true;
          jetstream = true;
        };
        
        services.cim-events = {
          enable = true;
          natsUrl = "nats://localhost:4222";
        };
        
        services.cim-projections = {
          enable = true;
          eventSource = "http://localhost:8080";
        };
      };
    };
    
    testScript = ''
      server.wait_for_unit("nats.service")
      server.wait_for_unit("cim-events.service")
      server.wait_for_unit("cim-projections.service")
      
      # Test event publishing
      server.succeed("curl -X POST http://localhost:8080/events -d '{\"type\":\"test\"}'")
      
      # Verify projection
      server.succeed("curl http://localhost:8081/projections | grep test")
    '';
  };
  
  # Example 4: Kubernetes deployment
  k8sDeployment = {
    apiVersion = "v1";
    kind = "List";
    items = [
      # NATS StatefulSet
      {
        apiVersion = "apps/v1";
        kind = "StatefulSet";
        metadata = {
          name = "nats";
          namespace = "cim";
        };
        spec = {
          serviceName = "nats";
          replicas = 3;
          selector.matchLabels.app = "nats";
          template = {
            metadata.labels.app = "nats";
            spec.containers = [{
              name = "nats";
              image = "nats:2.10-alpine";
              command = [
                "nats-server"
                "--cluster_name=cim"
                "--js"
                "--sd=/data"
              ];
              volumeMounts = [{
                name = "data";
                mountPath = "/data";
              }];
            }];
          };
          volumeClaimTemplates = [{
            metadata.name = "data";
            spec = {
              accessModes = [ "ReadWriteOnce" ];
              resources.requests.storage = "10Gi";
            };
          }];
        };
      }
      
      # CIM Events Deployment
      {
        apiVersion = "apps/v1";
        kind = "Deployment";
        metadata = {
          name = "cim-events";
          namespace = "cim";
        };
        spec = {
          replicas = 3;
          selector.matchLabels.app = "cim-events";
          template = {
            metadata.labels.app = "cim-events";
            spec.containers = [{
              name = "cim-events";
              image = "cim-events:latest";
              env = [
                {
                  name = "NATS_URL";
                  value = "nats://nats:4222";
                }
              ];
              ports = [{
                containerPort = 8080;
              }];
            }];
          };
        };
      }
    ];
  };
}