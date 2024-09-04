// imports
use std::path::Path;
use std::io;

pub fn verify_game(dir_path: &str) -> io::Result<bool> {
    // sanity check
    let path = Path::new(dir_path);
    if !path.is_dir() {
        return Ok(false);
    }
    // random few files that should exist cross-platform..
    let required_files = ["Game.exe", "credits.html"];
    let required_folders = ["www"];
    // check for required files
    for file in &required_files {
        let file_path = path.join(file);
        if !file_path.is_file() {
            return Ok(false);
        }
    }
    // check for required folders
    for folder in &required_folders {
        let folder_path = path.join(folder);
        if !folder_path.is_dir() {
            return Ok(false);
        }
    }
    // all good!
    Ok(true)
}


