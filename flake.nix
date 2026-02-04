{
  description = "Development environment for mnemo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";

    mnemo-src = {
      url = "github:lemueldls/mnemo";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      mnemo-src,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs { inherit system; });
        mnemo = pkgs.callPackage ./platform/nix/default.nix { inherit mnemo-src; };
        mnemo-bin = pkgs.callPackage ./platform/nix/package.nix { };
      in
      {
        packages = { inherit mnemo mnemo-bin; };
        defaultPackage = mnemo;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            cargo
            cargo-tauri
            nodejs
          ];

          buildInputs = with pkgs; [
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
          ];
        };
      }
    );
}
