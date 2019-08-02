use std::{env, path::PathBuf, thread};

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
    let t = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        let ly_code = PathBuf::from(r"C:\Amazon\Lumberyard\1.19.0.0\dev\Code\");
        let cryengine_crycommon = r"CryEngine\CryCommon\";
        // Path to WinKit comes from
        //c:\Amazon\Lumberyard\1.19.0.0\dev\Tools\build\waf-1.7.13\lmbrwaflib\mscv_helper.py
        let visual_studio = String::from(r"C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\include");
        let winkit_includes = PathBuf::from(r"C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\");
        let mut builder = bindgen::Builder::default()
            .clang_arg(String::from("-I") + &visual_studio)
            .with_codegen_config(bindgen::CodegenConfig::empty())
            .rustfmt_bindings(false)
            ;
        if cfg!(feature = "lmbr_fw_azcore") {
            builder = builder.header("wrapper_fw_azcore.hpp")
                .clang_arg(String::from("-I") + ly_code.join(r"Framework\AzCore").to_str().unwrap())
                //.clang_arg(String::from("-I") + ly_code.join(cryengine_crycommon).to_str().unwrap())
                ;
        }
        // if cfg!(feature = "lmbr_editor") {
        //     //builder = builder.whitelist_type()
        //     builder = builder.header("wrapper2.hpp")
        //         .clang_arg("-std=c++14")
        //         //.clang_arg("-fms-compatibility")
        //         //.clang_arg("-Wmicrosoft")
        //         //.clang_arg("-Wmicrosoft-enum-value")
        //         //.clang_arg("-Wno-c++11-narrowing")
        //         //.clang_arg("-Winvalid-constexpr")
                
        //         .clang_arg(String::from("-I") + winkit_includes.join(r"shared").to_str().unwrap())
        //         .clang_arg(String::from("-I") + winkit_includes.join(r"ucrt").to_str().unwrap())
        //         .clang_arg(String::from("-I") + winkit_includes.join(r"um").to_str().unwrap())
        //         .clang_arg(String::from("-I") + ly_code.join(cryengine_crycommon).to_str().unwrap())
        //         .clang_arg(String::from("-I") + ly_code.join(r"Sandbox\Editor\").to_str().unwrap())
        //         .clang_arg(String::from("-I") + ly_code.join(r"Sandbox\Plugins\EditorCommon\").to_str().unwrap())
        //         ;
        // }
        builder = builder
            //.enable_cxx_namespaces()
            //.generate_inline_functions(false)
            .generate_comments(false)
            .layout_tests(false)
            .blacklist_type(".*")
            .blacklist_item(".*")
            .blacklist_function(".*")
            .whitelist_recursively(false)
            .no_convert_floats()
            .ignore_functions()
            .ignore_methods()
            //.whitelist_type("AZ::Debug::.*")
            // .whitelist_function("nng_.*")
            // .whitelist_var("NNG_.*")
            // Generate `pub const NNG_UNIT_EVENTS` instead of `nng_unit_enum_NNG_UNIT_EVENTS`
            //.prepend_enum_name(false)
            ;
        let flags = builder.command_line_flags();
        for flag in flags {
            println!("{}", flag);
        }
        builder.dump_preprocessed_input().unwrap();
        let bindings = builder.generate().expect("Unable to generate bindings");
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings");
    }).unwrap();
    t.join().unwrap();
    
}