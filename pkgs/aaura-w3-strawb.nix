{ lib
, rustPlatform
, pkg-config
, openssl
, fontconfig
}:

rustPlatform.buildRustPackage rec {
  name = "aaura-w3-strawb";

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl fontconfig ];

  meta = with lib; {
    description = "A webserver serving Anna Aurora's dynamic website";
    homepage = "https://codeberg.org/annaaurora/aaura-w3-strawb";
  };
}