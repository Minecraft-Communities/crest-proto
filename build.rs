
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=css");                   // tell cargo to add the css lib
    println!("cargo:rerun-if-changed=libcss-wrapper.h");    // tell cargo to rebuild if libcss-wrapper.h changes


    let libparserutils_dir = env::var("NS_LIBPARSERUTILS").expect("Required environment variable 'NS_LIBPARSERUTILS' not set!");
    let libwapcaplet_dir = env::var("NS_LIBWAPCAPLET").expect("Required environment variable 'NS_LIBWAPCAPLET' not set!");
    let libcss_dir = env::var("NS_LIBCSS").expect("Required environment variable 'NS_LIBCSS' not set!");

    let libparserutils_dir_include = format!("{libparserutils_dir}/include");
    let libwapcaplet_dir_include = format!("{libwapcaplet_dir}/include");
    let libcss_dir_include = format!("{libcss_dir}/include");

    let bindings = bindgen::Builder::default()
        .header("libcss-wrapper.h")

        .clang_arg(format!("-I{}", libparserutils_dir_include))
        .clang_arg(format!("-I{}", libwapcaplet_dir_include))
        .clang_arg(format!("-I{}", libcss_dir_include))

        .generate()
        .expect("Unable to generate bindings!")
        ;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
        ;
}
