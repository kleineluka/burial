// imports
use tauri::Window;
use tauri::command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::utils::game;

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