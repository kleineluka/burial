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

#[derive(Serialize, Deserialize, Debug)]
struct Plugin {
    name: String,
    status: bool,
    description: String,
}

// get the game version from the files
#[command]
pub fn game_version(window: Window, in_path: String) {
    // verify the path is a game path
    if !game::verify_game(&in_path).unwrap() {
        window.emit("status", Some("Your game path is not valid!".to_string())).unwrap();
        return;
    }
    // navigate from in_path to www/js/main.js
     let main_js_path = format!("{}/www/js/main.js", in_path);
    // open main.js
    let file = File::open(main_js_path).unwrap();
    let reader = BufReader::new(file);
    // find const GAME_VERSION = "ANYTHING GOES HERE";
     for line in reader.lines() {
        let line = line.unwrap();
        // Look for the line that contains the GAME_VERSION constant
        if line.contains("const GAME_VERSION") {
            // Extract the value between quotes
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    // Return the extracted version
                    let game_version = &line[start + 1..start + 1 + end];
                    window.emit("game-version", Some(game_version.to_string())).unwrap();
                    return;
                }
            }
        }
    }
    // extract what is in the quotes
    window.emit("status", Some("Game Version Not Found...".to_string())).unwrap();
}

#[tauri::command]
pub fn parse_plugins(in_path: String) -> Result<String, Error> {
    // Define the path to the plugins.js file
    let plugins_js_path = format!("{}/plugins.js", in_path);

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