extern crate bindgen;
extern crate fs_extra;
use fs_extra::dir::*;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::*;

const LIB_LIST: &'static [&'static str] = &[
    "FRC_NetworkCommunication",
    "NiFpga",
    "NiFpgaLv",
    "niriodevenum",
    "niriosession",
    "NiRioSrv",
    "RoboRIO_FRC_ChipObject",
    "visa",
    "wpiHal",
    "wpiutil",
];

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

// fn place_libs() {
//     let options = CopyOptions {
//         overwrite: true,
//         skip_exist: false,
//         buffer_size: 6400,
//         copy_inside: true,
//         depth: 100,
//     };
//     let mut out_path = PathBuf::new();
//     out_path.push(env::var("CARGO_MANIFEST_DIR").expect("Couldn't read manifest dir env var."));
//     out_path.push(".frc");
//     let mut input = env::current_dir().expect("Couldn't find current directory");
//     input.push("HAL/lib");
//     copy(input, out_path, &options).expect("Couldn't copy libs.");
// }

fn announce_lib() {
    let mut out_path = PathBuf::new();
    out_path.push(env::var("CARGO_MANIFEST_DIR").expect("Couldn't read manifest dir env var."));
    out_path.push(".frc");
    fs::write("/tmp/frc.txt", &*out_path.as_path().to_string_lossy())
        .expect("Unable to write file");
}

fn main() {
    // place_libs();
    announce_lib();
    for lib in LIB_LIST.iter() {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    let path = env::current_dir().unwrap();
    // println!("{:?} {:?}", path, env::current_dir().unwrap());
    println!("cargo:rustc-link-search=native={}/HAL/lib", path.display());

    const SYMBOL_REGEX: &'static str = "HAL_[A-Za-z0-9]+";
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        .rustfmt_bindings(false)
        .header("HAL/include/HAL/HAL.h")
        .whitelist_type(SYMBOL_REGEX)
        .whitelist_function(SYMBOL_REGEX)
        .whitelist_var(SYMBOL_REGEX)
        // usage reporting enums
        .whitelist_type(".*tInstances")
        .whitelist_type(".*tResourceType")
        .clang_arg("-I./HAL/include")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14");
    println!("builder_args: {:?}", bindings.command_line_flags());
    let out = bindings.generate().expect("Unable to generate bindings");

    out.write_to_file(output().join("hal_bindings.rs"))
        .expect("Couldn't write bindings!");
}
