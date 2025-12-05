{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        lib = pkgs.lib;
        craneLib = (crane.mkLib nixpkgs.legacyPackages.${system});

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
        fenixToolchain = (fenix.packages.${system}.fromToolchainFile
          {
            file = ./rust-toolchain.toml;
            sha256 = "FuOGHL+DbavyycfaDakNP1ANZ0qox3ha+v2/4MVI5YY=";
          });
      in
      rec {
        devShells.default = pkgs.mkShell rec {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          # RUST_SRC_PATH = fenixToolchain;

          nativeBuildInputs = [ fenixToolchain ];

          buildInputs = with pkgs;  [
            vulkan-loader
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
              dataDir = mkOption {
                default = "/var/lib/unkr";
                type = types.path;
                description = "The data directory.";
              };
            };
            config = mkIf cfg.enable
              {
                users.users.unkr = {
                  group = "unkr";
                  isNormalUser = true;
                };
                users.groups.unkr = { };
                systemd = {
                  services.unkr = {
                    description = "Unkr runner";
                    wantedBy = [ "multi-user.target" ];
                    environment = { };
                    serviceConfig = {
                      Type = "simple";
                      User = "unkr";
                      Group = "unkr";
                      ExecStart = "${unkr}/bin/unkr ${cfg.command}";
                      Restart = "on-failure";
                      RestartSec = "100s";
                      WorkingDirectory = "${cfg.dataDir}";
                    };

                  };
                  tmpfiles.rules = [
                    "d '${cfg.dataDir}' - unkr unkr - -"
                  ];

                };
              };
          };
      });
}
