// imports
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use tauri::Window;
use tauri::command;
use sha2::{Sha256, Digest};
use crate::config::downloads;
use crate::utils::compression;
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
    // verify that the game path is right
    window.emit("status", "Making sure that everything is ready..").unwrap();
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("mod-install", "error_game_path").unwrap();
        return;
    }
    // make sure that the modloader is present
    let modloader_presence = modloader::modloader_prescence(in_path.clone());
    if !modloader_presence {
        window.emit("mod-install", "error_modloader").unwrap();
        return;
    }
    // download the mod (and lock the ui from doing other stuff..)
    window.emit("lock-ui", "enable").unwrap();
    window.emit("status", "Downloading the mod! Please wait, this may take a moment..").unwrap();
    let downloads = downloads::downloads_folder();
    if let Err(e) = connection::download_file(&mod_path, &downloads.to_string_lossy()).await {
        window.emit("mod-install", "error_connection").unwrap();
        return;
    }
    // get the name of the zip file and then open it
    window.emit("status", "Verifying the hash of the mod..").unwrap();
    let mod_name = mod_path.split("/").last().unwrap();
    let mod_file = downloads.join(mod_name);
    let file = match File::open(&mod_file) {
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
    // extract the mod contents into the game folder/tomb/mods/<mod name>
    window.emit("status", "Extracting the mod into the game directory..").unwrap();
    let mod_folder = format!("{}/tomb/mods/{}", in_path.clone(), mod_name.replace(".zip", ""));
    let mod_folder_path = std::path::Path::new(&mod_folder);
    compression::decompress_directory(&mod_file, &mod_folder_path).unwrap();
    // delete the downloads folder 
    window.emit("status", "Cleaning up..").unwrap();
    std::fs::remove_file(&mod_file).unwrap();
    downloads::clear_downloads().unwrap();
    // all done!
    window.emit("status", "Mod installed successfully!").unwrap();
    window.emit("mod-install", "success").unwrap();
    window.emit("lock-ui", "disable").unwrap();
}