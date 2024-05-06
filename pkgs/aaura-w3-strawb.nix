{ lib
, rustPlatform
, pkg-config
, fontconfig
}:

rustPlatform.buildRustPackage rec {
  name = "aaura-w3-strawb";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ fontconfig ];

  meta = with lib; {
    description = "A webserver serving Anna Aurora's dynamic website";
    homepage = "https://codeberg.org/annaaurora/aaura-w3-strawb";
  };
}
