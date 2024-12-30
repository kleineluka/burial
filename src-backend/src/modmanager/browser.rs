// imports
use tauri::Window;
use tauri::command;
use std::fs::File;
use std::io::{BufReader, Read};
use sha2::{Sha256, Digest};
use crate::config::downloads;
use crate::modmaking::converter;
use crate::utils::compression;
use crate::utils::connection;
use crate::utils::game;
use super::modloader;
use std::vec;

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

// install a (tomb or foreign mod)
#[command]
pub async fn install_mod(window: Window, in_path: String, mod_path: String, mod_hash: String, mod_tags: Vec<String>, sanitized_name: String, mod_json: converter::ModJson) {
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
    // lock the ui
    window.emit("lock-ui", "enable").unwrap();
    // branch based on whether it is tomb native or not
    if mod_tags.contains(&"foreign".to_string()) {
        install_foreign_mod(window, in_path, mod_path, sanitized_name, mod_json).await;
    } else {
        install_tomb_mod(window, in_path, mod_path, mod_hash, sanitized_name).await;
    }
}

// install a foreign mod
pub async fn install_foreign_mod(window: Window, in_path: String, mod_path: String, sanitized_name: String, mod_json: converter::ModJson) {
    // download mod_path into temp
    let temp_path = downloads::downloads_folder().join(sanitized_name.clone());
    window.emit("status", "Downloading the mod! Please wait, this may take a moment..").unwrap();
    if let Err(e) = connection::download_file(&mod_path, &temp_path.to_string_lossy()).await {
        window.emit("mod-install", "error_connection").unwrap();
        return;
    }
    // there should be a single file inside of the folder now, a zip file, find it
    let mod_file = match temp_path.read_dir().unwrap().next() {
        Some(f) => f.unwrap().path(),
        None => {
            window.emit("mod-install", "error_file_open").unwrap();
            return;
        }
    };  
    // extract that mod into a new folder in the same directory as the mod_file called "non_tomb"
    window.emit("status", "Extracting the mod..").unwrap();
    let mod_folder = temp_path.join("non_tomb");
    compression::decompress_directory_nosub(&mod_file, &mod_folder).unwrap();
    // and convert it
    window.emit("status", "Converting the mod to use Tomb modloader..").unwrap();
    let tomb_mod_folder = temp_path.join("tomb");
    let converted_mod = converter::convert_to_tomb(mod_folder.to_str().unwrap().to_string(), in_path.clone(), tomb_mod_folder.to_str().unwrap().to_string(), mod_json.name, mod_json.id, mod_json.authors, mod_json.description, mod_json.version);
    if converted_mod == "error:game_path" || converted_mod == "error:mod_path" {
        window.emit("mod-install", "error_conversion").unwrap();
        return;
    }
    // copy the converted mod to the game folder
    window.emit("status", "Installing the mod..").unwrap();
    // there should be a single file inside of tomb_mod_folder now, a folder, find it
    let mod_folder = match tomb_mod_folder.read_dir().unwrap().next() {
        Some(f) => f.unwrap().path(),
        None => {
            window.emit("mod-install", "error_conversion").unwrap();
            return;
        }
    };
    // move it to the game folder
    let game_path = std::path::Path::new(&in_path);
    let mod_path = game_path.join("tomb").join("mods");
    std::fs::rename(&mod_folder, &mod_path.join(sanitized_name)).unwrap();
    // clear the temp folder
    window.emit("status", "Cleaning up..").unwrap();
    downloads::clear_downloads().unwrap();
    // all done!
    window.emit("status", "Mod installed successfully!").unwrap();
    window.emit("mod-install", "success").unwrap();
    window.emit("lock-ui", "disable").unwrap();
}

// install a tomb mod
pub async fn install_tomb_mod(window: Window, in_path: String, mod_path: String, mod_hash: String, sanitized_name: String) {
    // start downloading the mod
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
    let mod_folder = format!("{}/tomb/mods/{}", in_path.clone(), sanitized_name);
    let mod_folder_path = std::path::Path::new(&mod_folder);
    // if the mod folder already exists, delete it
    if mod_folder_path.exists() {
        window.emit("status", "Removing the previous installation of the mod..").unwrap();
        std::fs::remove_dir_all(&mod_folder).unwrap();
    }
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

// uninstall a mod
#[command]
pub fn uninstall_mod(window: Window, mod_path: String) {
    // simply delete the folder
    window.emit("status", "Uninstalling mod..").unwrap();
    let mod_path = std::path::Path::new(&mod_path);
    if mod_path.exists() {
        // if the folder we are deleting (ex. the directory ends in /tomb) is tomb we can't delete it! (it's a core mod)
        if mod_path.ends_with("tomb") {
            window.emit("error", "You can't uninstall the core Tomb mod!").unwrap();
            window.emit("status-clear", "").unwrap();
            return;
        }
        std::fs::remove_dir_all(mod_path).unwrap();
        window.emit("reload-mods", "success").unwrap();
        window.emit("status", "Mod uninstalled!").unwrap();
    } else {
        window.emit("reload-mods", "error").unwrap();
        window.emit("status", "There was an issue uninstalling the mod..").unwrap();
    }
}