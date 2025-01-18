{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [
    SDL2
    SDL2_ttf
    SDL2_gfx
    SDL2_sound
    SDL2_image
    gcc
    espflash
    rustup
    rustc
    rustfmt
    cargo
    ];
}
