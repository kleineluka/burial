// imports
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{PathBuf};
use sha2::{Digest, Sha256};
use sysinfo::Disks;

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

// write a file to a given path
pub fn write_file(file_path: &str, data: &[u8]) {
    // if already exists, delete the file
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