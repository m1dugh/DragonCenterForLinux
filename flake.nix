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

    defaultArgs = {
        inherit (nixpkgs) lib;
        inherit pkgs;
    };

    in {
        devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
                rustc
                rustfmt
                cargo
            ];

            # Environment variable to allow
            # Running on buggy nvidia cards
            __NV_PRIME_RENDER_OFFLOAD = 1;

        };

        packages = rec {
            default = dragon-center-for-linux;
            dragon-center-for-linux = pkgs.callPackage ./default.nix defaultArgs;
        };

        formatter = pkgs.nixpkgs-fmt;
    });
}
