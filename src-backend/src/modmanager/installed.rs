// imports
use tauri::Window;
use tauri::command;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::config::disabled;
use crate::utils::operating::game;
use super::modloader;

// structure containing mod folder and mod.json file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModFolder {
    pub folder: String,
    pub modjson: ModJson,
}

// structure containing mod information (keep in mind some might miss some fields.. sigh)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModJson {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default = "default_status")]
    pub status: bool,
    #[serde(flatten)] // for other properties we don't need when deserializing
    pub extra: Option<HashMap<String, serde_json::Value>>,
}     

fn default_status() -> bool {
    true
}

// disable a mod by moving it to the disabled folder
pub fn disable_mod_folder(in_path: String) -> Result<(), String> {
    let mod_path = Path::new(&in_path);
    if !mod_path.exists() {
        return Err(format!("Mod path '{}' does not exist", in_path));
    }
    let mod_name = mod_path.file_name().ok_or("Invalid mod path: missing file name")?;
    disabled::verify_disabled().unwrap();
    let disabled_path = disabled::disabled_folder().join(mod_name);
    fs::create_dir_all(disabled::disabled_folder()).map_err(|e| format!("Failed to create disabled folder: {}", e))?;
    fs::rename(mod_path, &disabled_path).map_err(|e| format!("Failed to move mod to disable: {}", e))?;
    Ok(())
}

// enable a mod by moving it back to the mods folder
pub fn enable_mod_folder(in_path: String, game_path: String) -> Result<(), String> {
    let mod_path = Path::new(&in_path);
    if !mod_path.exists() {
        return Err(format!("Mod path '{}' does not exist", in_path));
    }
    let mod_name = mod_path.file_name().ok_or("Invalid mod path: missing file name")?;
    let mods_path = Path::new(&game_path).join("tomb").join("mods").join(mod_name);
    fs::rename(mod_path, &mods_path).map_err(|e| format!("Failed to move mod: {}", e))?;
    Ok(())
}

// get all of the installed mods
pub fn get_installed_mods(in_path: String) -> Vec<ModFolder> {
    // mods will be present: in_path + www + mods and there each folder will be a mod, and inside that folder, a mod.json file
    let mods_path = format!("{}/tomb/mods", in_path);
    if !Path::new(&mods_path).exists() {
        return Vec::new();
    }
    let mut mods: Vec<ModFolder> = Vec::new();
    // read all the folders in the mods folder
    for entry in std::fs::read_dir(mods_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // check if the entry is a folder
        if path.is_dir() {
            // read the mod.json file
            let modjson_path = format!("{}/mod.json", path.display());
            // safely attempt to read the mod.json file
            if let Ok(modjson_raw) = std::fs::read_to_string(modjson_path) {
                // parse the JSON, ignoring extra fields
                if let Ok(mut modjson) = serde_json::from_str::<ModJson>(&modjson_raw) {
                    modjson.status = true;
                    mods.push(ModFolder {
                        folder: path.display().to_string(),
                        modjson,
                    });
                } else {
                    eprintln!("Error parsing mod.json in folder: {:?}", path);
                }
            } else {
                eprintln!("Could not read mod.json in folder: {:?}", path);
            }
        }
    }
    // get all mods in the disabled folder, if any
    disabled::verify_disabled().unwrap();
    let disabled_path = disabled::disabled_folder();
    for entry in std::fs::read_dir(disabled_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // check if the entry is a folder
        if path.is_dir() {
            // read the mod.json file
            let modjson_path = format!("{}/mod.json", path.display());
            // safely attempt to read the mod.json file
            if let Ok(modjson_raw) = std::fs::read_to_string(modjson_path) {
                // parse the JSON, ignoring extra fields
                if let Ok(mut modjson) = serde_json::from_str::<ModJson>(&modjson_raw) {
                    modjson.status = false;
                    mods.push(ModFolder {
                        folder: path.display().to_string(),
                        modjson,
                    });
                } else {
                    eprintln!("Error parsing mod.json in folder: {:?}", path);
                }
            } else {
                eprintln!("Could not read mod.json in folder: {:?}", path);
            }
        }
    }
    mods
}

#[command]
pub fn installed_mods(window: Window, in_path: String) {
    // verify it is a game path, first..
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("installed-mods", "error_game_path").unwrap();
        return;
    }
    // make sure that mod loader is present
    let modloader_presence = modloader::modloader_prescence(in_path.clone());
    if !modloader_presence {
        window.emit("installed-mods", "error_modloader").unwrap();
        return;
    }
    // get the installed mods
    let mods = get_installed_mods(in_path);
    window.emit("installed-mods", Some(&mods)).unwrap();
}

#[command]
pub fn disable_mod(window: Window, in_path: String) {
    window.emit("status", "Disabling the mod...").unwrap();
    let result = disable_mod_folder(in_path);
    match result {
        Ok(_) => {
            window.emit("status", "Mod disabled successfully!").unwrap();
        },
        Err(e) => {
            window.emit("status", &format!("Error disabling mod: {}", e)).unwrap();
        }
    }
    window.emit("refresh-mods", "").unwrap();
}

#[command]
pub fn enable_mod(window: Window, in_path: String, game_path: String) {
    window.emit("status", "Enabling the mod...").unwrap();
    let result = enable_mod_folder(in_path, game_path);
    match result {
        Ok(_) => {
            window.emit("status", "Mod enabled successfully!").unwrap();
        },
        Err(e) => {
            window.emit("status", &format!("Error enabling mod: {}", e)).unwrap();
        }
    }
    window.emit("refresh-mods", "").unwrap();
}