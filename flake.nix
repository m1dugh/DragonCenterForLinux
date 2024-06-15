{
    description = "The MSI Dragon Center 2 utility for Linux";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
        systems.url = "github:nix-systems/default-linux";

        flake-utils = {
            url = "github:numtide/flake-utils";
            inputs.systems.follows = "systems";
        };
    };

    outputs = {
        flake-utils,
        nixpkgs,
        ...
    }:
    flake-utils.lib.eachDefaultSystem(system: 
    let pkgs = import nixpkgs {
        inherit system;
    };
    in rec {
        apps = rec {
            default = dragon-center-for-linux;

            dragon-center-for-linux = {
                type = "app";
                program = "${packages.dragon-center-for-linux}/bin/dragon-center-for-linux";
            };
        };
        packages = rec {
            default = dragon-center-for-linux;
            dragon-center-for-linux = pkgs.rustPlatform.buildRustPackage {
                pname = "dragon-center-for-linux";
                version = "0.0.1";

                src = ./.;

                cargoLock.lockFile = ./Cargo.lock;

                nativeBuildInputs = with pkgs; [
                    pkg-config
                ];
            };
        };
    });
}
