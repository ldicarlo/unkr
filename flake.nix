{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        lib = import lib { };
        craneLib = crane.lib.${system};
        code = pkgs.callPackage ./. { inherit nixpkgs system; };
      in
      rec {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.rustfmt
            pkgs.cargo
          ];
        };

        packages = rec {
          default = unkr;
          # https://crane.dev/getting-started.html
          unkr = craneLib.buildPackage rec {
            name = "unkr";
            src = craneLib.cleanCargoSource ./.;
            rust-dependencies = craneLib.buildDepsOnly {
              inherit src;
            };

            rust-package-binary = craneLib.buildPackage {
              inherit src;
              cargoArtifacts = rust-dependencies;

              doCheck = true;
            };
            # postUnpack = ''
            #   cd $sourceRoot/api
            #   sourceRoot="."
            # '';
          };
        };
        #    defaultPackage = packages.all;
      });
}
