extern crate cc;

fn main() {
    cc::Build::new()
        .flag("-I")
        .flag("~/Downloads/lua-5.3.5/src/")
        .flag("-llua")
        .flag("-lscript_api")
        .flag("-L./script_api/target/debug")
        .file("c/driver.c")
        .compile("driver");
}
