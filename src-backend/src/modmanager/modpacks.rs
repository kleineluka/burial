use std::f32::consts::E;
use std::fs;

use serde::{Deserialize, Serialize};
use tauri::{command, Window};
use crate::config::appdata::{self, save_folder};
use crate::modmanager::browser;
use crate::resources::save;
use crate::utils::compression;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
pub async fn install_modpack(window: Window, in_path: String, modpack_entry: ModPack, backup_saves: bool, out_path: String) {
    // first, clean out the game
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "Please set a valid game directory in the settings first!").unwrap();
        return;
    }
    // (optionally backup saves)
    if backup_saves {
        window.emit("status", "Backing up save files..").unwrap();
        let save_folder = appdata::save_folder();
        if !save_folder.exists() {
            window.emit("status", "No saves were found, skipping save backup..").unwrap();
        } else {
            // passed sanity check, now backup the saves (out_dir = out_path/"pre"_modpack_name_timestamp)
            let sanitized_modpack_name = standalone::sanitize_mod_folder_name(&modpack_entry.name);
            let out_dir = save_folder.join(out_path)
                .join("Saves")
                .join("Saves_Pre_".to_string() + 
                &sanitized_modpack_name + "_" +
                &chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string());
            std::fs::create_dir_all(&out_dir).unwrap();
            save::backup_rpgsave_files(&save_folder, &out_dir);
            // zip the backup (+del original)
            let out_file_path = out_dir.with_extension("zip");
            let out_file = fs::File::create(&out_file_path).unwrap();
            compression::compress_directory(&out_dir, &out_file).unwrap();  
            std::fs::remove_dir_all(&out_dir).unwrap();
            // delete the saves
            save::delete_rpgsave_files(&save_folder);
        }
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
        // delete any folder inside of the top level except "tomb"
        for entry in std::fs::read_dir(mods_folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if path.ends_with("tomb") {
                    continue;
                }
                std::fs::remove_dir_all(path).unwrap();
            }
        }
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

// get the current modpack from the lock
#[command]
pub fn current_modpack(window: Window) {
    let modpack = read_modpack();
    window.emit("current-modpack", modpack).unwrap();
}

// uninstall modpack
#[command]
pub fn uninstall_modpack(window: Window, in_path: String) {
    // see if a modpack is installed
    let modpack = read_modpack();
    if modpack.name == "vanilla" {
        window.emit("error", "No modpack is currently installed.").unwrap();
        return;
    }
    // make sure the game is valid
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "Please set a valid game directory in the settings first!").unwrap();
        return;
    }
    // uninstall the modpack
    window.emit("status", "Uninstalling modpack..").unwrap();
    write_modpack(ModpackLock {
        name: "vanilla".to_string(),
        lastUpdate: "never".to_string(),
    });
    modloader::uninstall_current_modloader(&in_path);
    window.emit("modpack-uninstalled", "success").unwrap();
}