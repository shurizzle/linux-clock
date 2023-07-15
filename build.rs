use std::path::PathBuf;

use bindgen::callbacks::{IntKind, ParseCallbacks};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "netbsd" {
        return;
    }

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings");

    if !out_path.exists() {
        std::fs::create_dir(&out_path).unwrap();
    }

    #[derive(Debug)]
    struct ClockConstants;

    impl ParseCallbacks for ClockConstants {
        fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
            if name.starts_with("CLOCK_") {
                Some(IntKind::I32)
            } else {
                None
            }
        }
    }

    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <sys/time.h>")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ClockConstants))
        .ctypes_prefix("::libc")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("time.rs"))
        .expect("Couldn't write bindings!");
}
