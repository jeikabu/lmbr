use lmbr_build::*;
use std::{path::PathBuf};

fn main() {
    let ly_root = lmbr_root_pathbuf().unwrap();
    let bintemp = bintemp_dir();
    
    link::az_framework();
    link::asset_builder();
    // println!("cargo:rustc-link-search=native={}",
    //     bintemp.join("Code/CryEngine/CryCommon").display()
    // );
    println!("cargo:rustc-link-search=native={}",
        bintemp.join("Code/Sandbox/EditorRustPlugin").display()
    );
    println!("cargo:rustc-link-search=native={}",
        ly_root.join("dev").join(bin_dir()).display()
    );
    println!("cargo:rustc-link-search=native={}",
        bintemp.join("Code/Framework/RustAz/RustAz").display()
    );
    println!("cargo:rustc-link-lib=static=RustAz");
    
    println!("cargo:rustc-link-lib=static=EditorRustPlugin");
    println!("cargo:rustc-link-lib=static=user32");
}