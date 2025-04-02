use std::fs;
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use crate::config::cache;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Settings {
    pub tcoaal: String,
    pub output: String,
    pub updates: bool,
    pub theme: String,
    pub animations: bool,
    pub tooltips: bool,
    pub modname: String,
    pub modid: String,
    pub modauthor: String,
    pub moddescription: String,
    pub deeplinks: bool,
    pub gametarget: String,
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
            updates: true,
            theme: "ashley".to_string(),
            animations: true,
            tooltips: true,
            modname: "".to_string(),
            modid: "".to_string(),
            modauthor: "".to_string(),
            moddescription: "".to_string(),
            deeplinks: true,
            gametarget: "latest".to_string(),
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

fn merge_settings(base: Value, other: Value) -> Value {
    match (base, other) {
        (Value::Object(mut base_map), Value::Object(other_map)) => {
            for (key, value) in other_map {
                let base_value: &mut Value = base_map.entry(key.clone()).or_insert(Value::Null);
                *base_value = merge_settings(base_value.take(), value);
            }
            Value::Object(base_map)
        }
        (_, other) => other,
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
    let file = fs::File::open(&file_path).expect("Failed to open file");
    let existing_data: Value = serde_json::from_reader(&file)
        .unwrap_or_else(|_| Value::Null);
    let default_settings = serde_json::to_value(Settings::default())
        .expect("Failed to serialize default settings");
    let merged_settings = merge_settings(default_settings, existing_data.clone());
    let settings: Settings = serde_json::from_value(merged_settings.clone())
        .expect("Failed to deserialize merged settings");
    if existing_data != merged_settings {
        write_settings(settings.clone());
    }
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