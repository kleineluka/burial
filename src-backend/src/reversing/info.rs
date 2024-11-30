use serde_json::Value;
// imports
use tauri::Window;
use tauri::command;
use std::fs::File;
use std::io::Read;
use serde_json::json;
use crate::utils::game;
use crate::reversing::sdk;
use crate::modmanager::modloader;

fn parse_plugins(in_path: String) -> Result<Value, Box<dyn std::error::Error>> {
    // read the file
    let file_path = format!("{}//www//js//plugins.js", in_path);
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    // extract the array of plugins
    let start = content.find("var $plugins =").ok_or("Failed to find $plugins array")?;
    let json_start = content[start + "var $plugins =".len()..]
        .trim_start()
        .chars()
        .position(|c| c == '[')
        .ok_or("Failed to find start of JSON array")? + start + "var $plugins =".len();
    let json_end = content[json_start..]
        .chars()
        .position(|c| c == ';')
        .ok_or("Failed to find end of JSON array")? + json_start;
    let plugins_json = &content[json_start..json_end];
    // parse n return
    let parsed: Value = serde_json::from_str(plugins_json)?;
    Ok(parsed)
}

#[command]
pub fn general_info(window: Window, in_path: String, silent: bool) {
    // verify the path is a game path (silent for first load in case they don't have a default)
    if !game::verify_game(&in_path).unwrap() {
        if !silent {
            window.emit("error", Some("Your game path is not valid!".to_string())).unwrap();
        } else {
            window.emit("status", Some("Your currently set game path is not valid!".to_string())).unwrap();
        }
        return;
    }
    // general contains three things: game version, mod loader presence, and sdk presence
    let game= game::game_version(in_path.clone()); // string
    let modloader_presence = modloader::modloader_prescence(in_path.clone()); // either True or False
    let sdk_presence = sdk::sdk_prescence(in_path.clone()); // either Player or Developer
    // format it + pack it into a JSON object
    let modloader_formatted = if modloader_presence { "Installed" } else { "Not Installed" };
    let general_info = json!({
        "game": game,
        "modloader_presence": modloader_formatted,
        "sdk_presence": sdk_presence
    });
    window.emit("status", Some("General information about your TCOAAL loaded!".to_string())).unwrap();
    window.emit("general_info_loaded", Some(general_info)).unwrap();
}

#[command]
pub fn plugins_info(window: Window, in_path: String) {
    // verify the path is a game path
    if !game::verify_game(&in_path).unwrap() {
        window.emit("error", Some("Your game path is not valid!".to_string())).unwrap();
        return;
    }
    // parse the plugins
    let plugins = parse_plugins(in_path.clone());
    match plugins {
        Ok(plugins) => {
            window.emit("status", Some("Plugins information loaded!".to_string())).unwrap();
            window.emit("plugins_info_loaded", Some(plugins)).unwrap();
        }
        Err(e) => {
            window.emit("error", Some(format!("Error while parsing plugins: {}", e))).unwrap();
        }
    }
}