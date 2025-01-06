use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::config::cache;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModPack {
    pub lastUpdate: String,
    pub modloaderVersion: String,
    pub mods: Vec<String>,
    pub icon: Option<String>,  
    pub description: String,
}

// write the current modpack to the file
pub fn write_modpack(modpack_name: String) {
    // get the cache folder
    let cache_dir = cache::cache_folder();
    // create the file path
    let file_path = cache_dir.join(".modpack.lock");
    // write the modpack name to the file
    std::fs::write(file_path, modpack_name)
        .expect("Failed to write modpack to file");
}

// get the user's current modpack
pub fn read_modpack() -> String {
    // see if the file exists, if not, make the default current modpack
    let cache_dir = cache::cache_folder();
    let file_path = cache_dir.join(".modpack.lock");
    if !file_path.exists() {
        write_modpack("none".to_string());
    }
    // read the modpack from the file
    let modpack = std::fs::read_to_string(file_path)
        .expect("Failed to read modpack from file");
    return modpack;
}