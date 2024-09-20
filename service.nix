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
  cfg = config.services.dragon-center;

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
  options.hardware.msi.dragon-center = {
    enable = mkEnableOption "Dragon center service";

    package = mkOption {
      type = types.package;
      default = pkgs.dragon-center;
      defaultText = "pkgs.dragon-center";
      description = "The package to use";
    };

    driver = mkOption {
      type = driverOptions;
      description = "The options for the driver";
      default = { };
    };
  };

  config = mkIf cfg.enable {

    boot = mkIf cfg.driver.enable {
      extraModulePackages = [
        cfg.driver.package
      ];

      kernelPackages = config.boot.kernelPackages // {
        msi-ec = cfg.driver.package;
      };
    };

    environment.systemPackages = [
      cfg.package
    ];
  };
}
