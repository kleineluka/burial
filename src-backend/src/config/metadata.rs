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
    pub repo_server: String,
    pub news_server: String
}

// construct a default metadata in the case of an error
impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            version: "0.0.0".to_string(),
            discord: "https://discord.gg/WWxAjJMspk".to_string(),
            github: "https://github.com/kleineluka/burial/".to_string(),
            website: "https://github.com/kleineluka/burial/".to_string(),
            itchio: "https://www.luka.moe/go/burial_itch".to_string(),
            gamebanana: "https://www.luka.moe/go/burial_gamebanana".to_string(),
            nexusmods: "https://www.luka.moe/go/burial_nexusmods".to_string(),
            repo_server: "https://llamawa.re/".to_string(),
            news_server: "https://codeberg.org/peachy/visions/raw/branch/main/mods.json".to_string()
        }
    }
}

// load the metadata from the url
pub async fn get_metadata(app_config: &app::Config, user_settings: &Settings, user_hash: &String) -> Result<Metadata, Error> {
    // if the user doesn't want to fetch metadata, return the default (RESPECT PRIVACY!!!!!!!!!!!!)
    if !user_settings.updates {
        return Ok(Metadata::default());
    }
    // set the timeout duration
    let timeout_duration = Duration::from_secs(app_config.metadata_timeout);
    let metadata_url = format!("{}metadata.json", app_config.api_server);
    let client = reqwest::Client::new();
    let request = client
        .get(&metadata_url)
        .header("hwid", user_hash)
        .header("appver", env!("CARGO_PKG_VERSION"))
        .build()
        .expect("Failed to build request");
    // try to fetch the metadata
    match time::timeout(timeout_duration, client.execute(request)).await {
        // no timeout, try to parse the response 
        Ok(Ok(response)) => {
            match response.json::<Metadata>().await {
                Ok(metadata) => Ok(metadata), // yipee !
                Err(_) => Ok(Metadata::default()), // parsing failure
            }
        }
        // return the default metadata on timeout
        _ => Ok(Metadata::default()),
    }
}

// get the (local) version of the application
#[command]
pub fn get_local_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}