{
  description = "The MSI Dragon Center 2 utility for Linux";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    systems.url = "github:nix-systems/default-linux";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs =
    { flake-utils
    , nixpkgs
    , ...
    }:
    flake-utils.lib.eachDefaultSystem
      (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        inherit (nixpkgs) lib;
        defaultArgs = {
          inherit pkgs lib;
        };

            buildInputs = with pkgs; [
              webkitgtk_4_1
              gtk3
              cairo
              gdk-pixbuf
              glib
              dbus
              openssl_3
              librsvg
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
              dbus
              openssl_3
              glib
              gtk3
              libsoup
              webkitgtk_4_1
              librsvg
              cargo
              cargo-tauri
              pkg-config
            ];
      in
      {
        devShells = {
            default = pkgs.mkShell {
              XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}";
              inherit nativeBuildInputs buildInputs;
            };
        };
        packages = rec {
          default = dragon-center;
          dragon-center = pkgs.writeShellScriptBin "dragon-center" ''
              XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS";
              ${pkgs.polkit}/bin/pkexec ${dragon-center-unwrapped} "$@"
          '';
          dragon-center-unwrapped = pkgs.rustPlatform.buildRustPackage {
            pname = "dragon-center";
            version = "0.0.1";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;

            inherit buildInputs nativeBuildInputs;

            doCheck = false;

          };
        };

        formatter = pkgs.nixpkgs-fmt;
      }) // {
      nixosModules.default = import ./service.nix;
    };
}
