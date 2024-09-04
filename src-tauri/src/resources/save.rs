// imports
use std::fs;
use std::path::PathBuf;
use tauri::Window;
use tauri::command;
use crate::config::appdata;
use crate::utils::files;
use crate::utils::rpgsave;

// find all saves in the save folder
#[command]
pub fn find_saves(window: Window) {
    // get the save folder path
     window.emit("status", Some("Loading save files.")).unwrap();
    let save_folder = appdata::save_folder();
    // sanity check
    if !save_folder.exists() {
        window.emit("status", "Save folder does not exist.").unwrap();
        return;
    }
    // iterate over the save folder (recursive)
    fn find_rpgsave_files(dir: &PathBuf, files: &mut Vec<PathBuf>) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).expect("Failed to read directory") {
                let entry = entry.expect("Failed to get directory entry");
                let path = entry.path();
                if path.is_dir() {
                    find_rpgsave_files(&path, files);
                } else if let Some(extension) = path.extension() {
                    if extension == "rpgsave" {
                        files.push(path);
                    }
                }
            }
        }
    }
    // set up vector and call the recursive function
    let mut save_files = Vec::new();
    find_rpgsave_files(&save_folder, &mut save_files);
    // return results to front end
    if save_files.is_empty() {
        window.emit("status", Some("No save files found.")).unwrap();
    } else {
        let file_names: Vec<String> = save_files.into_iter()
            .filter_map(|file| file.file_name().map(|name| name.to_string_lossy().into_owned()))
            .collect();
        let csv = file_names.join(",");
        window.emit("load-saves", Some(csv)).unwrap();
        window.emit("status", Some("Save files loaded!")).unwrap();
    }
}

// backup all rm files to a given path
#[command]
pub fn backup_saves(window: Window, backup_path: String) {
    // get the save folder path
    window.emit("status", Some("Backing up save files.")).unwrap();
    let save_folder = appdata::save_folder();
    // sanity check
    if !save_folder.exists() {
        window.emit("status", "Save folder does not exist.").unwrap();
        return;
    }
    // create the backup folder
    let backup_folder = PathBuf::from(backup_path);
    if !backup_folder.exists() {
        fs::create_dir_all(&backup_folder).expect("Failed to create backup folder");
    }
    // iterate over the save folder (recursive)
    fn backup_rpgsave_files(dir: &PathBuf, backup: &PathBuf) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).expect("Failed to read directory") {
                let entry = entry.expect("Failed to get directory entry");
                let path = entry.path();
                if path.is_dir() {
                    backup_rpgsave_files(&path, backup);
                } else if let Some(extension) = path.extension() {
                    if extension == "rpgsave" {
                        let backup_path = backup.join(path.file_name().expect("Failed to get file name"));
                        fs::copy(&path, &backup_path).expect("Failed to copy file");
                    }
                }
            }
        }
    }
    // call the recursive function
    backup_rpgsave_files(&save_folder, &backup_folder);
    // return results to front end
    window.emit("status", Some("Save files backed up!")).unwrap();
}

// open saves folder
#[command]
pub fn open_saves(window: Window) {
    let save_folder = appdata::save_folder();
    files::open_folder(&save_folder.to_string_lossy()).unwrap();
}

// read and decrypt a save
#[command]
pub fn read_save(window: Window, save_name: String) {
    // get the save folder path
    window.emit("status", Some("Reading save file.")).unwrap();
    let save_folder = appdata::save_folder();
    // sanity check
    if !save_folder.exists() {
        window.emit("status", "Save folder does not exist.").unwrap();
        window.emit("read-save", "error").unwrap();
        return;
    }
    // get the save file path and read it
    let save_path = save_folder.join(save_name);
    let save_data = files::read_file(&save_path.to_string_lossy());
    let save_data_string = String::from_utf8(save_data).unwrap();
    // attempt to decode it
    window.emit("status", Some("Decoding save file.")).unwrap();
    let decoded = rpgsave::decode(&save_data_string);
    let decoded_string = match decoded {
        Ok(data) => data,
        Err(_) => String::from("Failed to decode save file."),
    };
    // send back the decoded save data
    window.emit("status", Some("Save file decoded!")).unwrap();
    window.emit("load-save", Some(decoded_string)).unwrap();
}