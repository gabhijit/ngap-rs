use bindgen;
use pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:run-if-changed=build.rs");

    let ngap_codec_lib = pkg_config::Config::new()
        .statik(true)
        .probe("ngapcodec-15.3.0")
        .unwrap();

    println!("{:?}", ngap_codec_lib);

    let include_path = &ngap_codec_lib.include_paths[0];

    let mut clang_arg = String::from("-I");
    clang_arg.push_str(
        include_path
            .to_str()
            .expect("Unable to get include path for clang"),
    );

    println!("clang_arg: {:?}", clang_arg);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(clang_arg)
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
