use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bsc_dir = manifest_dir.join("bsc-m03");

    let mut c_sources = cc::Build::new();
    c_sources.include(&bsc_dir);
    c_sources.file(bsc_dir.join("hutucker/hu-tucker.c"));
    c_sources.file(bsc_dir.join("libbscsais/libbscsais.c"));
    c_sources.file(bsc_dir.join("libbscsais/libbscsais16.c"));
    c_sources.compile("bsc_m03_support");

    let mut cpp_sources = cc::Build::new();
    cpp_sources.cpp(true);
    cpp_sources.include(&bsc_dir);
    cpp_sources.file(manifest_dir.join("src/bsc_wrapper.cpp"));
    cpp_sources.flag("-w");

    if cfg!(target_env = "msvc") {
        cpp_sources.flag_if_supported("/std:c++17");
    } else {
        cpp_sources.flag_if_supported("-std=c++17");
    }

    cpp_sources.compile("bsc_m03_wrapper");

    for path in [
        "src/bsc_wrapper.cpp",
        "bsc-m03/bsc-m03.cpp",
        "bsc-m03/hutucker/hu-tucker.c",
        "bsc-m03/libbscsais/libbscsais.c",
        "bsc-m03/libbscsais/libbscsais16.c",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    println!("cargo:rustc-link-lib=static=bsc_m03_wrapper");
    println!("cargo:rustc-link-lib=static=bsc_m03_support");

    if cfg!(all(target_os = "macos", not(target_env = "msvc"))) {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if cfg!(all(any(target_os = "linux", target_os = "android"), not(target_env = "msvc"))) {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
