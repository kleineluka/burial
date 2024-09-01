// imports
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use tauri::api::dialog::FileDialogBuilder;

// get the file name (without extension) from a file path
pub fn file_name(file_path: &str) -> String {
    PathBuf::from(file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// get the file extension (without the dot) from a file path
pub fn file_extension(file_path: &str) -> String {
    PathBuf::from(file_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// delete a file from a given path
pub fn delete_file(file_path: &str) {
    fs::remove_file(file_path).unwrap();
}

// read a file from a given path
pub fn read_file(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

// write a file to a given path (will overwrite files..)
pub fn write_file(file_path: &str, data: &[u8]) {
    if PathBuf::from(file_path).exists() {
        delete_file(file_path);
    }
    let mut file = File::create(file_path).unwrap();
    file.write_all(data).unwrap();
}

// create a path if it doesn't exist
pub fn create_path(path: &str) {
    if !PathBuf::from(path).exists() {
        fs::create_dir_all(path).unwrap();
    }
}

// copy file from a to b
pub fn copy_file(from: &str, to: &str) {
    fs::copy(from, to).unwrap();
}

// copy folder from a to b
pub fn copy_folder(from: &str, to: &str) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            copy_folder(&path.to_string_lossy(), &format!("{}/{}", to, path.file_name().unwrap().to_string_lossy()));
        } else {
            copy_file(&path.to_string_lossy(), &format!("{}/{}", to, path.file_name().unwrap().to_string_lossy()));
        }
    }
}

// delete folder
pub fn delete_folder(folder_path: &str) {
    fs::remove_dir_all(folder_path).unwrap();
}

// create a folder if it does not already exist
pub fn verify_folder(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        // Create the directory if it doesn't exist
        fs::create_dir_all(path)?;
    } 
    Ok(())
}

// index directory recursively (return amount of .<x> files and what folders they are in)
pub fn index_directory_single<P: AsRef<Path>>(dir: P, extension: &str) -> (usize, Vec<String>) {
    // store for later..
    let mut file_count = 0;
    let mut folders_with_extension = HashSet::new();
    // recursive search function
    fn search_recursive(dir: &Path, extension: &str, file_count: &mut usize, folders_with_extension: &mut HashSet<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        search_recursive(&path, extension, file_count, folders_with_extension);
                    } else if let Some(ext) = path.extension() {
                        if ext == extension.trim_start_matches('.') {
                            *file_count += 1;
                            let mut current_dir = path.parent();
                            while let Some(parent) = current_dir {
                                folders_with_extension.insert(parent.to_path_buf());
                                current_dir = parent.parent();
                            }
                        }
                    }
                }
            }
        }
    }
    // start the search
    search_recursive(dir.as_ref(), extension, &mut file_count, &mut folders_with_extension);
    // convert the folders to strings
    let folder_names: Vec<String> = folders_with_extension
        .into_iter()
        .filter_map(|folder| folder.file_name().map(|name| name.to_string_lossy().into_owned()))
        .collect();
    (file_count, folder_names)
}

// count all files in a directory
pub fn index_directory_all<P: AsRef<Path>>(dir: P) -> (usize, Vec<String>) {
    // store for later..
    let mut file_count = 0;
    let mut folders_with_files = HashSet::new();
    // recursive search function
    fn search_recursive(dir: &Path, file_count: &mut usize, folders_with_files: &mut HashSet<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        // continue searching
                        search_recursive(&path, file_count, folders_with_files);
                    } else {
                        // file, so mark parents and add to count
                        *file_count += 1;
                        let mut current_dir = path.parent();
                        while let Some(parent) = current_dir {
                            folders_with_files.insert(parent.to_path_buf());
                            current_dir = parent.parent();
                        }
                    }
                }
            }
        }
    }
    // start the search
    search_recursive(dir.as_ref(), &mut file_count, &mut folders_with_files);
    // convert the folders to strings
    let folder_names: Vec<String> = folders_with_files
        .into_iter()
        .filter_map(|folder| folder.file_name().map(|name| name.to_string_lossy().into_owned()))
        .collect();
    (file_count, folder_names)
}