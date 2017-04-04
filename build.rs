extern crate gcc;

fn main() {
    gcc::compile_library("libearwax.a", &["src/libearwax.c"]);
}
