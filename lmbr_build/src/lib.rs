use std::{env, fmt};
use std::{path::PathBuf};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuildConfig {
    Debug,
    Release,
    Profile,
    Performance,
}

impl fmt::Display for BuildConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn build_config() -> BuildConfig {
    let is_release = env::var("PROFILE").map(|profile| profile == "release").unwrap_or(false);
    if is_release {
        BuildConfig::Profile
    } else {
        BuildConfig::Profile
    }
}

pub fn config_3rdparty() -> &'static str {
    let config = build_config();
    let config_3rdparty = if config == BuildConfig::Debug { "debug" } else { "release" };
    config_3rdparty
}

pub fn bintemp_dir() -> PathBuf {
    let config = build_config();
    let bintemp_subdir = format!("win_x64_vs2017_{}", config);
    let ly_root = PathBuf::from(lmbr_root().unwrap());
    let bintemp = ly_root.join("dev/BinTemp").join(bintemp_subdir);
    bintemp
}

pub fn bin_dir() -> &'static str {
    match build_config() {
        config if config == BuildConfig::Debug => "Bin64vc141.Debug",
        _ => "Bin64vc141",
    }
}

pub fn lmbr_root() -> Result<String, std::env::VarError> {
    std::env::var("LMBR_ROOT").or_else(|_| {
            if cfg!(target_os = "windows") {
                Ok(String::from(r"C:\Amazon\Lumberyard\1.21.0.0"))
            } else {
                Err(std::env::VarError::NotPresent)
            }
    })
}

pub fn lmbr_root_pathbuf() -> Option<PathBuf> {
    lmbr_root().ok().map(PathBuf::from)
}

pub mod link {
    use super::*;
    pub fn az_framework() {
        let bintemp = bintemp_dir();
        let bintemp_fw = bintemp.join("Code/Framework");
        let ly_root = lmbr_root_pathbuf().unwrap();
        let config_3rdparty = config_3rdparty();
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
        println!("cargo:rustc-link-lib=static=AzCore");
        println!("cargo:rustc-link-lib=static=AzFramework");
        println!("cargo:rustc-link-lib=static=lua");
        println!("cargo:rustc-link-lib=static=zlib");
    }
    pub fn asset_builder() {
        let bintemp = bintemp_dir();
        println!(
            "cargo:rustc-link-search=native={}",
            bintemp.join("Code/Tools/AssetProcessor/AssetBuilderSDK").display()
        );
        println!("cargo:rustc-link-lib=static=AssetBuilderSDK");
    }
}