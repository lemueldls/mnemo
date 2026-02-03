{
  lib,
  rustPlatform,
  cargo-tauri,
  npmHooks,
  fetchFromGitHub,
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

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "mnemo";
  version = "0.1.5";

  src = fetchFromGitHub {
    owner = "lemueldls";
    repo = "mnemo";
    tag = "mnemo-v${finalAttrs.version}";
    hash = "sha256-PjOUwLr9GIVoAel+VxRYqEXYPumGZE5/fYuuSqikNNA=";
  };

  # pnpmDeps = pnpm.fetchDeps {
  #   inherit (finalAttrs) pname version src;
  #   hash = "sha256-4D7ETUOLixpFB4luqQlwkGR/C6Ke6+ZmPg3dKKkrw7c=";
  # };

  cargoHash = "sha256-OyjBAgcLswOvI+mG4NK8atGX5q4d1T+2mtelpMSPJjQ=";

  cargoRoot = "platform/tauri";
  cargoLock = {
    lockFile = finalAttrs.src + /platform/Cargo.lock;
    # outputHashes = {
    #   "codex-0.1.1" = "";
    #   "krilla-0.4.0" = "";
    #   "typst-0.13.1" = "";
    };
  };

  # nativeBuildInputs = [
  #   cargo-tauri.hook
  #   # npmHooks.npmConfigHook
  #   pkg-config
  #   nodejs
  #   python3
  #   protobuf
  #   perl
  #   makeWrapper
  # ];

  # buildInputs = [
  #   glib
  #   gtk3
  #   openssl
  #   webkitgtk_4_1
  #   pango
  #   cairo
  #   pixman
  # ];

  # env.ELECTRON_SKIP_BINARY_DOWNLOAD = "1";

  # postPatch = ''
  #   substituteInPlace platform/tauri/tauri.conf.json \
  #     --replace-fail '"createUpdaterArtifacts": "v1Compatible"' '"createUpdaterArtifacts": false'
  #   substituteInPlace package.json \
  #     --replace-fail '"bootstrap:vendor-node": "node scripts/vendor-node.cjs",' "" \
  #     --replace-fail '"bootstrap:vendor-protoc": "node scripts/vendor-protoc.cjs",' ""
  # '';

  # preBuild = ''
  #   mkdir -p platform/tauri/vendored/node
  #   ln -s ${nodejs}/bin/node platform/tauri/vendored/node/mnemonode-x86_64-unknown-linux-gnu
  #   mkdir -p platform/tauri/vendored/protoc
  #   ln -s ${protobuf}/bin/protoc platform/tauri/vendored/protoc/mnemoprotoc-x86_64-unknown-linux-gnu
  #   ln -s ${protobuf}/include platform/tauri/vendored/protoc/include
  # '';

  # # Permission denied (os error 13)
  # # write to platform/tauri/vendored/protoc/include
  # doCheck = false;

  # preInstall = "pushd platform/tauri";

  # postInstall = "popd";

  # postFixup = ''
  #   wrapProgram $out/bin/mnemo-app \
  #     --inherit-argv0 \
  #     --set-default WEBKIT_DISABLE_DMABUF_RENDERER 1
  # '';

  # passthru.updateScript = nix-update-script { };

  meta = {
    description = "Note-taking app designed to enhance the retention of information.";
    # homepage = "https://mnemo.app/";
    # changelog = "https://github.com/lemueldls/mnemo/releases/tag/v${finalAttrs.version}";
    license = lib.licenses.agpl3Only;
    maintainers = with lib.maintainers; [ lemueldls ];
    mainProgram = "mnemo";
    platforms = [ "x86_64-linux" ];
  };
})
