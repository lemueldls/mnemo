{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
{
  # cachix.enable = false;
  dotenv.enable = true;

  packages = with pkgs; [
    # Native
    wrapGAppsHook4
    pkg-config

    # Wasm
    wasm-pack
    binaryen

    # Tauri
    # gobject-introspection
    # cargo-tauri

    # libGL

    # openssl
    # glib
    # pango
    # atk
    # gdk-pixbuf
    # gtk3
    # webkitgtk_4_1
    # cairo
    # atkmm
    # libsoup_3
    # glib-networking
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl

    xdg-utils
  ];

  # scripts = {
  #   setup.exec = ''
  #     wasm-pack build platform/wasm -t web --release
  #   '';
  # };

  # env = {
  #   # OpenSSL
  #   OPENSSL_DIR = "${pkgs.openssl.dev}";
  #   OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  #   OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

  #   # GIO
  #   GIO_MODULE_DIR = "${pkgs.glib-networking.out}/lib/gio/modules/";
  #   GIO_EXTRA_MODULES = "${pkgs.glib-networking.out}/lib/gio/modules/";

  #   WEBKIT_DISABLE_COMPOSITING_MODE = "1";
  #   WEBKIT_DISABLE_DMABUF_RENDERER = "1";
  # };

  enterShell = ''
    export NDK_HOME=$ANDROID_HOME/ndk-bundle
  '';

  android = {
    enable = true;
    # platforms.version = [ "32" "34" ];
    # systemImageTypes = [ "google_apis_playstore" ];
    # abis = [ "arm64-v8a" "x86_64" ];
    # cmdLineTools.version = "8.0";
    # tools.version = "26.1.1";
    # platformTools.version = "35.0.4";
    buildTools.version = [ "34.0.0" ];
    emulator.enable = false;
    sources.enable = false;
    systemImages.enable = false;
    ndk = {
      enable = true;
      version = [ "26.1.10909125" ];
    };
    googleAPIs.enable = false;
    googleTVAddOns.enable = false;
    extras = [ ];
    extraLicenses = [
      # "android-sdk-preview-license"
      # "android-sdk-arm-dbt-license"
      # "google-gdk-license"
      # "intel-android-extra-license"
      # "intel-android-sysimage-license"
      # "mips-android-sysimage-license"
    ];
  };

  # languages = {
  #   javascript = {
  #     enable = true;
  #     corepack.enable = true;
  #     # pnpm.enable = true;
  #   };
  #   typescript = {
  #     enable = true;
  #   };

  #   rust = {
  #     enable = true;
  #     channel = "stable";
  #     targets = [
  #       # Wasm
  #       "wasm32-unknown-unknown"

  #       # Android
  #       "aarch64-linux-android"
  #       "armv7-linux-androideabi"
  #       "i686-linux-android"
  #       "x86_64-linux-android"
  #     ];
  #   };
  # };

  pre-commit.hooks = {
    actionlint.enable = false;
    rustfmt.enable = false;
  };
}
