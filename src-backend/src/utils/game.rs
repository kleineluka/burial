// imports
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::io::{self, BufRead, BufReader};
use winreg::enums::*;
use winreg::RegKey;
use regex::Regex;

// make sure that a game directory is actually a tcoaal game directory
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

// get the main.js of the game
pub fn get_mainjs(dir_path: &str) -> PathBuf {
    let path = Path::new(dir_path);
    let mainjs_path: PathBuf = path.join("www").join("js").join("main.js");
    mainjs_path
}

// get the game version from the files
pub fn game_version(in_path: String) -> String {
    // navigate from in_path to www/js/main.js
     let main_js_path = format!("{}//www//js//main.js", in_path);
    // open main.js
    let file = File::open(main_js_path).unwrap();
    let reader = BufReader::new(file);
    // find const GAME_VERSION = "ANYTHING GOES HERE";
     for line in reader.lines() {
        let line = line.unwrap();
        // Look for the line that contains the GAME_VERSION constant
        if line.contains("const GAME_VERSION") {
            // Extract the value between quotes
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    // Return the extracted version
                    let game_version = &line[start + 1..start + 1 + end];
                    return game_version.to_string();
                }
            }
        }
    }
    // extract what is in the quotes
    return "Unknown".to_string();
}

pub fn find_installation() -> Result<Option<PathBuf>, Box<dyn Error>> {
    // find the steam installation
    let hk_local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam_key = hk_local_machine
        .open_subkey("SOFTWARE\\Wow6432Node\\Valve\\Steam")
        .map_err(|_| "Steam not found in registry")?;
    let steam_path: String = steam_key
        .get_value("InstallPath")
        .map_err(|_| "InstallPath not found in registry")?;
    // find and open the steam library
    let library_file_path = Path::new(&steam_path).join("steamapps").join("libraryfolders.vdf");
    if !library_file_path.exists() {
        return Err("libraryfolders.vdf not found.".into());
    }
    // parse the library file
    let content = fs::read_to_string(&library_file_path)?;
    let re = Regex::new(r#""path"\s+"([^"]+)""#)?;
    let mut paths_list = vec![steam_path.clone()];
    for cap in re.captures_iter(&content) {
        if let Some(path) = cap.get(1) {
            paths_list.push(path.as_str().to_string());
        }
    }
    // find the game path
    for library_path in paths_list {
        let game_path = PathBuf::from(&library_path).join("steamapps").join("common").join("The Coffin of Andy and Leyley");
        if game_path.exists() {
            return Ok(Some(game_path));
        }
    }
    // womp, womp..
    Ok(None)
}