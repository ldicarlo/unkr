{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
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
          buildInputs = with pkgs; [ pkg-config openssl rustc ];
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

        nixosModules.default = { config, lib, pkgs, ... }:
          let cfg = config.services.unkr;
          in
          with lib; {
            options.services.unkr = {
              enable = mkEnableOption "Enable Unkr service";
              command = mkOption {
                type = types.str;
                default = "get-combinations --elements-count 2 --picks 2";
              };
            };
            config = mkIf cfg.enable
              {
                systemd.services.unkr = {
                  description = "Unkr runner";
                  wantedBy = [ "multi-user.target" ];
                  environment = { };
                  serviceConfig = {
                    User = "nobody";
                    Group = "rebalance";
                    ExecStart = "${unkr}/bin/unkr ${cfg.command}";
                    Restart = "on-failure";
                    RestartSec = "100s";
                    WorkingDirectory = "/var/lib/unkr";
                  };
                };
              };
          };
      });
}
