# Default module set for CIM
{ ... }:

{
  imports = [
    ./cim-events.nix
    ./cim-projections.nix
  ];
}