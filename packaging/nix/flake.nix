{
  description = "Fast, lightweight Linux system information fetch tool written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "i686-linux"
        "riscv64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      packages = forAllSystems (system: {
        ferrisfetch = nixpkgsFor.${system}.callPackage ./package.nix {
          src = nixpkgs.lib.cleanSource ../..;
        };
        default = self.packages.${system}.ferrisfetch;
      });

      apps = forAllSystems (system: {
        ferrisfetch = {
          type = "app";
          program = "${self.packages.${system}.ferrisfetch}/bin/ferrisfetch";
        };
        default = self.apps.${system}.ferrisfetch;
      });

      devShells = forAllSystems (system: {
        default = nixpkgsFor.${system}.mkShell {
          inputsFrom = [ self.packages.${system}.ferrisfetch ];
          packages = with nixpkgsFor.${system}; [
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
          ];
        };
      });

      overlays.default = final: prev: {
        ferrisfetch = final.callPackage ./package.nix {
          src = final.lib.cleanSource ../..;
        };
      };
    };
}
