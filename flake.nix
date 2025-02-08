{
  description = "Anna Aurora's dynamic website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    Lyrically-Vantage.url = "https://codeberg.org/annaaurora/Lyrically-Vantage/archive/main.tar.gz";
    Lettuce-Synthetic.url = "https://codeberg.org/annaaurora/Lettuce-Synthetic/archive/main.tar.gz";
  };

  outputs = { self, nixpkgs, flake-utils, crane, Lyrically-Vantage, Lettuce-Synthetic }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
      in rec {
        packages.aaura-w3-strawb = pkgs.callPackage ./pkgs/aaura-w3-strawb.nix { inherit craneLib; };
        packages.aaura-w3-strawb-overlay = pkgs.callPackage ./pkgs/aaura-w3-strawb-overlay {
          Lyrically-Vantage = Lyrically-Vantage.packages.${system}.default;
          Lettuce-Synthetic = Lettuce-Synthetic.packages.${system}.default;
        };
        packages.default = self.packages.${system}.aaura-w3-strawb;

        overlays.default = final: prev: {
          aaura-w3-strawb = self.packages.${system}.aaura-w3-strawb;
          aaura-w3-strawb-overlay = self.packages.${system}.aaura-w3-strawb-overlay;
        };
      }
    ) // {
      nixosModules.aaura-w3-strawb = import ./modules/aaura-w3-strawb.nix;
      nixosModules.default = self.nixosModules.aaura-w3-strawb;
    };
}
