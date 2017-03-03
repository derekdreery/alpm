

extern crate gcc;

fn main() {
    gcc::compile_library("libprintf_wrapper.a", &["src/printf_wrapper.c"]);
}
