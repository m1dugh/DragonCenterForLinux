{ config
, lib
, pkgs
, ...
}:
with lib;
let
  cfg = config.services.dragon-center;
  format = pkgs.formats.yaml { };
in
{
  options.services.dragon-center = {
    enable = mkEnableOption "Dragon center service";

    package = mkOption {
      type = types.package;
      default = pkgs.dragon-center;
      defaultText = "pkgs.dragon-center";
      description = "The package to use";
    };

    withBootOptions = mkOption {
      default = true;
      type = types.bool;
      description = "Whether to add the boot options for embedded controller";
    };

    layout = mkOption {
        type = types.nullOr format.type;
        default = null;
        description = ''
            The embedded controller layout to use for this computer.
        '';
    };

    layoutFile = mkOption {
        type = types.nullOr types.path;
        default = "${cfg.package}/share/config.yaml";
        description = ''
            The path to the embedded controller layout to use for this computer
        '';
    };
  };

  config = mkIf cfg.enable {

    assertions = [{
        assertion = xor (cfg.layoutFile == null) (cfg.layout == null);
        message = "Only one of `services.dragon-center.layoutFile` and `services.dragon-center.layout` should be set";
    }];

    boot = mkIf cfg.withBootOptions {
      kernelModules = [ "ec_sys" ];
      extraModprobeConfig = ''
        options ec_sys write_support=1
      '';
    };

    environment.systemPackages = [
      cfg.package
    ];

    systemd.services.dragon-center = 
    let
        configFile = if cfg.layout != null then format.generate "config.yaml" cfg.layout
        else cfg.layoutFile;
    in {
      wantedBy = [ "multi-user.target" ];

      description = "starts the daemon for dragon center";
      path = [
        cfg.package
      ];

      serviceConfig = {
        ExecStart = "${getExe cfg.package} --config ${configFile}";
        Type = "simple";
        User = "root";
        RestartSec = "5s";
        Restart = "on-failure";
      };
    };
  };
}
