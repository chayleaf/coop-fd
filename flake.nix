{
  inputs.nixpkgs.url = "github:chayleaf/nixpkgs";
  inputs.crane.url = "github:ipetkov/crane";
  inputs.crane.inputs.nixpkgs.follows = "nixpkgs";
  inputs.flake-compat = {
    url = "github:edolstra/flake-compat";
    flake = false;
  };

  outputs = { crane, nixpkgs, ... }: let
    inherit (nixpkgs) lib;
    mkPkgs = pkgs: let
      craneLib = crane.mkLib pkgs;
    in rec {
      default = coop-fd;
      coop-fd = craneLib.buildPackage {
        pname = "coop-fd";
        version = "0.1.0";
        src = nixpkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            builtins.any (lib.flip lib.hasSuffix path) [ ".js" ".map" ".html" ".css" ]
            || craneLib.filterCargoSources path type;
        };
        doCheck = false;
      };
    };
  in {
    devShells = lib.genAttrs ["x86_64-linux" "aarch64-linux"] (system: let
      pkgs = import nixpkgs { inherit system; };
    in rec {
      default = coop-fd;
      coop-fd = import ./shell.nix { inherit pkgs; };
    });
    nixosModules = rec {
      default = coop-fd;
      coop-fd = { lib, config, pkgs, ... }: let
        cfg = config.services.coop-fd;
        format = pkgs.formats.json { };
        cfgFile = format.generate "config.json" cfg.config;
      in {
        options.services.coop-fd = {
          enable = lib.mkEnableOption "coop-fd";
          package = lib.mkOption { type = lib.types.package; };
          config = lib.mkOption {
            type = lib.types.submodule {
              options.usernames = lib.mkOption {
                type = with lib.types; listOf str;
              };
              options.listener = lib.mkOption {
                type = lib.types.str;
              };
              options.ignore_qr_condition = lib.mkOption {
                type = lib.types.str;
                default = "false";
              };
              options.data_path = lib.mkOption {
                type = lib.types.path;
                default = "/var/lib/coop-fd";
              };
              options.irkkt_mobile_client_secret = lib.mkOption {
                type = with lib.types; nullOr str;
                default = null;
              };
              options.irkkt_mobile_device_id = lib.mkOption {
                type = with lib.types; nullOr str;
                default = null;
              };
              options.irkkt_mobile_api_base = lib.mkOption {
                type = with lib.types; nullOr str;
                default = null;
              };
              options.private1_endpoint = lib.mkOption {
                type = with lib.types; nullOr str;
                default = null;
              };
            };
          };
        };
        config = lib.mkIf cfg.enable {
          services.coop-fd.package = lib.mkOptionDefault (mkPkgs pkgs).default;
          systemd.services.coop-fd = {
            description = "coop-fd";
            after = [ "network.target" ];
            wantedBy = [ "multi-user.target" ];
            environment.CONFIG_FILE = cfgFile;
            environment.RUST_LOG = "info";
            serviceConfig = {
              DynamicUser = true;
              ExecStart = "${cfg.package}/bin/coop-fd";
              Restart = "on-failure";
              RestartSec = "10s";
              StateDirectory = "coop-fd";
              WorkingDirectory = "/var/lib/coop-fd";
            };
          };
        };
      };
    };
    overlays = rec {
      default = coop-fd;
      coop-fd = final: prev: mkPkgs final;
    };
    packages = lib.genAttrs ["x86_64-linux" "aarch64-linux"] (system: mkPkgs (import nixpkgs { inherit system; }));
  };
}
