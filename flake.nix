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

        devShells.${system}.default = pkgs.mkShell {

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
        
        packages.${system} = {
            
            dragon-center2 = pkgs.stdenv.mkDerivation {
                
                name = "dragon-center2";
                src = ./.;

                nativeBuildInputs = with pkgs; [
                    gdb
                    gnumake
                    gcc
                    pkg-config
                    glade
                    clang-tools
                ];

                buildInputs = with pkgs; [
                    makeWrapper
                    glib
                    gtk3
                ];

                buildPhase = ''
                    mkdir -p $out/bin
                    make
                    '';
                installPhase = ''
                    cp ./bin/DragonCenter2 $out/bin/
                    '';
            };
        };

    };
}
