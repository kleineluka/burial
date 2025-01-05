// imports
use std::fs;
use serde::Deserialize;

// config structure + default values
#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_server: String,
    pub metadata_timeout: u64,
    pub mods_repository: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_server: "https://raw.githubusercontent.com/kleineluka/burial/refs/heads/main/api/".to_string(),
            metadata_timeout: 10,
            mods_repository: "test".to_string()
        }
    }
}

pub fn load_config() -> Config {
    // Try to load the configuration file, return default if it fails
    if let Ok(config_string) = fs::read_to_string("config.json") {
        if let Ok(config) = serde_json::from_str(&config_string) {
            return config;
        }
    }
    Config::default() // Return default configuration on error
}
