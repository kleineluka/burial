use serde::{Deserialize, Serialize};
use tauri::{command, Window};
use crate::modmanager::browser;
use crate::{config::cache, modmaking::converter, utils::game};
use crate::utils::services::standalone;

use super::modloader;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModPack {
    pub name: String,
    pub lastUpdate: String,
    pub mods: Vec<ModPackMod>,
    pub icon: Option<String>,  
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModPackMod {
    pub name: String,
    pub sha256: String,
    pub tags: Vec<String>,
    pub modJson: converter::ModJson,
    pub modUrl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModpackLock {
    pub name: String,
    pub lastUpdate: String,
}

// write the current modpack to the file
pub fn write_modpack(modpack: ModpackLock) {
    // get the cache folder
    let cache_dir = cache::cache_folder();
    // create the file path
    let file_path = cache_dir.join(".modpack.lock");
    // write the file
    std::fs::write(file_path, serde_json::to_string(&modpack).unwrap())
        .expect("Failed to write modpack to file");
}

// get the user's current modpack and when it was last updated
pub fn read_modpack() -> ModpackLock {
    // get the cache folder
    let cache_dir = cache::cache_folder();
    // create the file path
    let file_path = cache_dir.join(".modpack.lock");
    // if it doesn't exist, return "vanilla"
    if !file_path.exists() {
        return ModpackLock {
            name: "vanilla".to_string(),
            lastUpdate: "never".to_string(),
        };
    }
    // read the file
    let file = std::fs::read_to_string(file_path)
        .expect("Failed to read modpack from file");
    // parse the file
    let modpack: ModpackLock = serde_json::from_str(&file)
        .expect("Failed to parse modpack from file");
    // return the modpack
    modpack
}

// install modpack
//window: Window, in_path: String, mod_path: String, mod_hash: String, mod_tags: Vec<String>, mod_json: converter::ModJson
#[command]
pub async fn install_modpack(window: Window, in_path: String, modpack_entry: ModPack) {
    // first, clean out the game
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "Please set a valid game directory in the settings first!").unwrap();
        return;
    }
    // determine if already modded
    window.emit("status", "Cleaning up..").unwrap();
    let is_modded = modloader::modloader_prescence(in_path.clone());
    if is_modded {
        // uninstall the modloader
        let _uninstalled = modloader::uninstall_current_modloader(&in_path);
    }
    // install the modloader
    window.emit("status", "Installing modloader..").unwrap();
    let _installed = modloader::install_latest(in_path.clone()).await;
    // write the modpack lock
    window.emit("status", "Saving modpack info..").unwrap();
    let modpack_lock = ModpackLock {
        name: modpack_entry.name.clone(),
        lastUpdate: modpack_entry.lastUpdate.clone(),
    };
    write_modpack(modpack_lock);
    // double-triple-x100 check that the mods folder is empty (butchered past installations)
    let mods_folder = std::path::Path::new(&in_path).join("tomb").join("mods");
    if mods_folder.exists() {
        std::fs::remove_dir_all(mods_folder).unwrap();
    }
    // go through every mod and install it
    for mod_entry in modpack_entry.mods {
        // install the mod
        window.emit("status", format!("Installing mod {}..", mod_entry.name)).unwrap();
        let mut mod_source;
        if mod_entry.tags.contains(&"foreign".to_string()) {
            mod_source = standalone::ModSource::from_url(&mod_entry.modUrl);
        } else {
            mod_source = standalone::ModSource::LLamaware;
        }
        let _installed = browser::install_and_download(None, in_path.clone(), mod_entry.modUrl, Some(mod_entry.modJson), Some(mod_entry.sha256), mod_source).await;
    }
    // done!
    window.emit("status", "Modpack installed! Have fun ^_^").unwrap();
}

// uninstall modpack
#[command]
pub fn uninstall_modpack(window: Window, in_path: String) {

}