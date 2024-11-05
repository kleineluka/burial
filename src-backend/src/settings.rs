// imports
use std::fs;
use tauri::Window;
use tauri::Manager;
use tauri::command;
use crate::config::settings;
use crate::config::storage;
use crate::config::cache;
use crate::utils::deno;
use crate::utils::files;
use crate::utils::game;
use crate::utils::hausmaerchen;

// load settings
#[command]
pub fn load_settings(window: Window) {
    // first, ensure it exists + read
    settings::check_settings();
    let settings = settings::read_settings();
    let settings_json = serde_json::to_string(&settings)
        .expect("Failed to serialize settings");
    // send the settings to the frontend
    window.emit("settings-loaded", settings_json).unwrap();
}

// write settings
#[command]
pub fn save_settings(window: Window, tcoaal: String, output: String, biginstance: bool, updates: bool, theme: String, animations: bool) {
    // read the current settings
    let mut settings = settings::read_settings();
    // update the settings
    settings.tcoaal = tcoaal;
    settings.output = output;
    settings.biginstance = biginstance;
    settings.updates = updates;
    settings.theme = theme;
    settings.animations = animations;
    // write the updated settings
    settings::write_settings(settings);
    window.emit("settings-saved", {}).unwrap(); 
}

// reset settings
#[command]
pub fn reset_settings(window: Window) {
    settings::delete_settings();
    settings::check_settings();
    window.emit("settings-reset", {}).unwrap();
}

// remove deno
#[command]
pub fn remove_deno(window: Window) {
    deno::remove_deno();
    window.emit("deno-removed", {}).unwrap();
}

// remove hausmaerchen
#[command]
pub fn remove_hausmaerchen(window: Window) {
    hausmaerchen::remove_hausmaerchen();
    window.emit("hausmaerchen-removed", {}).unwrap();
}

// install dev tools (well, copy them over, really)
#[command]
pub fn install_dev_tools(window: Window) {
    // first, verify the cache
    cache::verify_cache().unwrap();
    // get the cache folder and where it will be installed to
    let cache = cache::cache_folder();
    let dev_tools_path = cache.join("devtools");
    // if it exists, remove it
    if dev_tools_path.exists() {
        std::fs::remove_dir_all(&dev_tools_path).unwrap();
    }
    // create the devtools folder
    std::fs::create_dir_all(&dev_tools_path).unwrap();
    // get the devtools from the resource path
    let resource_path = storage::read_from_store(&window.app_handle(), "state-bundled-resources").expect("Failed to read from store");
    let mut resource_dev_tools = std::path::PathBuf::from(resource_path.as_str().unwrap());
    resource_dev_tools.push("devtools");
    // copy the resource hausmaerchen to the cache hausmaerchen
    files::copy_directory(&resource_dev_tools.to_string_lossy(), &dev_tools_path.to_string_lossy()).unwrap();
    window.emit("dev-tools-installed", {}).unwrap();
    // open the devtools folder
    files::open_folder(&dev_tools_path.to_string_lossy()).unwrap();
}

// try and automatically find the game path for the user, if it's empty
#[command]
pub fn settings_auto_find(window: Window) {
    // try and find game path, emit response
    let game_path = game::find_installation().unwrap_or(None);
    if let Some(path) = game_path {
        window.emit("game-path", path.to_str().unwrap()).unwrap();
        return;
    }
    // emit back an error
    window.emit("game-path", "empty").unwrap();
}   

// automatically create an output folder in the documents (documents + Burial) or whatever os equivalent
#[command]
pub fn output_auto_find(window: Window) {
    // try and find output path, emit response
    let output_path = files::find_output();
    if let Some(path) = output_path {
        window.emit("output-path", path.to_str().unwrap()).unwrap();
        return;
    }
    // emit back an error
    window.emit("output-path", "empty").unwrap();
}   