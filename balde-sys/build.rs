use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;

fn main() {
    let results = pkg_config::probe_library("balde").unwrap();
    let includes: Vec<String> = results
        .include_paths
        .iter()
        .map(|include| format!("-I{}", include.to_str().unwrap()))
        .collect();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(includes)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let output_bindings: String = bindings.to_string();
    // Ignore glib's types in order to try and hack the types to use glib-sys
    let start_balde = output_bindings.find("pub const balde").unwrap();
    let mut output_file = File::create(out_path.join("bindings.rs")).unwrap();
    output_file
        .write(&output_bindings[start_balde..].as_bytes())
        .unwrap();
}
