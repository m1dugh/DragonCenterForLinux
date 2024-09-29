{ pkgs
, lib
, ...
}:
let
  fs = lib.fileset;
  sourceFiles = fs.difference ./. (fs.unions [
    ./service.nix
    ./flake.nix
    ./flake.lock
    ./default.nix
    ./.gitignore
    ./LICENSE
  ]);
  icon = builtins.fetchurl {
    url = "https://storage-asset.msi.com/event/vga/2018/dragon_center/images/app_logo.png";
    sha256 = "sha256:1ndspljr3s6p9rr8qml3xkyxr4xqn07ijrb12ckqzllxkz48fbjw";
  };
  desktop = pkgs.makeDesktopItem {
    inherit icon;
    name = "dragon-center";
    desktopName = "Dragon Center for linux";
    comment = "MSI Dragon Center for linux";
    genericName = "Dragon Center for linux";
    categories = [ "Settings" ];
    exec = "dragon-center-launcher";
  };

  source = pkgs.rustPlatform.buildRustPackage rec {
    pname = "dragon-center";
    version = "0.0.1";

    src = fs.toSource {
      root = ./.;
      fileset = sourceFiles;
    };

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

    meta.mainProgram = pname;
  };

  launcher = pkgs.writeShellScriptBin "dragon-center-launcher" ''
    pkexec env \
        DISPLAY=$DISPLAY \
        WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
        XDG_SESSION_TYPE=$XDG_SESSION_TYPE \
        XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR \
        ${lib.getExe source}
  '';
in
pkgs.stdenv.mkDerivation {
  src = source;

  name = "dragon-center";

  installPhase = ''
    runHook preInstall
    mkdir -p $out/bin
    ln -s $src/bin/dragon-center $out/bin/dragon-center
    runHook postInstall
  '';

  postInstall = ''
    ln -s ${launcher}/bin/dragon-center-launcher $out/bin/
    mkdir -p $out/share/applications
    ln -s ${desktop}/share/applications/* $out/share/applications
  '';

  meta = {
    mainProgram = "dragon-center";
    licences = lib.licences.mit;
  };
}
