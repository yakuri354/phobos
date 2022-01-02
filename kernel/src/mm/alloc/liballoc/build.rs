fn main() {
    let mut bld = cc::Build::new();
    if cfg!(debug_assertions) {
        bld.define("INFO", None);
    }
    bld.file("c_src/liballoc.c").compile("liballoc");
}
