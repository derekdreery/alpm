extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/printf_wrapper.c")
        .compile("printf_wrapper");
}
