// imports
use tauri::command;
use serde::{Deserialize, Serialize};
use tokio::time::{self, Duration};
use reqwest::Error;
use crate::config::app;
use super::settings::Settings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub version: String,
    pub discord: String,
    pub github: String,
    pub website: String,
    pub itchio: String,
    pub gamebanana: String,
    pub nexusmods: String,
}

// construct a default metadata in the case of an error
impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            version: "0.0.0".to_string(),
            discord: "https://discord.gg/WWxAjJMspk".to_string(),
            github: "https://github.com/kleineluka/burial/".to_string(),
            website: "https://github.com/kleineluka/burial/".to_string(),
            itchio: "".to_string(),
            gamebanana: "".to_string(),
            nexusmods: "".to_string()
        }
    }
}

// load the metadata from the url
pub async fn get_metadata(app_config: &app::Config, user_settings: &Settings) -> Result<Metadata, Error> {
    // if the user doesn't want to fetch metadata, return the default (RESPECT PRIVACY!!!!!!!!!!!!)
    if !user_settings.updates {
        return Ok(Metadata::default());
    }
    // since we are going to be blocking, it's a safe bet to set a timeout
    let timeout_duration = Duration::from_secs(app_config.metadata_timeout);
    match time::timeout(timeout_duration, reqwest::get(&app_config.metadata_server)).await {
        // no timeout, try to parse the response
        Ok(Ok(response)) => {
            match response.json::<Metadata>().await {
                Ok(metadata) => Ok(metadata), // yipee !
                Err(_) => Ok(Metadata::default()), // parsing failure
            }
        }
        // timeout, or error
        Ok(Err(_)) | Err(_) => Ok(Metadata::default()),
    }
}

// get the (local) version of the application
#[command]
pub fn get_local_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}