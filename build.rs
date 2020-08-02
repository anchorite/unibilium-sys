use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to link the library
    println!("cargo:rustc-link-lib=unibilium");

    // Tell cargo to invalidate crate if changed
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
