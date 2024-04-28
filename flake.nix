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
        };
        lib = pkgs.lib;
        craneLib = crane.lib.${system};

        commonArgs = {
          src = lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (lib.hasInfix "tests/" path) ||
              (lib.hasInfix "cache-tests/" path) ||
              (craneLib.filterCargoSources path type)
            ;
          };
          buildInputs = with pkgs; [ pkg-config openssl ];
        };
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          pname = "unkr";
        });

        unkr = craneLib.buildPackage (commonArgs // {
          pname = "unkr";
          inherit cargoArtifacts;
        });
      in
      rec {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.rustfmt
            pkgs.cargo
            pkgs.flamegraph
          ];
        };

        packages = rec {
          default = unkr;
        };

        nixosSystem = { };

      });
}
