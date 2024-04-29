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
        inherit (pkgs) lib;
        craneLib = crane.lib.${system};
      in
      rec {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.nodejs_20
            pkgs.rustc
            pkgs.rustfmt
            pkgs.cargo
            pkgs.pkg-config
            pkgs.openssl
          ];
        };

        nixosModules.default = { config, lib, pkgs, ... }:
          with lib;
          let
            cfg = config.services.rebalance;
            rebalance-backend = self.packages.${system}.rebalance-backend;
            rebalance-frontend = self.packages.${system}.rebalance-frontend;
          in
          {
            options.services.rebalance = {
              enable = mkEnableOption "Enable rebalance service";
              #   envFile = mkOption { type = types.str; };
            };
            config = mkIf cfg.enable
              {
                services.nginx.virtualHosts."rebalance.tools" = {
                  enableACME = true;
                  forceSSL = true;
                  locations."/" = {
                    proxyPass = "http://127.0.0.1:3000/";
                  };
                };
                services.nginx.virtualHosts."api.rebalance.tools" = {
                  enableACME = true;
                  forceSSL = true;
                  locations."/" = {
                    proxyPass = "http://127.0.0.1:9010/";
                  };
                };
                users.groups = { rebalance = { }; };

                users.users.rebalance = {
                  group = "rebalance";
                  isNormalUser = true;
                };

                systemd.services.rebalance-frontend = {
                  description = "rebalance-frontend server";
                  wantedBy = [ "multi-user.target" ];
                  environment = { };
                  serviceConfig = {
                    User = "rebalance";
                    Group = "rebalance";
                    ExecStart = "${pkgs.nodejs_20}/bin/node ${rebalance-frontend}/index.js";
                    Restart = "on-failure";
                    RestartSec = "10s";
                    WorkingDirectory = "/home/rebalance";
                  };
                };

                systemd.services.rebalance-backend = {
                  description = "rebalance-backend server";
                  wantedBy = [ "multi-user.target" ];
                  environment = {
                    REBALANCE_ENV = "PROD";
                    REBALANCE_API_URL = "https://api.rebalance.tools/";
                    REBALANCE_API_PORT = "9010";
                    REBALANCE_FRONT_URL = "https://rebalance.tools";
                  };
                  serviceConfig = {
                    User = "rebalance";
                    Group = "rebalance";
                    ExecStart = "${rebalance-backend}/bin/rebalance-backend";
                    Restart = "on-failure";
                    RestartSec = "10s";
                    WorkingDirectory = "/home/rebalance";
                    ReadWritePaths = [ "/home/rebalance" ];
                  };
                };

              };
          };

        packages = rec {
          default = packages.all;

          all = pkgs.symlinkJoin {
            name = "all";
            paths = [ rebalance-backend rebalance-frontend ];
          };

          rebalance-backend =
            let
              commonArgs = {
                src = ./api;
                buildInputs = [
                  pkgs.pkg-config
                  pkgs.openssl
                ];
              };
              cargoArtifacts = craneLib.buildDepsOnly commonArgs;
            in
            craneLib.buildPackage (commonArgs // {
              inherit cargoArtifacts;

              checkPhase = ''
                runHook preCheck
                cargo test
              '';
            });

          rebalance-frontend =
            let
              # https://github.com/CarolinaIgnites/editFrame/blob/4e73739bf78e4acb0a4f2fa38b93a8e466d69ea5/flake.nix#L27
              nixified =
                pkgs.stdenv.mkDerivation {
                  name = "rebalance-frontend";
                  # Manually declare output so that we have internet access to pull node
                  # modules. Nix people hate this btw, since it isn't pure- but guess
                  # what it's convenient and removes a moving piece.

                  # Use source
                  # maybe use https://github.com/knarkzel/sveltekit-nix/blob/master/flake.nix
                  src = ./ui;
                  # We need unzip to build this package
                  buildInputs = [ pkgs.node2nix ];
                  buildPhase = ''
                    node2nix --development --strip-optional-dependencies -l package-lock.json
                  '';
                  # Installing simply means copying all files to the output directory
                  installPhase = ''
                    # Build source files and copy them over.
                    mkdir -p $out/
                    cp *.json $out/
                    # lol and but not the flake file since the hash would have to be
                    # dependent on itself.
                    cp default.nix $out/
                    cp node-env.nix $out/
                    cp node-packages.nix $out/
                  '';
                };
              nodeDependencies = (pkgs.callPackage nixified { }).shell.nodeDependencies;
            in
            pkgs.stdenv.mkDerivation {
              name = "rebalance-frontend";
              src = ./ui;
              buildInputs = [ pkgs.nodejs_20 pkgs.nodePackages.tailwindcss ];

              configurePhase = ''
                cp -r ${nodeDependencies}/lib/node_modules ./node_modules
                chmod -R 755 node_modules;
              '';
              buildPhase = ''
                runHook preBuild
                export HOME=$TMP
                export PATH="${nodeDependencies}/bin:$PATH"
                npm run build
                runHook postBuild
              '';
              installPhase = ''
                runHook preInstall
                mkdir -p $out;
                cp package.json $out/;
                cp -r build/* $out/;
                runHook postInstall
              '';
            };
        };
      });
}




