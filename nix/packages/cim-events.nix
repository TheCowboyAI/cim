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
  
  # Optional features (disabled for now)
  # features = [];
  
  # Tests require NATS server
  doCheck = false;
  
  postInstall = ''
    # Install example configurations
    mkdir -p $out/share/cim-events/examples
    cp -r examples/* $out/share/cim-events/examples/
  '';
}