{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "shell-coop-fd";
  nativeBuildInputs = with pkgs; [ rustc cargo pkg-config ];
  buildInputs = with pkgs; [ tesseract leptonica libclang.lib ];
  LD_LIBRARY_PATH = "${pkgs.tesseract}/lib:${pkgs.leptonica}/lib";
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
