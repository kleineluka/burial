// imports
use tauri::Window;
use tauri::command;
use serde::{Deserialize, Serialize};
use tokio::time::{self, Duration};
use reqwest::Error;

// metadata url + (json) structure
const METADATA_URL: &str = "https://raw.githubusercontent.com/kleineluka/burial/refs/heads/main/api/metadata.json";
const TIMEOUT_DURATION: u64 = 10;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub version: String,
    pub discord: String,
    pub github: String,
    pub website: String,
}

// construct a default metadata in the case of an error
impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            version: "0.0.0".to_string(),
            discord: "https://discord.gg/WWxAjJMspk".to_string(),
            github: "https://github.com/kleineluka/burial/".to_string(),
            website: "https://github.com/kleineluka/burial/".to_string(),
        }
    }
}

// load the metadata from the url
pub async fn get_metadata() -> Result<Metadata, Error> {
    // since we are going to be blocking, it's a safe bet to set a timeout
    let timeout_duration = Duration::from_secs(TIMEOUT_DURATION);
    match time::timeout(timeout_duration, reqwest::get(METADATA_URL)).await {
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