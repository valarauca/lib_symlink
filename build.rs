extern crate gcc;

fn main() {
    gcc::Config::new().file("src/dlfcn_bindings.c").compile("dlfcn_bindings.a")
}
