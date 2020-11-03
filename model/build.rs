fn main() {
    let model = "../airbus-fly-by-wire-wasm/src/model";
    let msfs_sdk = std::env::var("MSFS_SDK").unwrap_or_else(calculate_msfs_sdk_path);
    println!("Found MSFS SDK: {:?}", msfs_sdk);

    let wasm = std::env::var("TARGET") == Ok("wasm32-wasi".to_string());

    // Build airbus-fbw model and link it
    {
        if wasm {
            std::env::set_var("AR", "llvm-ar");
        }

        let mut build = cc::Build::new();
        build
            .include(model)
            .files(
                std::fs::read_dir(model)
                    .unwrap()
                    .map(|d| d.unwrap().path().as_path().to_str().unwrap().to_owned())
                    .filter(|f| f.ends_with(".cpp")),
            )
            .cpp(true);

        if wasm {
            build
                .compiler("clang")
                .flag(&format!("--sysroot={}/WASM/wasi-sysroot", msfs_sdk));
        }

        build.compile("airbus_fbw");
    }

    // Build header definitions for rust
    {
        println!("cargo:rerun-if-changed=src/wrapper.hpp");
        let mut build = bindgen::Builder::default()
            .clang_arg(format!("-I{}", model))
            .clang_arg("-xc++")
            .header("./src/wrapper.hpp")
            .whitelist_type("fbwModelClass")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .impl_debug(true);

        if wasm {
            build = build
                .clang_arg(format!("--sysroot={}/WASM/wasi-sysroot", msfs_sdk))
                .clang_arg("-fvisibility=default");
        }

        build
            .generate()
            .unwrap()
            .write_to_file(
                std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
            )
            .unwrap();
    }
}

fn calculate_msfs_sdk_path(_: std::env::VarError) -> String {
    for p in ["/mnt/c/MSFS SDK", r"C:\MSFS SDK"].iter() {
        if std::path::Path::new(p).exists() {
            return p.to_string();
        }
    }
    panic!("Could not locate MSFS SDK. Make sure you have it installed or try setting the MSFS_SDK env var.");
}
