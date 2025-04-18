extern crate bindgen;

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=nvdialog.h");

    let bindings = bindgen::Builder::default()
        .header("nvdialog/include/nvdialog.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = Config::new("./nvdialog")
        .build_target("nvdialog")
        .define("NVD_BUILD_STATIC", "ON")
        .define("NVDIALOG_MAXBUF", "256")
        .define("NVD_USE_GTK4", "OFF")
        .define(
            "CMAKE_BUILD_TYPE",
            if cfg!(debug_assertions) {
                "Debug"
            } else {
                "Release"
            },
        )
        .build();

    println!("cargo:rustc-link-search=native={}/build/", dst.display());
    println!("cargo:rustc-link-lib=static=nvdialog");

    #[cfg(all(target_os = "linux", feature = "use-pkg-config"))]
    for l in pkg_config::Config::new()
        .atleast_version("3.0")
        .probe("gtk+-3.0")
        .expect("Could not find GTK+ 3.0 via pkg-config")
        .libs
    {
        println!("cargo:rustc-link-lib={}", l);
    }

    #[cfg(target_os = "windows")]
    {
        let libs = [ "shell32", "user32", "gdi32", "ole32" ];
        for l in libs {
            println!("cargo:rustc-link-lib={}", l);
        }
    }
}
