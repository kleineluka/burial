// imports
use tauri::Window;
use tauri::command;

// get the version of the application
#[command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}