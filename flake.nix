{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    android = { url = "github:tadfisher/android-nixpkgs"; };
  };

  outputs = { self, nixpkgs, utils, android }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          # android-sdk = android.sdk.${system} (sdkPkgs: with sdkPkgs; [
          #   cmdline-tools-latest
          #   build-tools-34-0-0
          #   platform-tools
          #   platforms-android-34
          #   ndk-23-0-7599858
          # ]);
        in
        {
          overlay = final: prev: {
            inherit (self.packages.${final.system}) android-sdk;
          };

          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              # Tauri

              ## Desktop
              pkg-config
              openssl #.out
              glib #.out
              pango
              atk
              gdk-pixbuf
              gtk3
              webkitgtk_4_1
              cairo
              glib-networking

              # ## Android
              # android-sdk
            ];

            # OpenSSL
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

            # Network
            GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules";

            # # Android
            # ANDROID_HOME = "${android-sdk}/share/android-sdk";
            # ANDROID_SDK_ROOT = "${android-sdk}/share/android-sdk";
            # NDK_HOME = "${android-sdk}/share/android-sdk/ndk/23.0.7599858";
          };
        }
      );
}
