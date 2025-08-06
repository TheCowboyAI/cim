# CIM Projections package definition
{ lib
, rustPlatform
, pkg-config
, openssl
, postgresql
, redis
, cimLib
}:

cimLib.mkCimPackage {
  name = "cim-projections";
  src = ../../modules/cim-projections;
  
  buildInputs = [ 
    openssl
    postgresql
    redis
  ];
  
  nativeBuildInputs = [ 
    pkg-config 
  ];
  
  # Build with all stores by default
  features = [ "all-stores" ];
  
  # Skip integration tests that require running services
  doCheck = false;
  
  postInstall = ''
    # Install example configurations
    mkdir -p $out/share/cim-projections/examples
    cp -r examples/* $out/share/cim-projections/examples/
  '';
}