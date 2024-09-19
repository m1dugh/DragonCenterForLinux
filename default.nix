{ pkgs
, lib
, ...
}: pkgs.rustPlatform.buildRustPackage rec {
  pname = "dragon-center";
  version = "0.0.1";

  src = ./.;

  libraryPath = with pkgs; [
      wayland
      libxkbcommon
      fontconfig
  ];

  cargoLock.lockFile = ./Cargo.lock;

  enableParallelBuilding = true;

  nativeBuildInputs = with pkgs; [
    pkg-config
  ] ++ libraryPath;

  postFixup = ''
    patchelf $out/bin/dragon-center \
        --set-rpath ${lib.makeLibraryPath libraryPath}
  '';

  meta = {
    mainProgram = "dragon-center";
    licences = lib.licences.mit;
  };
}
