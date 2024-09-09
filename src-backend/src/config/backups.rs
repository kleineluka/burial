// imports
use std::path::PathBuf;
use crate::utils::files;
use crate::config::cache;
use dirs::config_dir;

// return backup folder path
pub fn backup_folder() -> PathBuf {
    let mut burial_dir: PathBuf = config_dir().unwrap_or_else(|| PathBuf::from("."));
    burial_dir.push("burial");
    burial_dir.push("backups");
    burial_dir
}

// make sure backups folder exists
pub fn verify_backups() -> std::io::Result<()> {
    cache::verify_cache()?;
    let burial_dir: PathBuf = backup_folder();
    files::verify_folder(&burial_dir)?;
    Ok(())
}

// get a name for the new backup file (DD-MM-YYYY-HH-MM-SS)
pub fn new_backup_name() -> String {
    let now = chrono::Local::now();
    now.format("%d-%m-%Y-%H-%M-%S").to_string()
}