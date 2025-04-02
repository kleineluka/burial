// imports
use std::path::Path;
use std::path::PathBuf;
use tauri::Window;
use tauri::command;
use walkdir::WalkDir;
use crate::utils;
use crate::utils::operating::game;
use crate::utils::helpers::files;
use crate::utils::nemlei::cipher;

// first rule: recursively see what paths to traverse (all paths in rule_paths, or if all, then all paths recursively)
fn enforce_paths(in_path: &Path, rule_paths: Vec<String>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if rule_paths.contains(&"all".to_string()) {
        for entry in WalkDir::new(in_path) {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                paths.push(entry.path().to_path_buf());
            }
        }
    } else {
        for rule_path in rule_paths {
            let path = in_path.join(rule_path);
            if path.exists() {
                paths.push(path);
            }
        }
    }
    paths
}

// second rule: get all files in the folders being used
fn enforce_files(paths: Vec<PathBuf>, rule_files: Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if rule_files.contains(&"all".to_string()) {
        for path in paths {
            for entry in WalkDir::new(&path) {
                let entry = entry.unwrap();
                if entry.path().is_file() {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    } else {
        for path in paths {
            for rule_file in &rule_files {
                let file = path.join(rule_file);
                if file.exists() {
                    files.push(file);
                }
            }
        }
    }
    files
}

// third rule: get all files with the prefixes being used
fn enforce_prefixes(files: Vec<PathBuf>, rule_prefixes: Vec<String>) -> Vec<PathBuf> {
    let mut prefixes = Vec::new();
    if rule_prefixes.contains(&"all".to_string()) {
        for file in files {
            prefixes.push(file);
        }
    } else {
        for file in files {
            if let Some(file_name) = file.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if rule_prefixes.iter().any(|rule_prefix| file_name_str.starts_with(rule_prefix)) {
                    prefixes.push(file);
                }
            }
        }
    }
    prefixes
}

// fourth rule: get all files with the extensions being used
fn enforce_extensions(files: Vec<PathBuf>, rule_extensions: Vec<String>) -> Vec<PathBuf> {
    let mut extensions = Vec::new();
    if rule_extensions.contains(&"all".to_string()) {
        for file in files {
            extensions.push(file);
        }
    } else {
        for file in files {
            if let Some(extension) = file.extension() {
                let extension_str = extension.to_string_lossy();
                if rule_extensions.iter().any(|rule_extension| extension_str == rule_extension.as_str()) {
                    extensions.push(file);
                }
            }
        }
    }
    extensions
}

// sift through resources
#[command]
pub fn export_resources(window: Window, in_path: String, out_path: String, 
    rule_paths: Vec<String>, rule_files: Vec<String>, rule_prefixes: Vec<String>, rule_extensions: Vec<String>) {
    // sanity checks..
    window.emit("status", "Setting up..").unwrap();
    let in_path = Path::new(&in_path);
    let is_game = game::verify_game(&in_path.to_string_lossy()).unwrap();
    if !in_path.exists() || !is_game {
        window.emit("error", Some("That is not a valid TCOAAL folder!".to_string())).unwrap();
        return;
    }
    // make sure the out path exists via PathBuf
    let out_path = Path::new(&out_path).to_path_buf();
    utils::helpers::files::verify_folder(&out_path).unwrap();
    // filter resources through all the rules
    window.emit("status", Some("Filtering assets..".to_string())).unwrap();
    let paths = enforce_paths(in_path, rule_paths);
    let files = enforce_files(paths, rule_files);
    let prefixes = enforce_prefixes(files, rule_prefixes);
    let resources = enforce_extensions(prefixes, rule_extensions);
    // go through the resources and copy them to the out path
    window.emit("status", Some("Copying assets..".to_string())).unwrap();
    for resource in resources {
        let out_resource = Path::new(&out_path).join(resource.file_name().unwrap());
        files::copy_file(&resource.to_string_lossy(), &out_resource.to_string_lossy());
    }
    // go through the copied directory and decrypt any k9a files (and delete the original copy)
    window.emit("status", Some("Decrypting files..".to_string())).unwrap();
    for entry in WalkDir::new(&out_path) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            let entry_path = entry.path().to_string_lossy();
            if entry_path.ends_with(".k9a") {
                let (decrypted_data, file_name_with_extension) = cipher::decrypt_file(&entry_path);
                let out_path = Path::new(&out_path).join(file_name_with_extension);
                files::write_file(&out_path.to_string_lossy(), &decrypted_data);
                files::delete_file(&entry_path);
            }
        }
    }
    window.emit("status", "Desired assets have been exported!").unwrap();
}