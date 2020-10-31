fn main() {
    // Build airbus-fbw model and link it
    cc::Build::new()
        .cpp(true)
        .include("./airbus-fbw-bench/src-model/src")
        .file("./airbus-fbw-bench/src-model/src/fbw.cpp")
        .file("./airbus-fbw-bench/src-model/src/fbw_data.cpp")
        .file("./airbus-fbw-bench/src-model/src/rt_nonfinite.cpp")
        .file("./airbus-fbw-bench/src-model/src/rtGetInf.cpp")
        .file("./airbus-fbw-bench/src-model/src/rtGetNaN.cpp")
        .compile("airbus_fbw");

    // Build header definitions for rust
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    bindgen::Builder::default()
        .clang_arg("-I./airbus-fbw-bench/src-model/src")
        .header("./src/wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .impl_debug(true)
        .generate()
        .unwrap()
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .unwrap();
}
