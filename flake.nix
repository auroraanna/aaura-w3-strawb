{
  description = "Anna Aurora's dynamic website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    Lyrically-Vantage.url = "https://codeberg.org/annaaurora/Lyrically-Vantage/archive/main.tar.gz";
    Lettuce-Synthetic.url = "https://codeberg.org/annaaurora/Lettuce-Synthetic/archive/main.tar.gz";
  };

  outputs = { self, nixpkgs, flake-utils, Lyrically-Vantage, Lettuce-Synthetic }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in rec {
        packages.aaura-w3-strawb = pkgs.callPackage ./pkgs/aaura-w3-strawb.nix {};
        packages.aaura-w3-strawb-overlay = pkgs.callPackage ./pkgs/aaura-w3-strawb-overlay {
          Lyrically-Vantage = Lyrically-Vantage.packages.${system}.default;
          Lettuce-Synthetic = Lettuce-Synthetic.packages.${system}.default;
        };
        packages.default = self.packages.${system}.aaura-w3-strawb;
      }
    ) // {
      overlays.default = import ./overlay.nix;

      nixosModules.aaura-w3-strawb = import ./modules/aaura-w3-strawb.nix;
      nixosModules.default = self.nixosModules.aaura-w3-strawb;
    };
}
