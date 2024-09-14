// imports
use tauri::Window;
use tauri::command;
use crate::utils::game;
use crate::config::settings;

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
     // save the game path
    let settings = settings::Settings {
        tcoaal: String::from(in_path),
        output: String::from(""),
    };
    // write the updated settings
    settings::write_settings(settings);
    // emit the status
    window.emit("game-status", "saved").unwrap();
 }