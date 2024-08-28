// imports
use tauri::Window;
use tauri::command;
use std::fs;
use std::path::Path;
use crate::utils::cipher;
use crate::utils::files;

// perform cipher on a single file
fn decrypt_file(window: &Window, in_path: String, out_path: String) {
    // get the input file name and extension (for log)
    let vanilla_name = files::file_name(&in_path);
    let vanilla_extension = files::file_extension(&in_path);
    window.emit("log", Some(format!("Decrypting file: {}.{}", vanilla_name, vanilla_extension))).unwrap();
    // read the file, and decrypt it..
    let data = files::read_file(&in_path);
    let decrypted_data = cipher::decrypt_file(&data, &in_path);
    // get the file name extension
    let file_name = files::file_name(&in_path);
    let file_extension = cipher::file_extension(&data);
    // create the new path given the out_path, file name, and file extension
    let out_path = Path::new(&out_path).join(format!("{}.{}", file_name, file_extension));
    let out_path_str = out_path.to_string_lossy();
    files::write_file(&out_path_str, &decrypted_data);
}

// perform cipher on a single folder, recursively, while retaining original file structure
fn decrypt_folder(window: &Window, in_path: String, out_path: String) {
    // sanity check..
    let in_path = Path::new(&in_path);
    let out_path = Path::new(&out_path);
    if !in_path.is_dir() {
        println!("input path: {:?}", in_path);
        window.emit("error", Some("Input path is not a folder!".to_string())).unwrap();
        return;
    }
    // recursively traverse the folder
    fn process_directory(window: &Window, in_dir: &Path, out_dir: &Path) {
        for entry in fs::read_dir(in_dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            // either keep traversing or start decrypting
            if path.is_dir() {
                // recursively process + create subdirectories
                let new_out_dir = out_dir.join(path.file_name().expect("Failed to get file name"));
                if !new_out_dir.exists() {
                    fs::create_dir_all(&new_out_dir).expect("Failed to create directory.");
                }
                process_directory(window, &path, &new_out_dir);
            } else {
                // decrypt files in folder
                if let Some(ext) = path.extension() {
                    if ext == "k9a" {
                        let in_path_str = path.to_string_lossy();
                        let out_path_str = out_dir.to_string_lossy();
                        decrypt_file(window, in_path_str.to_string(), out_path_str.to_string());
                    }
                }
            }
        }
    }
    // start the recursion
    process_directory(window, &in_path, &out_path);
}

// perform cipher, recieve three values from front-end: kind of path (file or folder), path to decrypt, and path to save
#[command]
pub fn decrypt(window: Window, path_kind: String, in_path: String, out_path: String, show_logs: bool) {
    // remove " from the paths (like how Windows automatically adds them)
    let in_path = in_path.replace("\"", "");
    let out_path = out_path.replace("\"", "");
    // create the output folder if it doesn't exist
    files::create_path(&out_path);
    // branch between file and folder
    match path_kind.as_str() {
        "file" => {
            // if in_path's extension is not .k9a, return an error
            // note: keep it here because we reuse decrypt_file in folder!
            let file_extension = files::file_extension(&in_path);
            if file_extension != "k9a" {
                window.emit("error", Some("Input file is not a .k9a file!".to_string())).unwrap();
                return;
            }
            decrypt_file(&window, in_path, out_path);
            // send a success mesasge and event to the window (here, again, for same reason)
            if show_logs {
                window.emit("log", Some("File is done processing.".to_string())).unwrap();
                window.emit("success", Some("Your file has been successfully decrypted!".to_string())).unwrap();
            }
        }
        "folder" => {
            // perform decryption on the folder
            decrypt_folder(&window, in_path, out_path);
            if show_logs {
                window.emit("log", Some("Folder is done processing.".to_string())).unwrap();
                window.emit("success", Some("Your folder has been successfully decrypted!".to_string())).unwrap();
            }
        }
        _ => {}
    }
}