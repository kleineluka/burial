// imports
use tauri::Window;
use tauri::command;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::utils::cipher;
use crate::utils::files;

// perform cipher on a single file
fn encrypt_file_output(window: &Window, in_path: String, out_path: String, advanced_positions: bool) {
    // sanity check.. (and a nifty little log)
    let file_name = files::file_name(&in_path);
    let file_extension = files::file_extension(&in_path);
    if file_extension == "k9a" {
        window.emit("error", Some("You cannot encrypt a .k9a file!".to_string())).unwrap();
        return;
    }
    window.emit("status", Some(format!("Encrypting file: {}.{}", file_name, file_extension))).unwrap();
    // get the decrypted data + new name and file extension
    let (encrypted_data, file_name_with_extension) = cipher::encrypt_file(&in_path, advanced_positions);
    // create the new file path
    let new_out_path = Path::new(&out_path).join(&file_name_with_extension);
    // write the encrypted data to the new file
    files::write_file(&new_out_path.to_string_lossy(), &encrypted_data);
    window.emit("status", Some(format!("File {} has been encrypted.", file_name_with_extension))).unwrap();
}

fn encrypt_folder_output(window: &Window, in_path: String, out_path: String, advanced_positions: bool) {
    // sanity check..
    let in_path = Path::new(&in_path);
    let out_path = Path::new(&out_path);
    if !in_path.is_dir() {
        window.emit("error", Some("Input path is not a folder!".to_string())).unwrap();
        return;
    }
    // index how many files and all folders to search
    window.emit("status", Some("Indexing directory for files..")).unwrap();
    let (file_count, found_folders) = files::index_directory_all(&in_path);
    if file_count == 0 {
        window.emit("error", Some("No files found to encrypt.".to_string())).unwrap();
        return;
    }
    let desired_folders: HashSet<_> = found_folders.into_iter().collect();
    let mut processed_files = 0;
    // recursively traverse the folder
    fn process_directory(window: &Window, in_dir: &Path, out_dir: &Path, desired_folders: &HashSet<String>, processed_files: &mut usize, file_count: usize, advanced_positions: bool) {
        for entry in fs::read_dir(in_dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_dir() {
                // skip the folder if it doesn't contain any files
                let folder_name = path.file_name().unwrap().to_string_lossy().into_owned();
                if !desired_folders.contains(&folder_name) {
                    continue;
                }
                // create corresponding subdirectory in the output path and traverse it
                let new_out_dir = out_dir.join(&folder_name);
                if !new_out_dir.exists() {
                    fs::create_dir_all(&new_out_dir).expect("Failed to create directory.");
                }
                window.emit("status", Some(format!("Encrypting files in folder: {}", folder_name))).unwrap();
                process_directory(window, &path, &new_out_dir, desired_folders, processed_files, file_count, advanced_positions);
            } else {
                // decrypt files in the folder
                if let Some(_ext) = path.extension() {
                    *processed_files += 1;
                    let file_name = files::file_name(&path.to_string_lossy());
                    let file_extension = files::file_extension(&path.to_string_lossy());
                    window.emit("status", Some(format!("Current Progress: {}/{} (encrypting {}.{})", processed_files, file_count, file_name, file_extension))).unwrap();
                    let (encrypted_data, file_name_with_extension) = cipher::encrypt_file(&path.to_string_lossy(), advanced_positions);
                    let out_path = Path::new(&out_dir).join(file_name_with_extension);
                    files::write_file(&out_path.to_string_lossy(), &encrypted_data);
                }
            }
        }
    }
    // start the process
    process_directory(window, &in_path, &out_path, &desired_folders, &mut processed_files, file_count, advanced_positions);
    window.emit("status", Some("All files have been encrypted.")).unwrap();
}

// perform cipher to encrypt
#[command]
pub fn encrypt(window: Window, path_kind: String, in_path: String, out_path: String, advanced_positions: bool) {
    // sanitize if ""
    let in_path = in_path.replace("\"", "");
    let out_path = out_path.replace("\"", "");
    // create the output folder if it doesn't exist
    files::create_path(&out_path);
    // branch between file and folder
    match path_kind.as_str() {
        "file" => {
            encrypt_file_output(&window, in_path, out_path, advanced_positions);
        }
        "folder" => {
            encrypt_folder_output(&window, in_path, out_path, advanced_positions);
        }
        _ => {}
    }
}