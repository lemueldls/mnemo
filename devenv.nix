{ pkgs, lib, config, inputs, ... }: {
  certificates = [
    "localhost"
    "localhost:3000"
  ];

  packages = with pkgs; [
    # Native
    pkg-config

    # Tauri
    openssl
    glib
    pango
    atk
    gdk-pixbuf
    gtk3
    webkitgtk_4_1
    cairo
    atkmm
    libsoup_3
    glib-networking
  ];

  # scripts = {
  #   setup.exec = ''
  #     yarn install --frozen-lockfile
  #     yarn wasm-pack build backend/wasm -t web --release
  #   '';
  # };

  env = {
    # OpenSSL
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

    # Network
    GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules";
  };

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
    # platformTools.version = "34.0.4";
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

  processes.supabase = {
    exec = "yarn supabase start";
    # process-compose.shutdown = {
    #   command = "yarn supabase stop";
    #   signal = 15;
    #   timeout_seconds = 30;
    # };
  };

  languages = {
    javascript = {
      enable = true;
      corepack.enable = true;
      # yarn = {
      #   enable = true;
      #   package = pkgs.yarn-berry;
      # };
    };
    typescript = {
      enable = true;
    };

    rust = {
      enable = true;
      channel = "stable";
      targets = [
        # Wasm
        "wasm32-unknown-unknown"

        # Android
        "aarch64-linux-android"
        "armv7-linux-androideabi"
        "i686-linux-android"
        "x86_64-linux-android"
      ];
    };
  };

  pre-commit.hooks = {
    actionlint.enable = true;
    rustfmt.enable = false;
  };
}
