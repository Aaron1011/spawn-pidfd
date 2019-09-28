use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .whitelist_var("ONE_FD_BUF_SIZE")
        .generate()
        .expect("Failed to generate bindings!");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}
