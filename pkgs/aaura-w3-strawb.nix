{ lib
, craneLib
, pkg-config
, fontconfig
}:

craneLib.buildPackage {
  name = "aaura-w3-strawb";

  src = craneLib.cleanCargoSource ../.;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ fontconfig ];

  meta = with lib; {
    description = "A webserver serving Anna Aurora's dynamic website";
    homepage = "https://codeberg.org/annaaurora/aaura-w3-strawb";
  };
}
