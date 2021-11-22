use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/creature_fighter.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/creature_fighter.fl", out_path.join("creature_fighter.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
}
