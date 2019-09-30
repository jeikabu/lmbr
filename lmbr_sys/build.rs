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
    let ly_root = lmbr_build::lmbr_root();
    let ly_root = PathBuf::from(ly_root.expect("Set LMBR_ROOT environment variable to Lumberyard Code/ path"));
    let ly_code = ly_root.join("dev/Code/");
    let az_core = ly_code.join("Framework/AzCore");
    let types = ["pub type f32 = crate::F32;", "pub type f64 = crate::F64;", "pub type u16 = crate::U16;", "pub type u32 = crate::U32;", "pub type u64 = crate::U64;"];
    let mut builder = bindgen::Builder::default()
        //.with_codegen_config(bindgen::CodegenConfig::empty())
        .clang_arg("-std=c++14")
        .enable_cxx_namespaces()
        .generate_inline_functions(true)
        .generate_comments(false)
        .layout_tests(false)
        //.blacklist_type(".*")
        // .blacklist_item(".*")
        // .blacklist_function(".*")
        // .ignore_functions()
        // .ignore_methods()
        // Generate `pub const NNG_UNIT_EVENTS` instead of `nng_unit_enum_NNG_UNIT_EVENTS`
        //.prepend_enum_name(false)
        //.clang_arg(String::from("-I") + ly_code.join("Framework/AzCore").to_str().unwrap())
        .clang_arg(String::from("-I") + az_core.to_str().unwrap())
        .clang_arg(String::from("-I") + az_core.join("Platform/Windows").to_str().unwrap())
        .clang_arg(String::from("-I") + ly_root.join("3rdParty/boost/1.61.0-az.2/").to_str().unwrap())
        .clang_arg(String::from("-I") + ly_code.join("CryEngine/CryCommon/").to_str().unwrap())
        .raw_line("type F32 = f32;")
        .raw_line("type F64 = f64;")
        .raw_line("type U16 = u16;")
        .raw_line("type U32 = u32;")
        .raw_line("type U64 = u64;")
        .blacklist_type(r"AZ::u\d{2,3}") // e.g. u32 u128
        .blacklist_type(r"(f32|f64)")
        .module_raw_lines("root::AZ", types.iter().map(|s| *s))
        .module_raw_lines("root", types.iter().map(|s| *s))
        ;
    #[cfg(target_os = "macos")]
    {
        builder = builder
            .clang_arg("-DTARGET_OS_MAC")
            .clang_arg("-I/Library//Developer/CommandLineTools/usr/lib/clang/10.0.1/include/")
            .clang_arg("-I/Library//Developer/CommandLineTools/usr/include/c++/v1/")
            .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/")
            ;
    }
    #[cfg(target_os = "windows")]
    {
        // Path to WinKit comes from
        //c:\Amazon\Lumberyard\1.19.0.0\dev\Tools\build\waf-1.7.13\lmbrwaflib\mscv_helper.py
        let visual_studio = String::from(r"C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\include");
        let winkit_includes = PathBuf::from(r"C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\");
        builder = builder
            .clang_arg(String::from("-I") + &visual_studio)
            .clang_arg(String::from("-I") + winkit_includes.join(r"shared").to_str().unwrap())
            .clang_arg(String::from("-I") + winkit_includes.join(r"ucrt").to_str().unwrap())
            .clang_arg(String::from("-I") + winkit_includes.join(r"um").to_str().unwrap())
            ;
    }
    if cfg!(feature = "lmbr_fw_azcore") {
        builder = builder.header("wrapper_fw_azcore.hpp")
            .whitelist_type("AZ::Debug::Trace")
            //.whitelist_type("AZ::EBusRouterPolicy_Container")
            // .blacklist_item("AZStd::.*")
            // .blacklist_item("AZStd::list_(const_)?iterator.*")
            // .blacklist_item("AZStd::list_(base_)?node(_ptr)?_type")
            // .blacklist_item("AZStd::hash_table_.*_type")
            // .blacklist_item("AZStd::unordered_map_.*_type")
            ;
    }
    // if cfg!(feature = "lmbr_editor") {
    //     //builder = builder.whitelist_type()
    //     builder = builder.header("wrapper_editor.hpp")
    //         //.clang_arg("-fms-compatibility")
    //         //.clang_arg("-Wmicrosoft")
    //         //.clang_arg("-Wmicrosoft-enum-value")
    //         //.clang_arg("-Wno-c++11-narrowing")
    //         //.clang_arg("-Winvalid-constexpr")
    //         .whitelist_type("IPlugin")
    //         .whitelist_type("PLUGIN_INIT_PARAM")
    //         .whitelist_type("SPluginSettings")
    //         .clang_arg(String::from("-I") + ly_code.join(r"Sandbox\Editor\").to_str().unwrap())
    //         .clang_arg(String::from("-I") + ly_code.join(r"Sandbox\Plugins\EditorCommon\").to_str().unwrap())
    //         ;
    // }
    
    // let flags = builder.command_line_flags();
    // for flag in flags {
    //     println!("{}", flag);
    // }
    //builder.dump_preprocessed_input().unwrap();
    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}