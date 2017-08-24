extern crate gcc;

fn main() {
    //gcc::compile_library("libearwax.a", &["src/libearwax.c"]);
    gcc::Build::new()
        .file("src/libearwax.c")
        .compile("libearwax.a");
}
