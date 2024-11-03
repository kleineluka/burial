// imports
use tauri::Window;
use tauri::command;
use crate::utils::files;

// read mod json
#[command]
pub fn load_modjson(window: Window, in_path: String) {
    if !files::file_exists(&in_path) {
        window.emit("error", "The mod.json file does not exist!").unwrap();
        return;
    }
    let modjson = files::read_file(&in_path);
    let modjson_string = String::from_utf8(modjson).unwrap();
    window.emit("load-modjson", modjson_string).unwrap();
    window.emit("status", "Loaded the mod.json file!").unwrap();
}

// write mod json
#[command]
pub fn save_modjson(window: Window, in_path: String, modjson: String) {
    // make a file at the given path, if it is not empty
    if in_path.is_empty() {
        window.emit("error", "The mod.json file path is empty!").unwrap();
        return;
    }
    files::write_file(&in_path, modjson.as_bytes());
    window.emit("status", "Saved the mod.json file!").unwrap();
}