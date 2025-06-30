{
  description = "A Nix flake with a development shell and custom command";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:juspay/services-flake";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      process-compose-flake,
      services-flake,
      naersk,
      rust-overlay,
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        # scriptFiles = builtins.attrNames (builtins.readDir ./scripts);
        # scripts = builtins.map (file: import (./scripts + "/${file}") { inherit pkgs; }) scriptFiles;

        toolchain = pkgs.rust-bin.nightly.latest.complete.override {
          targets = [
            "wasm32-unknown-unknown"
          ];
        };

        # naersk' = pkgs.callPackage naersk { };
        #
        # rustPack = naersk'.buildPackage {
        #   src = ./.;
        #   SQLX_OFFLINE = true;
        # };

        compose = import process-compose-flake.lib { inherit pkgs; };

        natsServices = {
          nats-server."nats".enable = true;
        };

        postgresServices = {
          postgres."postgres" = {
            enable = true;
            initialScript.before = ''
              CREATE ROLE myuser WITH LOGIN PASSWORD 'mypasswd' SUPERUSER;
              CREATE USER root SUPERUSER
            '';

            extensions = e: [
              e.timescaledb
            ];

            settings.shared_preload_libraries = "timescaledb";

            initialDatabases = [
              {
                name = "mydb";
                schemas = [
                  ./query/table_1.sql
                  ./query/timescale_table.sql
                  ./query/data_1.sql
                ];
              }
            ];
          };
        };

        serviceMod = services-flake.processComposeModules.default;

        devCompose = compose.evalModules {
          modules = [
            serviceMod
            {
              services = natsServices;
            }
          ];
        };
      in
      {
        packages = {
          default = pkgs.buildEnv {
            name = "test-pcakage";
            paths = [
            ];
          };
        };

        devShells = {
          test = pkgs.mkShell {
            name = "test";

            shellHook = ''echo "test"'';
          };
          minimal = pkgs.mkShell {
            name = "my-dev-shell";

            # Add dependencies here
            buildInputs = [
              # scripts
            ];

          };
          selfpkg = pkgs.mkShell {
            name = "my-dev-shell";

            # Add dependencies here
            buildInputs = [
              # scripts
              devCompose.config.outputs.package
            ];

            shellHook = ''
              tmux new-session -d -s my-session 'process-compose; exec fish'
            '';
          };
          default = pkgs.mkShell {
            name = "my-dev-shell";

            # Add dependencies here
            buildInputs = [
              pkgs.nodejs
              pkgs.yarn
              pkgs.sqlx-cli
              devCompose.config.outputs.package
              # scripts
              toolchain
            ];

            # Set environment variables if needed
            shellHook = ''
              tmux new-session -d -s my-session 'process-compose; exec fish'
            '';
          };
        };

      }
    );
}
