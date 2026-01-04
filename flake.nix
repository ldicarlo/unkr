{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, advisory-db }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        lib = pkgs.lib;

        fenixToolchain =
          (
            fenix.packages.${system}.toolchainOf
              {
                channel = "nightly";
                date = "2025-06-30";
                sha256 = "sha256-FuOGHL+DbavyycfaDakNP1ANZ0qox3ha+v2/4MVI5YY=";


                # .fromToolchainFile
                #   {
                #     file = ./rust-toolchain;
                #     sha256 = "sha256-FuOGHL+DbavyycfaDakNP1ANZ0qox3ha+v2/4MVI5YY=";
              }
          ).withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustc-dev"
            "rustfmt"
            "llvm-tools"
          ];

        craneLib = (crane.mkLib nixpkgs.legacyPackages.${system}).overrideToolchain fenixToolchain;
        src = lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (lib.hasInfix "/unkr/tests/" path) ||
            (lib.hasInfix "/unkr/cache-tests/" path) ||
            (lib.hasInfix "/unkr/src/gpu/" path) ||
            (craneLib.filterCargoSources path type)
          ;
        };

        buildInputs = with pkgs; [
          fenixToolchain
          cmake
          libxkbcommon
          shaderc
          spirv-tools
          stdenv.cc.cc.lib
          vulkan-extension-layer
          vulkan-headers
          vulkan-loader
          vulkan-tools
          vulkan-tools-lunarg
          vulkan-validation-layers
          vulkan-volk
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          xorg.libxcb
          pkg-config
          openssl
        ];

        nativeBuildInputs = with pkgs; buildInputs ++ [
          pkg-config
          cmake
          clang
          cargo-hakari
        ];

        # https://github.com/4JX/L5P-Keyboard-RGB/blob/ce2639fc67942a77fd39c3ce427542785b9568e7/flake.nix
        # https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
        commonArgs = {
          SHADERC_LIB_DIR = pkgs.lib.makeLibraryPath [ pkgs.shaderc ];
          hardeningDisable = [ "fortify" ];
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
          VULKAN_SDK = "${pkgs.vulkan-validation-layers}/share/vulkan/";
          RUST_SRC_PATH = "${fenixToolchain}/lib/rustlib/rustc-src/rust/library";
          pname = "unkr";
          inherit src;
          inherit buildInputs;
          inherit nativeBuildInputs;
        };

        fileSetForCrate =
          crate:
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneLib.fileset.commonCargoSources ./unkr)
              (craneLib.fileset.commonCargoSources ./shader)
              (craneLib.fileset.commonCargoSources crate)
            ];
          };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          doCheck = false;
        };

        unkr = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "unkr";
            inherit src;
            cargoExtraArgs = "-p unkr";
          }
        );

        shader = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "shader";
            cargoExtraArgs = "-p shader";
            src = fileSetForCrate ./shader;
          }
        );
      in
      rec {

        checks = {
          # Build the crates as part of `nix flake check` for convenience
          inherit unkr; inherit shader;

          # Run clippy (and deny all warnings) on the workspace source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-workspace-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          my-workspace-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              # This can be commented out or tweaked as necessary, e.g. set to
              # `--deny rustdoc::broken-intra-doc-links` to only enforce that lint
              env.RUSTDOCFLAGS = "--deny warnings";
            }
          );

          # Check formatting
          my-workspace-fmt = craneLib.cargoFmt {
            inherit src;
          };

          my-workspace-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            # taplo arguments can be further customized below as needed
            # taploExtraArgs = "--config ./taplo.toml";
          };

          # Audit dependencies
          my-workspace-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Audit licenses
          my-workspace-deny = craneLib.cargoDeny {
            inherit src;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on other crate derivations
          # if you do not want the tests to run twice
          my-workspace-nextest = craneLib.cargoNextest (
            commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
              cargoNextestPartitionsExtraArgs = "--no-tests=pass";
            }
          );

          # Ensure that cargo-hakari is up to date
          my-workspace-hakari = craneLib.mkCargoDerivation {
            inherit src;
            pname = "my-workspace-hakari";
            cargoArtifacts = null;
            doInstallCargoArtifacts = false;

            buildPhaseCargoCommand = ''
              cargo hakari generate --diff  # workspace-hack Cargo.toml is up-to-date
              cargo hakari manage-deps --dry-run  # all workspace crates depend on workspace-hack
              cargo hakari verify
            '';

            inherit nativeBuildInputs;
          };
        };

        devShells.default = pkgs.mkShell rec {
          hardeningDisable = [ "fortify" ];

          SHADERC_LIB_DIR = pkgs.lib.makeLibraryPath [ pkgs.shaderc ];
          RUST_SRC_PATH = "${fenixToolchain}/lib/rustlib/rustc-src/rust/library";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
          VULKAN_SDK = "${pkgs.vulkan-validation-layers}/share/vulkan/";

          inherit nativeBuildInputs;
          inherit buildInputs;
        };

        packages = rec {
          default = pkgs.symlinkJoin {
            name = "all";
            paths = [ unkr shader ];
          };
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
