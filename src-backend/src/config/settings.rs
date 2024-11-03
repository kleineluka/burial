use std::fs;
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::config::cache;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub tcoaal: String,
    pub output: String,
    pub hotload: bool,
    pub theme: String,
}

pub fn check_settings() {
    // cache_dir + settings.json
    let cache_dir = cache::cache_folder();
    let file_path = cache_dir.join("settings.json");
    if !file_path.exists() {
        // create the default config with default values
        let default_settings = Settings {
            tcoaal: "".to_string(),
            output: "".to_string(),
            hotload: false,
            theme: "ashley".to_string(),
        };
        // serialize the config to a JSON string
        let json_data = serde_json::to_string_pretty(&default_settings)
            .expect("Failed to serialize default config");
        // create and write to the file
        let mut file = fs::File::create(file_path)
            .expect("Failed to create file");
        file.write_all(json_data.as_bytes())
            .expect("Failed to write data to file");
    }
}

pub fn read_settings() -> Settings {
    // cache_dir + settings.json
    let cache_dir = cache::cache_folder();
    let file_path = cache_dir.join("settings.json");
    // if it doesn't exist, make default settings first
    if !file_path.exists() {
        check_settings();
    }
    // read the file
    let file = fs::File::open(file_path)
        .expect("Failed to open file");
    let settings: Settings = serde_json::from_reader(file)
        .expect("Failed to read file");
    settings
}

pub fn write_settings(settings: Settings) {
    // cache_dir + settings.json
    let cache_dir = cache::cache_folder();
    let file_path = cache_dir.join("settings.json");
    // serialize the config to a JSON string
    let json_data = serde_json::to_string_pretty(&settings)
        .expect("Failed to serialize config");
    // create and write to the file
    let mut file = fs::File::create(file_path)
        .expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write data to file");
}

pub fn delete_settings() {
    // cache_dir + settings.json
    let cache_dir = cache::cache_folder();
    let file_path = cache_dir.join("settings.json");
    // delete the file
    fs::remove_file(file_path)
        .expect("Failed to delete file");
}

pub fn first_run() -> bool {
    // cache_dir + setup.lock
    let cache_dir = cache::cache_folder();
    let file_path = cache_dir.join(".setup.lock");
    !file_path.exists()
}