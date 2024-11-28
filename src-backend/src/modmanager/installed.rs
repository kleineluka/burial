// imports
use tauri::Window;
use tauri::command;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use crate::utils::game;
use super::modloader;

// structure containing mod folder and mod.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct ModFolder {
    pub folder: String,
    pub modjson: ModJson,
}

// structure containing mod information (keep in mind some might miss some fields.. sigh)
#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(flatten)] // for other properties we don't need when deserializing
    pub extra: Option<HashMap<String, serde_json::Value>>,
}     

// get all of the installed mods
pub fn get_installed_mods(in_path: String) -> Vec<ModFolder> {
    // mods will be present: in_path + www + mods and there each folder will be a mod, and inside that folder, a mod.json file
    let mods_path = format!("{}//tomb//mods", in_path);
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
                if let Ok(modjson) = serde_json::from_str::<ModJson>(&modjson_raw) {
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
        window.emit("mod-uninstall", "success").unwrap();
        window.emit("status", "Mod uninstalled!").unwrap();
    } else {
        window.emit("mod-uninstall", "error").unwrap();
        window.emit("status", "There was an issue uninstalling the mod..").unwrap();
    }
}