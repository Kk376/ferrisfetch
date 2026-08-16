{ pkgs ? import <nixpkgs> { } }:

pkgs.callPackage ./package.nix {
  src = pkgs.lib.cleanSource ../..;
}
