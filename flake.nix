{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          packages = with pkgs; [
            tree
          ];

          buildInputs = [
            libclang.lib
            pkg-config
            pre-commit

            cargo
            rustc
            rustfmt

            libgcc
            netsurf.libparserutils
            netsurf.libwapcaplet
            netsurf.libcss

          ];

          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          RUST_SRC_PATH = rustPlatform.rustLibSrc;

          NS_LIBPARSERUTILS = "${pkgs.netsurf.libparserutils}";
          NS_LIBWAPCAPLET = "${pkgs.netsurf.libwapcaplet}";
          NS_LIBCSS = "${pkgs.netsurf.libcss}";
        };
      }
    );
}
