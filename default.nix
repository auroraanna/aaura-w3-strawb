{ pkgs ? import <nixpkgs> {} }: {
  aaura-w3-strawb = pkgs.callPackage ./pkgs/aaura-w3-strawb.nix {};
}
