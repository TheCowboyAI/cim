# CIM Events package definition
{ lib
, rustPlatform
, pkg-config
, openssl
, cimLib
, fetchFromGitHub
}:

cimLib.mkCimPackage {
  name = "cim-events";
  src = ../../modules/cim-events;
  
  buildInputs = [ 
    openssl 
  ];
  
  nativeBuildInputs = [ 
    pkg-config 
  ];
  
  # Optional features
  features = lib.optional (lib.elem "ipfs" features or []) "ipfs";
  
  # Tests require NATS server
  checkPhase = ''
    echo "Skipping tests that require NATS server"
    cargo test --lib
  '';
  
  postInstall = ''
    # Install example configurations
    mkdir -p $out/share/cim-events/examples
    cp -r examples/* $out/share/cim-events/examples/
  '';
}