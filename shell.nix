{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    packages = [
      pkgs.rustup
      pkgs.cargo-workspaces
      pkgs.cargo-nextest
    ];
  }
