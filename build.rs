fn main() {
    const BUILD_DIR: &str = "/home/brent/clone/libcint/build";
    println!("cargo:rustc-env=LD_LIBRARY_PATH={BUILD_DIR}");
}
