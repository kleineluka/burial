// imports
use std::path::PathBuf;
use crate::utils::files;
use dirs::config_dir;

// return disabled folder path
pub fn disabled_folder() -> PathBuf {
    let mut burial_dir: PathBuf = config_dir().unwrap_or_else(|| PathBuf::from("."));
    burial_dir.push("burial");
    burial_dir.push("disabled");
    burial_dir
}

// make sure disabled folder exists
pub fn verify_disabled() -> std::io::Result<()> {
    let burial_dir: PathBuf = disabled_folder();
    files::verify_folder(&burial_dir)?;
    Ok(())
}

// clear the disabled folder
pub fn clear_disabled() -> std::io::Result<()> {
    let burial_dir = disabled_folder().to_string_lossy().to_string();
    files::delete_folder(&burial_dir);
    Ok(())
}