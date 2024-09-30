{ config
, lib
, pkgs
, ...
}:
let
  inherit (lib)
    mkOption
    mkEnableOption
    types
    mkIf
    ;
  cfg = config.services.msi-dragon-center;

  driverOptions = {
    options = {
      enable = mkEnableOption "MSI ec driver config";

      package = mkOption {
        type = types.package;
        default = pkgs.msi-ec-kmods;
        defaultText = "pkgs.msi-ec-kmods";
        description = "The driver package to use";
      };
    };
  };
in
{
  options.services.msi-dragon-center = {
    enable = mkEnableOption "Dragon center service";

    package = mkOption {
      type = types.package;
      default = pkgs.dragon-center;
      defaultText = "pkgs.dragon-center";
      description = "The package to use";
    };

    logLevel = mkOption {
      type = types.enum [
        "debug"
        "info"
        "warn"
        "error"
      ];
      default = "info";
      description = "The log level";
    };

    driver = mkOption {
      type = types.submodule driverOptions;
      description = "The options for the driver";
      default = { };
    };
  };

  config = mkIf cfg.enable {

    boot = mkIf cfg.driver.enable {
      extraModulePackages = [
        cfg.driver.package
      ];
      kernelModules = [
        "msi-ec"
      ];
    };

    users.groups.dragon-center.gid = lib.mkDefault 990;

    systemd.services.msi-dragon-center = {
      enable = true;
      wantedBy = [ "multi-user.target" ];

      description = "the msi dragon center daemon";

      serviceConfig = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/dragon-center-daemon";
      };

      environment = {
        DRAGON_CENTER_NO_FORK = toString 1;
        DRAGON_CENTER_GID = toString config.users.groups.dragon-center.gid;
        RUST_LOG = cfg.logLevel;
      };
    };

    environment.systemPackages = [
      cfg.package
    ];
  };
}
