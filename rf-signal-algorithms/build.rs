use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=signals");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("c/itwom3.0.hh")
        .header("c/cost.hh")
        .header("c/ecc33.hh")
        .header("c/egli.hh")
        .header("c/hata.hh")
        .header("c/pel.hh")
        .header("c/soil.hh")
        .header("c/sui.hh")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build the C++ library
    cc::Build::new()
        .file("c/itwom3.0.cc")
        .file("c/cost.cc")
        .file("c/ecc33.cc")
        .file("c/egli.cc")
        .file("c/hata.cc")
        .file("c/pel.cc")
        .file("c/soil.cc")
        .file("c/sui.cc")
        .compile("libsignals.a");
}
