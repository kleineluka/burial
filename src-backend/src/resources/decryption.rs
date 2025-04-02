// imports
use tauri::Window;
use tauri::command;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::utils::nemlei::cipher;
use crate::utils::helpers::files;

// perform cipher on a single file
fn decrypt_file_output(window: &Window, in_path: String, out_path: String) {
    // sanity check.. (and a nifty little log)
    let file_name = files::file_name(&in_path);
    let file_extension = files::file_extension(&in_path);
    window.emit("status", Some(format!("Decrypting file: {}.{}", file_name, file_extension))).unwrap();
    // get the decrypted data + new name and file extension
    let (decrypted_data, file_name_with_extension) = cipher::decrypt_file(&in_path);
    let detected_extension = cipher::get_extension_from_mime(&decrypted_data);
    let proper_path = format!("{}.{}", file_name_with_extension, detected_extension);
    // create the new file path
    let new_out_path = Path::new(&out_path).join(&proper_path);
    // write the decrypted data to the new file
    files::write_file(&new_out_path.to_string_lossy(), &decrypted_data);
    window.emit("status", Some(format!("File {} has been decrypted.", file_name_with_extension))).unwrap();
}

// perform cipher on a single folder, recursively, while retaining original file structure
fn decrypt_folder_output(window: &Window, in_path: String, out_path: String) {
    // sanity check..
    let in_path = Path::new(&in_path);
    let out_path = Path::new(&out_path);
    if !in_path.is_dir() {
        window.emit("error", Some("Input path is not a folder!".to_string())).unwrap();
        return;
    }
    // index the directory to get count + locations
    let desired_folders: HashSet<_> = files::index_directory_all(&in_path).1.into_iter().collect();
    let mut processed_files = 0;
    // recursively traverse the folder
    fn process_directory(window: &Window, in_dir: &Path, out_dir: &Path, desired_folders: &HashSet<String>, processed_files: &mut usize) {
        for entry in fs::read_dir(in_dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_dir() {
                // skip the folder if it doesn't contain any .k9a files
                let folder_name = path.file_name().unwrap().to_string_lossy().into_owned();
                if !desired_folders.contains(&folder_name) {
                    continue;
                }
                // create corresponding subdirectory in the output path and traverse it
                let new_out_dir = out_dir.join(&folder_name);
                if !new_out_dir.exists() {
                    fs::create_dir_all(&new_out_dir).expect("Failed to create directory.");
                }
                window.emit("status", Some(format!("Decrypting files in folder: {}", folder_name))).unwrap();
                process_directory(window, &path, &new_out_dir, desired_folders, processed_files);
            } else {
                // decrypt files in the folder
                *processed_files += 1;
                window.emit("status", Some(format!("Processed Files: {}", processed_files))).unwrap();
                let (decrypted_data, file_name_with_extension) = cipher::decrypt_file(&path.to_string_lossy());
                let detected_extension = cipher::get_extension_from_mime(&decrypted_data);
                let proper_name = format!("{}.{}", file_name_with_extension, detected_extension);
                let out_path = Path::new(&out_dir).join(&proper_name);
                files::write_file(&out_path.to_string_lossy(), &decrypted_data);
                *processed_files += 1;
            }
        }
    }
    // start the process
    process_directory(window, &in_path, &out_path, &desired_folders, &mut processed_files);
    window.emit("status", Some("All .k9a files have been decrypted.")).unwrap();
}

// perform cipher to decrypt
#[command]
pub fn decrypt(window: Window, path_kind: String, in_path: String, out_path: String) {
    // sanitize if ""
    let in_path = in_path.replace("\"", "");
    let out_path = out_path.replace("\"", "");
    // create the output folder if it doesn't exist
    files::create_path(&out_path);
    // branch between file and folder
    match path_kind.as_str() {
        "file" => {
            decrypt_file_output(&window, in_path, out_path);
        }
        "folder" => {
            decrypt_folder_output(&window, in_path, out_path);
        }
        _ => {}
    }
}
