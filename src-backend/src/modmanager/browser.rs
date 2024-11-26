// imports
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use tauri::Window;
use tauri::command;
use sha2::{Sha256, Digest};
use crate::config::cache;
use crate::utils::connection;
use crate::utils::game;
use crate::modmanager::modloader;

// verify that the user's current installation is ready to install mods
#[command]
pub fn mod_ready(window: Window, in_path: String) {
    // first, see if the game path is valid
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("mod-ready", "error_game_path").unwrap();
        return;
    }
    // and now see if the modloader is present
    let modloader_presence = modloader::modloader_prescence(in_path);
    if !modloader_presence {
        window.emit("mod-ready", "error_modloader").unwrap();
        return;
    }
    // all good!
    window.emit("mod-ready", "success").unwrap();
}

// install a mod
#[command]
pub async fn install_mod(window: Window, in_path: String, mod_path: String, mod_hash: String) {
    // assume everything is valid at this stage!
    let cache = cache::cache_folder();
    if let Err(e) = connection::download_file(&mod_path, &cache.to_string_lossy()).await {
        window.emit("mod-download", "error_connection").unwrap();
        return;
    }
    // get the name of the zip file and then open it
    let mod_name = mod_path.split("/").last().unwrap();
    let mod_file = cache.join(mod_name);
    let mut file = match File::open(&mod_file) {
        Ok(f) => f,
        Err(_) => {
            window.emit("mod-install", "error_file_open").unwrap();
            return;
        }
    };
    // verify the hash of the newly downloaded zip
    let mut hasher = Sha256::new();
    let mut buf_reader = BufReader::new(file);
    let mut buffer = vec![0; 8192];
    while let Ok(read_bytes) = buf_reader.read(&mut buffer) {
        if read_bytes == 0 {
            break;
        }
        hasher.update(&buffer[..read_bytes]);
    }
    let computed_hash = format!("{:x}", hasher.finalize());
    if computed_hash != mod_hash {
        window.emit("mod-install", "error_hash_mismatch").unwrap();
        return;
    }
    // extract the mod into 
}