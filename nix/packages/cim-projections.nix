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
  ] ++ lib.optionals (lib.elem "postgres" features or []) [ postgresql ]
    ++ lib.optionals (lib.elem "redis" features or []) [ redis ];
  
  nativeBuildInputs = [ 
    pkg-config 
  ];
  
  # Build with all stores by default
  features = features or [ "all-stores" ];
  
  # Skip integration tests that require running services
  checkPhase = ''
    echo "Skipping integration tests that require external services"
    cargo test --lib
  '';
  
  postInstall = ''
    # Install example configurations
    mkdir -p $out/share/cim-projections/examples
    cp -r examples/* $out/share/cim-projections/examples/
  '';
}