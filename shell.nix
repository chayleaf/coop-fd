{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "shell-coop-fd";
  nativeBuildInputs = with pkgs; [ rustc cargo ];
}
