{ stdenv
, lib
, python3Packages
, imagemagick
, oxipng
, Lyrically-Vantage
}:

let
  markdown = stdenv.mkDerivation {
    name = "aaura-w3-strawb-markdown";
    src = ../../markdown;
    installPhase = ''
      mkdir $out
      mv * $out/
      mkdir -p $out/art/lyrically-vantage
      ln -s ${Lyrically-Vantage}/* $out/art/lyrically-vantage/
    '';
  };
in stdenv.mkDerivation rec {
  name = "aaura-w3-strawb-overlay";

  static-src = ../../static-src;
  static = ../../static;
  src = ./.;

  buildInputs = (with python3Packages; [ python ffmpy pillow ])
    ++ [ imagemagick oxipng ];

  installPhase = ''
    runHook preInstall

    pwd

    mkdir -p $out/static
    convert ${static-src}/favicon.png -scale 36x30 $out/static/favicon.png
    oxipng --opt max -Z --strip safe $out/static/favicon.png

    python process.py ${markdown} $out/markdown

    cp ${Lyrically-Vantage}/* $out/markdown/art/lyrically-vantage/

    runHook postInstall
  '';

  meta = with lib; {
    description = "Files to be overlaid for serving Anna Aurora's website";
    homepage = "https://codeberg.org/annaaurora/aaura-w3-strawb";
  };
}
