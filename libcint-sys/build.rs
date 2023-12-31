use std::env;
use std::path::PathBuf;

fn main() {
    const BUILD_DIR: &str = "/home/brent/clone/libcint/build";

    println!("cargo:rustc-link-search={BUILD_DIR}");
    println!("cargo:rustc-link-lib=cint");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bind = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{BUILD_DIR}/include"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bind.write_to_file(out_path.join("bindings.rs")).unwrap();
}
