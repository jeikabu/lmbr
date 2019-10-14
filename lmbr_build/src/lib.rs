use std::{env, fmt};

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

pub fn bintemp_dir() -> String {
    let config = build_config();
    format!("win_x64_vs2017_{}", config)
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