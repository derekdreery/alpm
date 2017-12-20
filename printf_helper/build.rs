extern crate gcc;

fn main() {
    gcc::compile_library("libprintf_test_helper.a", &["src/helper.c"]);
}
