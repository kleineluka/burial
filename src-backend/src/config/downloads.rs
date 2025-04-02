// imports
use std::path::PathBuf;
use crate::utils::helpers::files;
use crate::config::cache;
use dirs::config_dir;

// return downloads folder path
pub fn downloads_folder() -> PathBuf {
    let mut burial_dir: PathBuf = config_dir().unwrap_or_else(|| PathBuf::from("."));
    burial_dir.push("burial");
    burial_dir.push("downloads");
    burial_dir
}

// make sure downloads folder exists
pub fn verify_downloads() -> std::io::Result<()> {
    cache::verify_cache()?;
    let burial_dir: PathBuf = downloads_folder();
    files::verify_folder(&burial_dir)?;
    Ok(())
}

// clear the downloads folder
pub fn clear_downloads() -> std::io::Result<()> {
    let burial_dir = downloads_folder().to_string_lossy().to_string();
    files::delete_folder(&burial_dir);
    Ok(())
}