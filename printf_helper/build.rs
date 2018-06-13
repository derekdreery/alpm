extern crate cc;

// use std::env;

fn main() {
    cc::Build::new()
        .file("src/helper.c")
        .compile("printf-helper");
    
    // let out_dir = env::var("OUT_DIR").unwrap();
    
    // println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-lib=static=printf_helper");
}
