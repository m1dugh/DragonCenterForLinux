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
    inherit (nixpkgs) lib;
    in rec {
        devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
                pkg-config
                libsoup
                cargo-tauri
                glibc
                cairo
                gtk3
                webkitgtk
            ];

            # Environment variable to allow
            # Running on buggy nvidia cards
            __NV_PRIME_RENDER_OFFLOAD = 1;

        };
        apps = rec {
            default = dragon-center-for-linux;

            dragon-center-for-linux = {
                type = "app";
                program = "${packages.dragon-center-for-linux}/bin/dragon-center-for-linux";
            };
        };
        packages = rec {
            default = dragon-center-for-linux;
            dragon-center-for-linux = 
            let libs = with pkgs; [
                libsoup
                glibc
                cairo
                gtk3
                webkitgtk
            ];
            in pkgs.rustPlatform.buildRustPackage {
                pname = "dragon-center-for-linux";
                version = "0.0.1";

                src = ./.;

                cargoLock.lockFile = ./Cargo.lock;

                __NV_PRIME_RENDER_OFFLOAD = 1;

                PKG_CONFIG_PATH = lib.strings.concatMapStringsSep ":" (lib: "${lib}/lib/pkgconfig") libs;
                nativeBuildInputs = with pkgs; [
                    pkg-config
                ] ++ libs;

            };
        };
    });
}
