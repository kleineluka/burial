// imports 
use std::env;
use std::path::{Path, PathBuf};

// return backup folder path
pub fn save_folder() -> PathBuf {
    let appdata = env::var("APPDATA").expect("Failed to get %APPDATA% directory");
    Path::new(&appdata).join("CoffinAndyLeyley")
}