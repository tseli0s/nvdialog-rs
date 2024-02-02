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
        .define("NVD_BUILD_STATIC", "OFF")
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
            }
            Err(_) => {
                panic!("You need Gtk3 on Linux to use NvDialog.")
            }
        }
    }

    println!("cargo:rustc-link-search=native={}/build/", dst.display());
    println!("cargo:rustc-link-lib=nvdialog");
}
