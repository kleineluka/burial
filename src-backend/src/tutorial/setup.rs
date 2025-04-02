// imports
use tauri::Window;
use tauri::Manager;
use tauri::command;
use crate::config::storage;
use crate::utils;
use crate::utils::operating::game;
use crate::config::settings;

// try and automatically find the game path for the user, if it's empty
#[command]
pub fn setup_auto_find(window: Window) {
    let current_game_path = storage::read_from_store(&window.app_handle(), "settings-tcoaal").unwrap_or_default();
    // if it's "", try and find the game
    if current_game_path == "" {
        // try and find the game path, emit response
        let game_path = game::find_installation().unwrap_or(None);
        if let Some(path) = game_path {
            window.emit("game-path", path.to_str().unwrap()).unwrap();
            return;
        }
    }
    // emit back an error
    window.emit("game-path", "empty").unwrap();
}   

// initial setup screen when the user saves the game path
#[command]
pub fn setup_game(window: Window, in_path: String) {
    // first, verify it is a valid game path (check if it is empty first)
    if in_path.is_empty() {
        window.emit("game-status", "empty").unwrap();
    } else {
        // if it is not empty, verify it is a valid game path
        if game::verify_game(&in_path).unwrap_or(false) {
            window.emit("game-status", "valid").unwrap();
            setup_settings(window, in_path);
        } else {
            window.emit("game-status", "invalid").unwrap();
        }
    }
}
 
 // move on to here once we know the path is good or the user doesn't care	
 #[command]
 pub fn setup_settings(window: Window, in_path: String) {
    // try and get a default output path, if possible
    let output_path = utils::helpers::files::find_output().unwrap_or_default();
    let output_path_string = output_path.to_str().unwrap_or("").to_string();
     // save the game path
    let settings = settings::Settings {
        tcoaal: String::from(in_path),
        output: output_path_string,
        updates: true,
        theme: String::from("ashley"),
        animations: true,
        tooltips: true,
        modname: String::from(""),
        modid: String::from(""),
        modauthor: String::from(""),
        moddescription: String::from(""),
        deeplinks: true,
        gametarget: String::from("latest"),
    };
    // write the updated settings
    settings::write_settings(settings);
    // emit the status
    window.emit("game-status", "saved").unwrap();
 }