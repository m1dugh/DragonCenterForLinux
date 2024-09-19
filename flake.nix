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
            pkgs.mkShell {
              nativeBuildInputs = with pkgs; [
                rustc
                rustfmt
                cargo
              ];

              LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${lib.makeLibraryPath (with pkgs; [
                wayland
                libxkbcommon
                fontconfig
              ])}";

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
