use std::{env, path::PathBuf};

use gluegen::generate_glue;

fn main() {
    // rerun build script if definition file is changed:
    let lib_name = "SharedBase";
    println!("cargo::rerun-if-changed=lib/{lib_name}.def");
    println!("cargo::rustc-link-lib={lib_name}");
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut lib_dir = manifest_dir.clone();
    lib_dir.push("lib");
    println!("cargo:rustc-link-search=all={}", lib_dir.display());

    let def_file = std::fs::File::open(format!("lib/{lib_name}.def")).unwrap();
    let manual_types = &["bCString"];
    let additional_opaque_types = &[];
    generate_glue(def_file, lib_name, manual_types, additional_opaque_types);
}
