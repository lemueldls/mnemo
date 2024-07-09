{
  inputs = { utils.url = "github:numtide/flake-utils"; };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
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
            ];

            GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules";
          };
        }
      );
}
