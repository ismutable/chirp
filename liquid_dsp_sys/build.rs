use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=liquid");
    println!("cargo:rustc-link-search=native=/usr/lib/x84_64-linux-gnu");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindgings.")
}
