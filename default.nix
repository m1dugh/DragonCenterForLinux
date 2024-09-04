{ pkgs
, lib
, ...
}: pkgs.rustPlatform.buildRustPackage rec {
  pname = "dragon-center";
  version = "0.0.1";

  src = ./.;

  libraryPath = with pkgs; [
    glibc
  ];

  cargoLock.lockFile = ./Cargo.lock;

  LD_LIBRARY_PATH = lib.makeLibraryPath libraryPath;

  nativeBuildInputs = with pkgs; [
    pkg-config
  ] ++ libraryPath;

  postFixup = ''
    mkdir -p $out/share
    install -m 0644 $src/config.yaml $out/share/
  '';

  meta = {
    mainProgram = "dragon-center";
    licences = lib.licences.mit;
  };
}
