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
            clang
            gcc13
          ];

          buildInputs = [
            clang
            pkg-config
            pre-commit

            cargo
            rustc
            rustfmt

            gcc13
            netsurf.libparserutils
            netsurf.libwapcaplet
            netsurf.libcss
          ];

          LIBCLANG_PATH =           "${pkgs.clang.passthru.cc.lib}/lib";
          RUST_SRC_PATH =           rustPlatform.rustLibSrc;

          LIBCSS = "${netsurf.libcss}";
        };
      }
    );
}
