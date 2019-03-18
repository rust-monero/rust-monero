extern crate bindgen;
extern crate cc;

use std::env;

fn main() {
    let mut build = cc::Build::new();
    let tool = build.get_compiler();
    if tool.is_like_clang() || tool.is_like_gnu() {
        //        build.flag_if_supported("-std=c99");
        build
            .flag_if_supported("-msse4.1")
            .flag_if_supported("-maes");
    }

    build.warnings(false);
    build
        .file("ccrypto/src/aesb.c")
        .file("ccrypto/src/blake256.c")
        .file("ccrypto/src/crypto-ops-data.c")
        .file("ccrypto/src/crypto-ops.c")
        .file("ccrypto/src/groestl.c")
        .file("ccrypto/src/hash-extra-blake.c")
        .file("ccrypto/src/hash-extra-groestl.c")
        .file("ccrypto/src/hash-extra-jh.c")
        .file("ccrypto/src/hash-extra-skein.c")
        .file("ccrypto/src/hash.c")
        .file("ccrypto/src/jh.c")
        .file("ccrypto/src/keccak.c")
        .file("ccrypto/src/oaes_lib.c")
        .file("ccrypto/src/random.c")
        .file("ccrypto/src/skein.c")
        .file("ccrypto/src/slow-hash.c")
        .file("ccrypto/src/tree-hash.c")
        .include("ccrypto/src");

    build.compile("ccrypto");

    //    println!("cargo:rustc-link-lib=bz2");

    //    let bindings = bindgen::Builder::default()
    //        .header("ccrypto/src/crypto-ops.h")
    //        .command_line_flags()
    //        .generate()
    //        .expect("Unable to generate bindings");
    //    bindings.write_to_file("src/bindings.rs")
    //        .expect("Couldn't write bindings!");
}
