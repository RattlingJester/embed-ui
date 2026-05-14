use std::env;
use std::path::PathBuf;

fn main() {
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let native_lib_path = project_root.join("../vendor");

    println!(
        "cargo:rustc-link-search=native={}",
        native_lib_path.display()
    );

    println!("cargo:rustc-link-lib=dylib=sdl2");

    println!("cargo:rerun-if-changed=vendor/");
}
