{
  pkgs ? import <nixpkgs> {},
  craneLib
}: {
  aaura-w3-strawb = pkgs.callPackage ./pkgs/aaura-w3-strawb.nix { inherit craneLib; };
  aaura-w3-strawb-overlay = pkgs.callPackage ./pkgs/aaura-w3-strawb-overlay {};
}
