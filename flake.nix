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
    , self
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

      in
      {
        packages = rec {
          default = dragon-center;
          dragon-center = pkgs.rustPlatform.buildRustPackage {
            pname = "dragon-center";
            version = "0.0.1";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;

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
              curl
              wget
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
          };
        };

        formatter = pkgs.nixpkgs-fmt;
      }) // {
      nixosModules.default = import ./service.nix;
    };
}
