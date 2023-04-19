extern crate bindgen;

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=nvdialog.h");

    // The bindgen::Builder is the main entry point to bindgen, and lets
    // you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("nvdialog/include/nvdialog.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = Config::new("./nvdialog")
        .build_target("nvdialog")
        .define("NVD_BUILD_STATIC", "OFF")
        .define("NVDIALOG_MAXBUF", "256")
        .define("NVD_USE_GTK4", "OFF")
        .define("CMAKE_BUILD_TYPE", if cfg!(debug_assertions) { "Debug"} else { "Release" })
        .build();

    #[cfg(target_os = "linux")]
    {
        match pkg_config::Config::new().probe("gtk+-3.0") {
            Ok(library) => {
                println!("Found Gtk3, version {}.", library.version);
                println!("Link paths: {:?}", library.link_paths);
                println!("Link libraries: {:?}", library.libs);
    
                for path in library.link_paths {
                    println!("cargo:rustc-link-search=native={}", path.display());
                }
                for lib in library.libs {
                    println!("cargo:rustc-link-lib={}", lib);
                }
            },
            Err(_) => {
                panic!("You need Gtk3 on Linux to use NvDialog.")
            },
        }
    }

    println!("cargo:rustc-link-search=native={}/build/", dst.display());
    println!("cargo:rustc-link-lib=nvdialog");
}