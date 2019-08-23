use lmbr_build::*;
use std::{path::PathBuf};

fn main() {
    // Check output of `cargo build --verbose`, should see something like:
    // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
    // That contains output from cmake
    let config = build_config();
    let config_3rdparty = if config == BuildConfig::Debug { "debug" } else { "release" };
    let ly_root = PathBuf::from(lmbr_root().unwrap());
    let bintemp = ly_root.join("dev/BinTemp").join(bintemp_dir());
    let bintemp_fw = bintemp.join("Code/Framework");
    println!(
        "cargo:rustc-link-search=native={}",
        bintemp_fw.join("AzCore/AzCore").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        bintemp_fw.join("AzFramework/AzFramework").display()
    );
    println!("cargo:rustc-link-search=native={}",
        ly_root.join(r"3rdParty\Lua\5.1.1.8-az\build\win_x64\vc140").join(config_3rdparty).display()
    );
    println!("cargo:rustc-link-search=native={}",
        ly_root.join(r"3rdParty\zlib\1.2.8-pkg.2\build\win_x64\vc140").join(config_3rdparty).display()
    );
    println!("cargo:rustc-link-search=native={}",
        bintemp.join("Code/CryEngine/CryCommon").display()
    );
    
    println!("cargo:rustc-link-lib=static=AzCore");
    println!("cargo:rustc-link-lib=static=AzFramework");
    println!("cargo:rustc-link-lib=static=lua");
    println!("cargo:rustc-link-lib=static=zlib");
    println!("cargo:rustc-link-lib=static=CryCommonLib");
    println!("cargo:rustc-link-lib=static=user32");
}