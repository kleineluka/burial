// imports
use tauri::Window;
use tauri::command;
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
