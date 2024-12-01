{
    description = "Flake for AoC solutions";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = {self, nixpkgs, flake-utils, ... }@inputs:
        flake-utils.lib.eachDefaultSystem (
            system:
            let
                overlays = [ ];
                pkgs = import nixpkgs { inherit system overlays; };
            in
            with pkgs;
            {
                devShells.default = mkShell {
                    nativeBuildInputs = [
                        cargo
                        rustc
                        gcc
                        rustfmt
                        clippy
                    ];
                    RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
                };
            }
        );
}