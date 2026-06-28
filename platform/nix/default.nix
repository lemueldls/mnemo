{
  lib,
  stdenv,

  cargo-tauri,
  cmake,
  curl,
  glib-networking,
  nodejs,
  openssl,
  pkg-config,
  fetchPnpmDeps,
  pnpmConfigHook,
  pnpm,
  rust,
  rustPlatform,
  webkitgtk_4_1,
  wrapGAppsHook4,

  mnemo-src ? ../../.,
  apiBaseUrl ? "https://mnemo.world",
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "mnemo";
  version = "0.3.1";

  src = mnemo-src;

  cargoRoot = "platform";
  cargoHash = "sha256-XC6ulPbHS66ft660Qd/m9vO+rsetwxIEanF0VxU2MIY=";

  buildAndTestSubdir = "${finalAttrs.cargoRoot}/tauri";

  pnpmDeps = fetchPnpmDeps {
    inherit (finalAttrs) pname version src;
    fetcherVersion = 3;
    hash = "sha256-nnHt4hTwAQecN6u4NJNBGsD5jvTGrf1MOaLnimy2wI8=";
  };

  nativeBuildInputs = [
    cargo-tauri.hook
    cmake
    nodejs
    pkg-config
    pnpmConfigHook
    pnpm
    wrapGAppsHook4
  ];

  buildInputs = [
    openssl
  ]
  ++ lib.optional stdenv.hostPlatform.isDarwin curl
  ++ lib.optionals stdenv.hostPlatform.isLinux [
    glib-networking
    webkitgtk_4_1
  ];

  tauriBuildFlags = [
    "--config"
    "tauri.package.conf.json"
  ];

  env = {
    # `fetchPnpmDeps` and `pnpmConfigHook` use a specific version of pnpm, not upstream's
    COREPACK_ENABLE_STRICT = 0;

    OPENSSL_NO_VENDOR = true;

    NUXT_TELEMETRY_DISABLED = 1;
    NUXT_PUBLIC_API_BASE_URL = apiBaseUrl;
  };

  meta = {
    description = "Local-first note-taking app leveraging the Typst ecosystem.";
    homepage = "https://mnemo.world";
    changelog = "https://github.com/lemueldls/mnemo/releases/tag/mnemo-v${finalAttrs.version}";
    license = lib.licenses.agpl3Only;
    # maintainers = with lib.maintainers; [ lemueldls ];
    mainProgram = "mnemo";
    platforms = [ "x86_64-linux" ];
  };
})
