// imports
use tauri::Window;
use tauri::command;
use std::option::Option;
use crate::modmaking::converter;
use crate::utils::game;
use crate::utils::services::gamebanana::GamebananaMod;
use crate::utils::services::llamaware::LlamawareMod;
use crate::utils::services::standalone;
use crate::utils::services::standalone::ModSource;
use super::modloader;

/*pub enum ModSource {
    LLamaware, // assign-only for now
    Gamebanana,
    ZipUrl,
    Github,
    Unsupported,
} */
// deterministic installation of foreign mods
// optionally support a window for status updates and a mod json for metadata
pub async fn install_foreign_mod(window: Option<&Window>, in_path: String, mod_path: String, mod_json: Option<converter::ModJson>, mod_source: ModSource) -> String {
   // branch based on mod source
    match mod_source {
        ModSource::Gamebanana => {
            return GamebananaMod::download_mod(window, in_path, mod_path).await;
        }
        // default
        _ => {
            return "unsupported".to_string();
        }
    }
}

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
pub async fn install_mod(window: Window, in_path: String, mod_path: String, mod_hash: String, mod_tags: Vec<String>, mod_json: converter::ModJson) {
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
    // find out what kind of mod it is
    let mut mod_source;
    if mod_tags.contains(&"foreign".to_string()) {
        mod_source = standalone::ModSource::from_url(&mod_path);
    } else {
        mod_source = standalone::ModSource::LLamaware;
    }
    // install dependant based on that
    let mut install_result = "";
    if mod_source == standalone::ModSource::LLamaware {
        install_result = &LlamawareMod::install_mod(Some(&window), in_path, mod_path, mod_hash).await;
    } else {
        install_result = &install_foreign_mod(Some(&window), in_path, mod_path, Some(mod_json), mod_source).await;
    }
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
        window.emit("refresh-mods", "success").unwrap();
        window.emit("status", "Mod uninstalled!").unwrap();
    } else {
        window.emit("refresh-mods", "error").unwrap();
        window.emit("status", "There was an issue uninstalling the mod..").unwrap();
    }
}