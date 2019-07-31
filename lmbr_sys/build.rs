use std::{env, path::PathBuf};

fn main() {
    // Check output of `cargo build --verbose`, should see something like:
    // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
    // That contains output from cmake
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     dst.join("lib").display()
    // );
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     dst.join("lib64").display()
    // );

    // Tell rustc to use nng static library
    //println!("cargo:rustc-link-lib=static=nng");

    // https://rust-lang-nursery.github.io/rust-bindgen
    // https://docs.rs/bindgen
    let mut builder = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_arg("-IC:/Amazon/Lumberyard/1.19.0.0/dev/Code/Framework/AzCore")
        ;
    builder = builder
        .enable_cxx_namespaces()
        .generate_inline_functions(true)
        .whitelist_type("AZ::Debug::.*")
        // .whitelist_function("nng_.*")
        // .whitelist_var("NNG_.*")
        // Generate `pub const NNG_UNIT_EVENTS` instead of `nng_unit_enum_NNG_UNIT_EVENTS`
        .prepend_enum_name(false)
        ;
    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}