{ config
, lib
, pkgs
, ...
}:
with lib;
let
    cfg = config.services.dragon-center;
in {
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
    };

    config = mkIf cfg.enable {

        boot = mkIf cfg.withBootOptions {
            kernelModules = ["ec_sys"];
            extraModprobeConfig = ''
                options ec_sys write_support=1
            '';
        };
            
        systemd.services.dragon-center = {
            path = [
                cfg.package
            ];

            serviceConfig = {
                ExecStart = "${getExe cfg.package}";
                Type = "simple";
                User = "root";
                RestartSec = "5s";
                Restart = "on-failure";
            };
        };
    };
}
