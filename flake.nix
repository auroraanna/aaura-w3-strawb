{
  description = "Anna Aurora's dynamic website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in rec {
        packages.aaura-w3-strawb = pkgs.callPackage ./pkgs/aaura-w3-strawb.nix {};
        packages.default = self.packages.${system}.aaura-w3-strawb;
      }
    ) // {
      overlays.default = import ./overlay.nix;
    };
}
