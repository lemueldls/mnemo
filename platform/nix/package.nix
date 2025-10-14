{
  stdenvNoCC,
  lib,
  fetchurl,
  dpkg,
  rustPlatform,
  cargo-tauri,
  npmHooks,
  pnpm,
  pkg-config,
  python3,
  nodejs,
  webkitgtk_4_1,
  glib,
  gtk3,
  openssl,
  pango,
  cairo,
  pixman,
  protobuf,
  perl,
  makeWrapper,
  nix-update-script,
}:
stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "mnemo";
  version = "0.1.5";

  src = fetchurl {
    url = "https://github.com/lemueldls/mnemo/releases/download/mnemo-v${finalAttrs.version}/Mnemo_${finalAttrs.version}_amd64.deb";
    hash = "sha256-znoj7gPHGDD90OedOFyn4+TNEiR4dLdj17FNAQ7+hSM=";
  };

  unpackCmd = "dpkg -x $curSrc source";

  nativeBuildInputs = [
    dpkg
    cargo-tauri.hook
    pkg-config
    nodejs
    python3
    protobuf
    perl
    makeWrapper
  ];

  buildInputs = [
    glib
    gtk3
    openssl
    webkitgtk_4_1
    pango
    cairo
    pixman
  ];

  dontConfigure = true;
  dontBuild = true;

  dontStrip = true;

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin
    mkdir -p $out/share

    cp -r usr/bin/mnemo $out/bin/mnemo
    cp -r usr/share/applications $out/share/applications
    cp -r usr/share/icons $out/share/icons
    cp -r usr/share/licenses $out/share/licenses

    runHook postInstall
  '';

  meta = {
    description = "Note-taking app designed to enhance the retention of information.";
    homepage = "https://mnemo.nuxt.dev/";
    changelog = "https://github.com/lemueldls/mnemo/releases/tag/mnemo-v${finalAttrs.version}";
    license = lib.licenses.agpl3Only;
    maintainers = with lib.maintainers; [ lemueldls ];
    mainProgram = "mnemo";
    platforms = [ "x86_64-linux" ];
  };
})
