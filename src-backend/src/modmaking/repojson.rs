// imports
use tauri::Window;
use tauri::command;
use crate::utils::helpers::files;

// read repo json
#[command]
pub fn load_repojson(window: Window, in_path: String) {
    if !files::file_exists(&in_path) {
        window.emit("error", "The mod.json file does not exist!").unwrap();
        return;
    }
    let repojson = files::read_file(&in_path);
    let repojson_string = String::from_utf8(repojson).unwrap();
    window.emit("load-repojson", repojson_string).unwrap();
    window.emit("status", "Loaded the repo.json file!").unwrap();
}

// write repo json
#[command]
pub fn save_repojson(window: Window, in_path: String, repojson: String) {
    // make a file at the given path, if it is not empty
    if in_path.is_empty() {
        window.emit("error", "The repo.json file path is empty!").unwrap();
        return;
    }
    files::write_file(&in_path, repojson.as_bytes());
    window.emit("status", "Saved the repo.json file!").unwrap();
}