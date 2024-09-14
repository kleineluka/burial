    // imports
use tauri::Window;
use tauri::command;
use crate::utils::files;

// open a new window
#[command]
pub fn backup_file(window: Window, in_path: String) {
    window.emit("status", "Backing up file..").unwrap();
    let _ = files::backup_file_multiple(&in_path);
    window.emit("status", "File backed up!").unwrap();
}