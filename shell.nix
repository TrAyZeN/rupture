{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    alsaLib
    cmake
    freetype
    expat
    openssl
    pkgconfig
    python3
    vulkan-validation-layers
    xorg.libX11
  ];

  APPEND_LIBRARY_PATH = pkgs.stdenv.lib.makeLibraryPath [
    pkgs.vulkan-loader
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.xorg.libXrandr
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
