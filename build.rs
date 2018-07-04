extern crate cpp_build;

fn main() {
    cpp_build::Config::new()
        .compiler("/usr/bin/clang++-3.8")
        .cpp_set_stdlib(Some("c++"))
        .flag("--std=c++11")
        .include("include")
        .object("half_mod/halfmod.o")
        .object("half_mod/str_tok.o")
        .build("src/lib.rs");
}