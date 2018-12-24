extern crate cc;


const CCRYPTO_PATH: &str = "ccrypto";
const CCRYPTO_SOURCE_PATH: &str = CCRYPTO_PATH + "/src";


fn main() {
    let mut build = cc::Build::new();
    let tool = build.get_compiler();
    if tool.is_like_clang() || tool.is_like_gnu() {
        build.flag_if_supported("-std=c99")
            .flag_if_supported("-msse4.1")
            .flag_if_supported("-maes")
    }

    build.warnings(false);
    build.file(CCRYPTO_SOURCE_PATH + "aesb.c")
        .file(CCRYPTO_SOURCE_PATH + "blake256.c")
        .file(CCRYPTO_SOURCE_PATH + "crypto-ops-data.c")
        .file(CCRYPTO_SOURCE_PATH + "crypto-ops.c")
        .file(CCRYPTO_SOURCE_PATH + "groestl.c")
        .file(CCRYPTO_SOURCE_PATH + "hash-extra-blake.c")
        .file(CCRYPTO_SOURCE_PATH + "hash-extra-groestl.c")
        .file(CCRYPTO_SOURCE_PATH + "hash-extra-jh.c")
        .file(CCRYPTO_SOURCE_PATH + "hash-extra-skein.c")
        .file(CCRYPTO_SOURCE_PATH + "hash.c")
        .file(CCRYPTO_SOURCE_PATH + "jh.c")
        .file(CCRYPTO_SOURCE_PATH + "keccak.c")
        .file(CCRYPTO_SOURCE_PATH + "oaes_lib.c")
        .file(CCRYPTO_SOURCE_PATH + "random.c")
        .file(CCRYPTO_SOURCE_PATH + "skein.c")
        .file(CCRYPTO_SOURCE_PATH + "slow-hash.c")
        .file(CCRYPTO_SOURCE_PATH + "tree-hash.c")
        .include(CCRYPTO_SOURCE_PATH);

    build.compile("ccrypto");
}