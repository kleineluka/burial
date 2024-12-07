// imports
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;
use walkdir::WalkDir;

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
    let _ = fs::remove_file(file_path);
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

// rename file
pub fn rename_file(from: &str, to: &str) {
    fs::rename(from, to).unwrap();
}

// true/false if file exists
pub fn file_exists(file_path: &str) -> bool {
    PathBuf::from(file_path).exists()
}

// open os file explorer at a given path
pub fn open_folder(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "The specified path does not exist.",
        ));
    }
    #[cfg(target_os = "windows")]
    Command::new("explorer").arg(path).spawn()?;
    #[cfg(target_os = "macos")]
    Command::new("open").arg(path).spawn()?;
    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(path).spawn()?;
    Ok(())
}


// delete all files in a folder
pub fn clear_folder(folder_path: &str) {
    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            delete_file(&path.to_string_lossy());
        }
    }
}

// create a folder if it does not already exist
pub fn verify_folder(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        // Create the directory if it doesn't exist
        fs::create_dir_all(path)?;
    } 
    Ok(())
}

// keep making a folder until it exists (ex. if folder_name is taken, add a number to the end, return the final path)
pub fn verify_folder_multiple(base_path: &String) -> String {
    let mut path = PathBuf::from(base_path);
    let mut i = 1;
    while path.exists() {
        path = PathBuf::from(format!("{}{}", base_path, i));
        i += 1;
    }
    fs::create_dir_all(&path).unwrap();
    path.to_string_lossy().to_string()
}

// get all files recursively in a directory
pub fn collect_files_recursive(dir: String) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok()) // filter out errors
        .filter(|entry| entry.path().is_file()) //only files
        .map(|entry| entry.into_path()) // convert entries into PathBuf
        .collect() // to vec of pathbuf
}

// backup file (make a copy and append .bak)
pub fn backup_file(file_path: &str) {
    let backup_path = format!("{}.bak", file_path);
    copy_file(file_path, &backup_path);
}

// backup file (keep going until a unique name is found)
pub fn backup_file_multiple(file_path: &str) {
    let mut backup_path = format!("{}.bak", file_path);
    // if its taken, add a number to the end
    let mut i = 1;
    while PathBuf::from(&backup_path).exists() {
        backup_path = format!("{}.bak{}", file_path, i);
        i += 1;
    }
    copy_file(file_path, &backup_path);
}

// find the latest backup if multiple were made
pub fn newest_backup(file_path: &str) -> Option<PathBuf> {
    let file_parent = Path::new(file_path).parent().unwrap_or_else(|| Path::new("."));
    println!("File parent: {:?}", file_parent);
    // regex: capture .bak and .bak<number>
    let backup_regex = Regex::new(r"(\.bak)(\d+)?").unwrap();
    // Go through all files in directory
    let mut newest_backup: Option<(PathBuf, u32)> = None;
    for entry in fs::read_dir(file_parent).unwrap_or_else(|e| {
        eprintln!("Failed to read directory '{}': {}", file_parent.display(), e);
        return fs::read_dir(".").unwrap(); // fallback to current directory to prevent panic
    }) {
        // get entry
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read directory entry: {}", e);
                continue;
            }
        };
        // convert file name to string
        let file_name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => {
                eprintln!("Failed to convert file name to string for entry: {:?}", entry.path());
                continue;
            }
        };
        // check if the file name matches the backup pattern
        if let Some(captures) = backup_regex.captures(&file_name) {
            let backup_number = captures.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
            if let Some((_, number)) = newest_backup {
                if backup_number > number {
                    newest_backup = Some((entry.path(), backup_number));
                }
            } else {
                newest_backup = Some((entry.path(), backup_number));
            }
        }
    }
    newest_backup.map(|(path, _)| path)
}

// restore the newest backup to be the original (but, first, backup the original)
pub fn restore_file_multiple(file_path: &str) {
    println!("Restoring file: {}", file_path);
    let newest_backup = newest_backup(file_path).unwrap();
    backup_file(file_path);
    rename_file(&newest_backup.to_string_lossy(), file_path);
}

// restore a backup (take a file, remove it, and rename the .bak file to the original name)
pub fn restore_backup(file_path: &str) {
    let backup_path = format!("{}.bak", file_path);
    delete_file(file_path);
    rename_file(&backup_path, file_path);
}

// recursively copy all files in a directory to another directory (and match file structure)
pub fn copy_directory(src_dir: &str, dest_dir: &str) -> std::io::Result<()> {
    // make directory if it doesn't exist
    fs::create_dir_all(dest_dir)?;
    // copy all files in the source directory to the destination directory
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let dest_path = format!("{}/{}", dest_dir, file_name);
        if path.is_dir() {
            copy_directory(&path.to_string_lossy(), &dest_path)?;
        } else {
            copy_file(&path.to_string_lossy(), &dest_path);
        }
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

pub fn find_output() -> Option<PathBuf> {
    // find documents
    let documents_dir = match dirs::document_dir() {
        Some(dir) => dir,
        None => {
            return None;
        }
    };
    // make burial folder in it
    let burial_dir = documents_dir.join("burial");
    if !burial_dir.exists() {
        if let Err(e) = fs::create_dir_all(&burial_dir) {
            eprintln!("Failed to create burial directory: {}", e);
            return None;
        }
    }
    // return the burial folder
    Some(burial_dir)
}

// ex. www/data/actors.json + data/actors.json -> data/actors.json
pub fn relative_path(in_path: &String, file_path: &String) -> String {
    file_path.split(in_path).collect::<Vec<&str>>()[1].to_string()
}

// ensure all folders leading up to a file exist
pub fn validate_path(file_path: &str) {
    let path = Path::new(file_path).parent().unwrap().to_str().unwrap();
    fs::create_dir_all(path).unwrap();
}