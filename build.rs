
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=css");                   // tell cargo to add the css lib
    println!("cargo:rerun-if-changed=libcss-wrapper.h");    // tell cargo to rebuild if libcss-wrapper.h changes

    let nix_cflags_compile = env::var("NIX_CFLAGS_COMPILE").unwrap_or_default();

    let bindings = bindgen::Builder::default()
        .header("libcss-wrapper.h")
        .clang_args(nix_cflags_compile.split_whitespace())
        .generate()
        .expect("Unable to generate bindings!")
        ;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
        ;
}
