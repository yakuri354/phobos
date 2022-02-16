use std::{
    env,
    fs::{rename, File},
};

const FILE: &str = "src/arch/amd64/asm.S"; // TODO This is awful

fn main() {
    println!("cargo:rerun-if-changed={}", FILE);
    nasm_rs::compile_library("asm.o", &[FILE]).expect("Error compiling NASM module");

    println!("cargo:rustc-link-lib=static=asm.o")
}
