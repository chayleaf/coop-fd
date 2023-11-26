{ lib
, config
, pkgs
, ...
}:

let
  cfg = config.services.coop-ofd;
  format = pkgs.formats.json { };
  cfgFile = format.generate "config.json" cfg.config;
in
{
  options.services.coop-ofd = {
    enable = lib.mkEnableOption "coop-ofd";
    package = lib.mkOption { type = lib.types.package; };
    config = lib.mkOption {
      type = lib.types.submodule {
        freeformType = format.type;
        options.data_path = lib.mkOption {
          type = lib.types.path;
          default = "/var/lib/coop-ofd";
        };
      };
    };
  };
  config = lib.mkIf cfg.enable {
    systemd.services.coop-ofd = {
      description = "coop-ofd";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      environment.CONFIG_FILE = cfgFile;
      serviceConfig = {
        DynamicUser = true;
        ExecStart = "${cfg.package}/bin/coop-ofd";
        Restart = "on-failure";
        RestartSec = "10s";
        StateDirectory = "coop-ofd";
        WorkingDirectory = "/var/lib/coop-ofd";
      };
    };
  };
}
