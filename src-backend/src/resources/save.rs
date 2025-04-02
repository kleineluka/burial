// imports
use std::fs;
use std::path::PathBuf;
use tauri::Window;
use tauri::command;
use crate::config::appdata;
use crate::utils::helpers::files;
use crate::utils::helpers::rpgsave;

// recursive backup of save files
pub fn backup_rpgsave_files(dir: &PathBuf, backup: &PathBuf) {
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

// iterate over the save folder (recursive)
pub fn delete_rpgsave_files(dir: &PathBuf) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to get directory entry");
            let path = entry.path();
            if path.is_dir() {
                delete_rpgsave_files(&path);
            } else if let Some(extension) = path.extension() {
                if extension == "rpgsave" {
                    fs::remove_file(&path).expect("Failed to delete file");
                }
            }
        }
    }
    }

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

// backup all rpgsave files to a given path
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
    // call the recursive backup function
    backup_rpgsave_files(&save_folder, &backup_folder);
    // return results to front end
    window.emit("status", Some("Save files backed up!")).unwrap();
}

// open saves folder
#[command]
pub fn open_saves(_window: Window) {
    let save_folder = appdata::save_folder();
    files::open_folder(&save_folder.to_string_lossy()).unwrap();
}

// create a copy of a save file by appending "_copy_timestamp" to the file name (before the extension)
#[command]
pub fn copy_save(window: Window, save_name: String) {
    // get the save folder path
    window.emit("status", Some("Copying save file.")).unwrap();
    let save_folder = appdata::save_folder();
    // sanity check
    if !save_folder.exists() {
        window.emit("status", "Save folder does not exist.").unwrap();
        window.emit("copy-save", "error").unwrap();
        return;
    }
    // get the save file path and read it
    let save_path = save_folder.join(save_name);
    // get the file name and extension
    let file_name = save_path.file_stem().expect("Failed to get file name");
    let file_extension = save_path.extension().expect("Failed to get file extension");
    // create the copy file path
    let copy_name = format!("{}_copy_{}", file_name.to_string_lossy(), chrono::Local::now().timestamp());
    let copy_path = save_folder.join(format!("{}.{}", copy_name, file_extension.to_string_lossy()));
    // copy the file
    fs::copy(&save_path, &copy_path).expect("Failed to copy file");
    // return results to front end with the name of the new file in the status message
    window.emit("status", Some(format!("Save file copied to {}.", copy_path.file_name().unwrap().to_string_lossy()))).unwrap();
}

// delete all saves
#[command]
pub fn delete_all(window: Window) {
    // get the save folder path
    window.emit("status", Some("Deleting save files.")).unwrap();
    let save_folder = appdata::save_folder();
    // sanity check
    if !save_folder.exists() {
        window.emit("error", "Save folder does not exist.").unwrap();
        return;
    }
    // call the recursive function
    delete_rpgsave_files(&save_folder);
    // return results to front end
    window.emit("status", Some("Save files deleted!")).unwrap();
}

// delete all auto saves (saves that start with "auto")
#[command]
pub fn delete_auto(window: Window) {
    // get the save folder path
    window.emit("status", Some("Deleting auto save files.")).unwrap();
    let save_folder = appdata::save_folder();
    // sanity check
    if !save_folder.exists() {
        window.emit("error", "Save folder does not exist.").unwrap();
        return;
    }
    // iterate over the save folder (recursive)
    fn delete_auto_rpgsave_files(dir: &PathBuf) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).expect("Failed to read directory") {
                let entry = entry.expect("Failed to get directory entry");
                let path = entry.path();
                if path.is_dir() {
                    delete_auto_rpgsave_files(&path);
                } else if let Some(extension) = path.extension() {
                    if extension == "rpgsave" {
                        let file_name = path.file_stem().expect("Failed to get file name");
                        if file_name.to_string_lossy().starts_with("auto") {
                            fs::remove_file(&path).expect("Failed to delete file");
                        }
                    }
                }
            }
        }
    }
    // call the recursive function
    delete_auto_rpgsave_files(&save_folder);
    // return results to front end
    window.emit("status", Some("Auto save files deleted!")).unwrap();
}

// read and decompress a save
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

// write and compress a save
#[command]
pub fn write_save(window: Window, save_name: String, save_data: String) {
    // get the save folder path
    window.emit("status", Some("Writing save file.")).unwrap();
    let save_folder = appdata::save_folder();
    // sanity check
    if !save_folder.exists() {
        window.emit("status", "Save folder does not exist.").unwrap();
        window.emit("write-save", "error").unwrap();
        return;
    }
    // get the save file path and write it
    let save_path = save_folder.join(save_name);
    // attempt to encode it
    window.emit("status", Some("Encoding save file.")).unwrap();
    let encoded = rpgsave::encode(&save_data);
    let encoded_string = match encoded {
        Ok(data) => data,
        Err(_) => String::from("Failed to encode save file."),
    };
    // write the encoded save data
    let save_data = encoded_string.as_bytes();
    files::write_file(&save_path.to_string_lossy(), save_data);
    // send back the status message
    window.emit("status", Some("Save file written!")).unwrap();
}