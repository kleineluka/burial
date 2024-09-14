// imports
use std::fs;
use std::io::Write;
use tauri::Window;
use tauri::command;
use crate::config::cache;

// save that the setup has already been run
#[command]
pub fn setup_finish(window: Window) {
    let cache_dir = cache::cache_folder();
    let file_path = cache_dir.join("setup.lock");
    let mut file = fs::File::create(file_path)
        .expect("Failed to create file");
    file.write_all("done".as_bytes())
        .expect("Failed to write data to file");
    window.emit("setup-status", "finished").unwrap();
}