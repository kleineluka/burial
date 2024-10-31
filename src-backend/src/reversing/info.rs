// imports
use tauri::Window;
use tauri::command;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use regex::Regex;
use crate::utils::game;
use crate::reversing::sdk;
use crate::modmanager::modloader;

#[derive(Serialize, Deserialize, Debug)]
struct Plugin {
    name: String,
    status: bool,
    description: String,
}

#[allow(dead_code)]
#[tauri::command]
pub fn parse_plugins(in_path: String) -> Result<String, Error> {
    // Define the path to the plugins.js file
    let plugins_js_path = format!("{}/(plugins.js", in_path);

    // Read the contents of the plugins.js file
    let contents = fs::read_to_string(plugins_js_path)?;

    // Remove the non-JSON parts (comments and the assignment)
    let re = Regex::new(r"(?s)//.*?\n|\s*var \$plugins\s*=\s*|\[|\];").unwrap();
    let cleaned_content = re.replace_all(&contents, "").to_string();

    // Parse the cleaned content as JSON
    let plugins: Vec<Plugin> = serde_json::from_str(&cleaned_content)?;

    // Create a new JSON object where each plugin's name is a key
    let mut plugins_json = json!({});

    for plugin in plugins {
        plugins_json[&plugin.name] = json!({
            "status": plugin.status,
            "description": plugin.description
        });
    }

    // Convert the resulting JSON object to a pretty-printed string
    Ok(serde_json::to_string_pretty(&plugins_json)?)
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
    let game= game::game_version(in_path.clone()); // String
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