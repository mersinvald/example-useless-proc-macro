#![feature(proc_macro)]
#[macro_use]
extern crate multivec_macro;

#[derive(VGenerate)]
struct VGenerator;

fn main() {
    let vector = VGenerator::generate();
    let () = vector;
}

