{ pkgs ? (import <nixpkgs> { }).pkgs }:
with pkgs;
mkShell {
  buildInputs = [ cargo SDL2 SDL2_ttf ];
  shellHook = ''
    LD_LIBRARY_PATH="${pkgs.SDL2}/lib"
  '';
}
