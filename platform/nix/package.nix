{
  stdenv,
  lib,
  fetchurl,
  autoPatchelfHook,
  dpkg,
  openssl,
  webkitgtk_4_1,
  libappindicator,
  wrapGAppsHook4,
  shared-mime-info,
  glib-networking,
}:

stdenv.mkDerivation (finalAttrs: {
  pname = "mnemo";
  version = "0.3.1";

  src = fetchurl {
    url = "https://github.com/lemueldls/mnemo/releases/download/mnemo-v${finalAttrs.version}/Mnemo_${finalAttrs.version}_amd64.deb";
    sha256 = "sha256-y2LVhF0Y4oFw+n9wn1poCRt3Ob9I1IrJuAOVStHMQ3A=";
  };

  dontConfigure = true;
  dontBuild = true;

  nativeBuildInputs = [
    dpkg
    autoPatchelfHook
    wrapGAppsHook4
  ];

  buildInputs = [
    webkitgtk_4_1
  ];

  installPhase = ''
    runHook preInstall

    install -Dm755 usr/bin/mnemo $out/bin/${finalAttrs.meta.mainProgram}
    cp -r usr/share $out

    wrapProgram "$out/bin/${finalAttrs.meta.mainProgram}" \
      --prefix GIO_EXTRA_MODULES : "${glib-networking}/lib/gio/modules"

    runHook postInstall
  '';

  meta = {
    description = "Local-first note-taking app leveraging the Typst ecosystem.";
    homepage = "https://mnemo.world";
    changelog = "https://github.com/lemueldls/mnemo/releases/tag/mnemo-v${finalAttrs.version}";
    license = lib.licenses.agpl3Only;
    maintainers = with lib.maintainers; [ lemueldls ];
    mainProgram = "mnemo";
    platforms = [ "x86_64-linux" ];
  };
})
