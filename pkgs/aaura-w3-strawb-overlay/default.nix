{ stdenv
, lib
, lmms
, ffmpeg
, python3Packages
, imagemagick
, oxipng
, Lyrically-Vantage
, Lettuce-Synthetic
, neofox
}:

let
  video-game-nostalgia = stdenv.mkDerivation {
    name = "aaura-w3-strawb-video-game-nostalgia";
    src = ../../markdown;
    nativeBuildInputs = [ lmms ffmpeg ];
    buildPhase = ''
      mkdir $out
      lmms render art/video-game-nostalgia/video-game-nostalgia.mmpz --allowroot -i sincbest --format wav --loop --output video-game-nostalgia.wav
      ffmpeg -v 1 -i video-game-nostalgia.wav video-game-nostalgia.flac
    '';
    installPhase = ''
      mv video-game-nostalgia.flac $out/
    '';
  };
  markdown = stdenv.mkDerivation {
    name = "aaura-w3-strawb-markdown";
    src = ../../markdown;
    installPhase = ''
      mkdir $out
      mv * $out/

      ln -s ${Lyrically-Vantage}/* $out/art/lyrically-vantage/
      ln -s ${Lettuce-Synthetic}/composited/* $out/art/lettuce-synthetic/
      ln -s ${video-game-nostalgia}/video-game-nostalgia.flac $out/art/video-game-nostalgia/
    '';
  };
  neofox-64x64-png = stdenv.mkDerivation {
    name = "neofox-optimized";
    src = neofox;
    nativeBuildInputs = [ imagemagick oxipng ];
    buildPhase = ''
      mkdir optimized

      for emote in ${neofox}/*.png; do
        magick "$emote" -resize 64x64 optimized/"$(basename "$emote")"
        oxipng --opt max -Z --strip safe optimized/"$(basename "$emote")"
      done
    '';
    installPhase = ''
      mkdir $out
      mv optimized/* $out/
    '';
  };
  neofox-64x64-avif = stdenv.mkDerivation {
    name = "neofox-avif";
    src = neofox;
    nativeBuildInputs = [ imagemagick ];
    buildPhase = ''
      mkdir optimized

      for emote in ${neofox}/*.png; do
        magick "$emote" -resize 64x64 -quality 30 optimized/"$(basename "$emote" .png)".avif
      done
    '';
    installPhase = ''
      mkdir $out
      mv optimized/* $out/
    '';
  };
in stdenv.mkDerivation rec {
  name = "aaura-w3-strawb-overlay";

  static-src = ../../static-src;
  static = ../../static;
  src = ./.;

  nativeBuildInputs = (with python3Packages; [ python ffmpy pillow ])
    ++ [ imagemagick oxipng ];

  installPhase = ''
    runHook preInstall

    pwd

    mkdir -p $out/static
    convert ${static-src}/favicon.png -scale 36x30 $out/static/favicon.png
    oxipng --opt max -Z --strip safe $out/static/favicon.png

    mkdir -p $out/static/neofox
    for emote in ${neofox-64x64-png}/* ${neofox-64x64-avif}/*; do
      ln -s "$emote" $out/static/neofox/"$(basename "$emote")"
    done

    python process.py ${markdown} $out/markdown

    ln -s ${Lyrically-Vantage}/* $out/markdown/art/lyrically-vantage/
    ln -s ${Lettuce-Synthetic}/composited/* $out/markdown/art/lettuce-synthetic/
    ln -s ${video-game-nostalgia}/* $out/markdown/art/video-game-nostalgia/

    runHook postInstall
  '';

  meta = with lib; {
    description = "Files to be overlaid for serving Anna Aurora's website";
    homepage = "https://codeberg.org/annaaurora/aaura-w3-strawb";
  };
}
