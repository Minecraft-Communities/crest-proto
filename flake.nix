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
            glibc.dev
            netsurf.libparserutils
            netsurf.libwapcaplet
            netsurf.libcss

          ];

          LIBCLANG_PATH =           "${pkgs.clang.passthru.cc.lib}/lib";
          CLANG_STD_INCLUDE_PATH =  "${pkgs.clang.passthru.cc.dev}/include";
          LIBGCC_PATH =             "${pkgs.gcc13}";
          GLIBC_INCLUDE =           "${pkgs.glibc.dev}/include";
          RUST_SRC_PATH =           rustPlatform.rustLibSrc;
        };
      }
    );
}
