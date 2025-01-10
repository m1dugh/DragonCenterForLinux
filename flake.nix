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
        devShells = {
          default = self.devShells.${system}.wayland;
          wayland =
            let
              libraries = with pkgs;[
                webkitgtk_4_1
                gtk3
                cairo
                gdk-pixbuf
                glib
                dbus
                openssl_3
                librsvg
              ];
              packages = with pkgs; [
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
            in
            pkgs.mkShell {
              buildInputs = packages;

              LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${lib.makeLibraryPath libraries}";
              PKG_CONFIG_PATH = lib.strings.concatMapStringsSep ":" (pkg: "${pkg.dev}/lib/pkgconfig/") libraries;

              # Environment variable to allow
              # Running on buggy nvidia cards
              __NV_PRIME_RENDER_OFFLOAD = 1;

            };
        };

        packages = rec {
          default = dragon-center;
          dragon-center = pkgs.callPackage ./default.nix defaultArgs;
        };

        formatter = pkgs.nixpkgs-fmt;
      }) // {
      nixosModules.default = import ./service.nix;
    };
}
