// imports
use std::path::PathBuf;
use dirs::config_dir;
use crate::files;

// make sure cache folder exists
pub fn verify_cache() -> std::io::Result<()> {
    // %appdata% on windows, $XDG_CONFIG_HOME or ~/.config on linux, ~/Library/Application Support on mac (i think)
    let mut burial_dir: PathBuf = config_dir().unwrap_or_else(|| PathBuf::from("."));
    burial_dir.push("burial");
    files::verify_folder(&burial_dir)?;
    Ok(())
}

// get the cache folder
pub fn cache_folder() -> PathBuf {
    let _ = verify_cache();
    let mut burial_dir: PathBuf = config_dir().unwrap_or_else(|| PathBuf::from("."));
    burial_dir.push("burial");
    burial_dir
}

// create a temporary folder in the cache
pub fn create_temp() -> PathBuf {
    let mut burial_dir: PathBuf = cache_folder();
    burial_dir.push("temp");
    files::verify_folder(&burial_dir).unwrap();
    burial_dir
}

// just get the temporary folder
pub fn temp_folder() -> PathBuf {
    let mut burial_dir: PathBuf = cache_folder();
    burial_dir.push("temp");
    burial_dir
}

// delete the temporary folder
pub fn clear_temp() {
    let burial_dir: PathBuf = create_temp();
    files::delete_folder(&burial_dir.to_str().unwrap());
}