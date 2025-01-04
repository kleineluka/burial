// imports
use tauri::Window;
use tauri::command;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::utils::files;
use crate::utils::game;
use crate::utils::compression;
use crate::config::backups;

// create a backup
#[command]
pub fn create_backup(window: Window, in_path: String, in_name: String) {
    // sanity checks..
    let in_path = Path::new(&in_path);
    let is_game = game::verify_game(&in_path.to_string_lossy()).unwrap();
    if !in_path.exists() || !is_game {
        window.emit("error", Some("That is not a valid TCOAAL folder!".to_string())).unwrap();
        return;
    }
    // get the backup folder + name of new file (or custom name)
    let backup_folder = backups::backup_folder();
    backups::verify_backups().unwrap();
    let backup_name = if in_name == "null" {
        backups::new_backup_name()
    } else {
        in_name
    };
    // ensure a backup with the same name doesn't exist
    let zip_path = backup_folder.join(backup_name.clone() + ".zip");
    if zip_path.exists() {
        window.emit("error", Some("A backup with that name already exists!".to_string())).unwrap();
        return;
    }
    // copy the game folder to the backup folder
    window.emit("status", Some("Cloning game folder..".to_string())).unwrap();
    let backup_path = backup_folder.join(&backup_name);
    files::copy_folder(&in_path.to_string_lossy(), &backup_path.to_string_lossy());
    // zip the backup folder
    window.emit("status", Some("Compressing the backup folder.. (this may take some time)".to_string())).unwrap();
    let zip_path = backup_folder.join(backup_name.clone() + ".zip");
    let file = fs::File::create(&zip_path).unwrap();
    compression::compress_directory(&backup_path, file).unwrap();
    // remove the backup folder
    window.emit("status", Some("Deleting the uncompressed backup..".to_string())).unwrap();
    files::delete_folder(&backup_path.to_string_lossy());
    window.emit("status", Some("Backup complete! Saved as ".to_string() + &*backup_name)).unwrap();
    window.emit("reload-backups", {}).unwrap();
}

// get a list of all backups + return as csv
#[command]
pub fn get_backups(window: Window) {
    backups::verify_backups().unwrap();
    let folder_path = backups::backup_folder();
    let mut backups = Vec::new();
    let mut disk_space = Vec::new();
    // read the directory entries
    let entries = match fs::read_dir(folder_path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to read directory: {}", err);
            window.emit("backups", "null").unwrap();
            return;
        }
    };
    // iterate over the entries
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Failed to read entry: {}", err);
                continue;
            }
        };
        let path = entry.path();
        // check if the entry is a file and has a .zip extension
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "zip" {
                    if let Some(filename) = path.file_name() {
                        // get the disk space of the file
                        let metadata = fs::metadata(&path).unwrap();
                        let size = metadata.len();
                        disk_space.push(size);
                        // make the file name ready for display ! 
                        let filename = filename.to_string_lossy().to_string();
                        let filename = filename.replace(".zip", "");
                        backups.push(filename);
                    }
                }
            }
        }
    }
    // sanitize it as a CSV string 
    let sanitized_backups: String = backups.into_iter().collect::<HashSet<String>>().into_iter().collect::<Vec<String>>().join(",");
    let sanitized_disk_space: String = disk_space.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    let sanitized_data = sanitized_backups.clone() + "|" + &*sanitized_disk_space;
    // send back "null" if no backups found
    if sanitized_backups.is_empty() {
        window.emit("backups", "null").unwrap();
    } else {
        window.emit("backups", sanitized_data).unwrap();
    }
}

// delete a specific backup
#[command]
pub fn delete_backup(window: Window, in_name: String) {
    let backup_folder = backups::backup_folder();
    let zip_path = backup_folder.join(in_name.clone() + ".zip");
    files::delete_file(&zip_path.to_string_lossy());
    window.emit("status", Some("Backup deleted!".to_string())).unwrap();
    window.emit("reload-backups", {}).unwrap();
}

// delete all backups 
#[command]
pub fn clean_backups(window: Window) {
    let backup_folder = backups::backup_folder();
    files::delete_folder(&backup_folder.to_string_lossy());
    backups::verify_backups().unwrap();
    window.emit("status", Some("All backups deleted!".to_string())).unwrap();
    window.emit("reload-backups", {}).unwrap();
}

// overwrite the given game folder with a backup
#[command]
pub fn restore_backup(window: Window, in_path: String, in_name: String) {
    // sanity checks..
    let in_path = Path::new(&in_path);
    let is_game = game::verify_game(&in_path.to_string_lossy()).unwrap();
    if !in_path.exists() || !is_game {
        window.emit("error", Some("That is not a valid TCOAAL folder!".to_string())).unwrap();
        return;
    }
    // get the backup folder + name of new file (or custom name)
    let backup_folder = backups::backup_folder();
    let backup_name = in_name;
    let zip_path = backup_folder.join(backup_name.clone() + ".zip");
    if !zip_path.exists() {
        window.emit("error", Some("That backup does not exist!".to_string())).unwrap();
        return;
    }
    // delete all contents of the game folder
    window.emit("status", Some("Deleting current game contents..".to_string())).unwrap();
    files::clear_folder(&in_path.to_string_lossy());
    // decompress the backup to the game folder
    window.emit("status", Some("Restoring backup to game folder..".to_string())).unwrap();
    compression::decompress_zip(&zip_path, &in_path).unwrap();
    window.emit("status", Some("Backup restored!".to_string())).unwrap();
}

// open backups folder
#[command]
pub fn open_backups() {
    backups::verify_backups().unwrap();
    let backup_folder = backups::backup_folder();
    files::open_folder(&backup_folder.to_string_lossy()).unwrap();
}