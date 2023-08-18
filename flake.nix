{
    description = "The MSI Dragon Center 2 utility for Linux";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    outputs = {
        self,
        nixpkgs
    } @ inputs :
    let
        system = "x86_64-linux";
        inherit (nixpkgs) lib;
        pkgs = nixpkgs.legacyPackages.${system};
    in {

        devShells.${system}={

            default = pkgs.mkShell {

                nativeBuildInputs = with pkgs; [
                    gdb
                    gnumake
                    gcc
                    pkg-config
                    glade
                    clang-tools
                ];

                buildInputs = with pkgs; [
                    glib
                    gtk3
                ];
            };
        };

        packages.${system} = {

            dragon-center2 = pkgs.stdenv.mkDerivation {

                name = "DragonCenter";
                src = ./.;

                nativeBuildInputs = with pkgs; [
                    gnumake
                    gcc
                    pkg-config
                ];

                buildInputs = with pkgs; [
                    glib
                    gtk3
                ];

                configurePhase = ''
                    mkdir -p $out/bin
                    '';

                buildPhase = ''
                    make
                    '';

                installPhase = ''
                    cp -r ./resources "$out/"
                    install -D ./bin/Release/DragonCenter2 -m 0555 "$out/bin/$name"
                    '';
            };

            default = self.packages.${system}.dragon-center2;
        };

        apps.${system}.default = 
            let
            mypkgs = self.packages.${system};
        in {
            type = "app";
            program = "${mypkgs.default}/bin/DragonCenter";
        };

    };
}
