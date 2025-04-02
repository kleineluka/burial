// imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Window;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::config::{cache, downloads};
use crate::modmaking::converter;
use crate::utils::frontend::emitter;
use crate::utils::frontend::emitter::EventEmitter;
use crate::utils::helpers::{compression, connection, files};

use super::standalone;

// constant endpoints for GameBanana
const IMAGE_ENDPOINT: &str = "https://images.gamebanana.com/img/ss/mods/";
const MOD_INFO_ENDPOINT: &str = "https://api.gamebanana.com/Core/Item/Data?itemtype=Mod&itemid=<MOD_ID>&fields=Game%28%29.name%2Cname%2COwner%28%29.name%2Ctext%2CFiles%28%29.aFiles%28%29%2Cscreenshots%2CUpdates%28%29.aGetLatestUpdates%28%29";
//const TCOAAL_GAME_ID: &str = "18762";

// gamebanana data structures (adapted from https://github.com/MadMax1960/Concursus)
#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub filename: String,
    pub download_link: String,
    pub description: String,
    pub md5: String,
    pub filesize: usize,
}

#[derive(Debug)]
pub struct GamebananaMod {
    pub mod_id: Option<String>,
    pub name: String,
    pub submitter: String,
    pub files: HashMap<String, Files>,
    pub description: String,
    pub version: String,
    pub images: Vec<String>,
    pub mod_dir_path: PathBuf,
    pub game_folder_data_name: String,
    pub game_name: String,
}

impl GamebananaMod {

    // default constructor
    pub fn new() -> Self {
        Self {
            mod_id: None,
            name: String::new(),
            submitter: String::new(),
            files: HashMap::new(),
            description: String::new(),
            version: String::from("???"),
            images: Vec::new(),
            mod_dir_path: PathBuf::new(),
            game_folder_data_name: String::new(),
            game_name: String::new(),
        }
    }

    // parse a mod's info from a gamebanana json
    pub fn parse_json(text: &str) -> Option<Self> {
        // parse mod info
        let json_obj: Value = serde_json::from_str(text).ok()?;
        let mut mod_instance = GamebananaMod::new();
        mod_instance.name = json_obj[1].as_str()?.to_string();
        mod_instance.submitter = json_obj[2].as_str()?.to_string();
        mod_instance.description = json_obj[3].as_str()?.to_string();
        // parse files dictionary
        if let Some(files_obj) = json_obj[4].as_object() {
            for (key, value) in files_obj {
                let file = Files {
                    filename: value["_sFile"].as_str()?.to_string(),
                    download_link: value["_sDownloadUrl"].as_str()?.to_string(),
                    description: value["_sDescription"].as_str()?.to_string(),
                    md5: value["_sMd5Checksum"].as_str()?.to_string(),
                    filesize: value["_nFilesize"].as_u64()? as usize,
                };
                mod_instance.files.insert(key.clone(), file);
            }
        }
        // parse the images list
        if let Some(images_arr) = json_obj[5].as_array() {
            for image in images_arr {
                if let Some(file) = image["_sFile"].as_str() {
                    mod_instance.images.push(format!("{}{}", IMAGE_ENDPOINT, file));
                }
            }
        }
        // extract version from latest update
        if let Some(updates_arr) = json_obj[6].as_array() {
            if let Some(latest_update) = updates_arr.get(0) {
                mod_instance.version = latest_update["_sVersion"].as_str().unwrap_or("???").to_string();
            }
        }
        // extract the game name (we are only supporting tcoaal, anyways)
        mod_instance.game_name = json_obj[0].as_str()?.to_string();
        Some(mod_instance)
    }

    // fill in the mod's information via id
    pub async fn get_mod_info(mod_id: &str) -> Option<Self> {
        let url = MOD_INFO_ENDPOINT.replace("<MOD_ID>", mod_id);
        let response = reqwest::get(&url).await.ok()?; 
        let json = response.text().await.ok()?; 
        let mut mod_instance = Self::parse_json(&json)?; 
        mod_instance.mod_id = Some(mod_id.to_string());
        Some(mod_instance)
    }

    // fill in mod info from a url rather than id (extract id from https://gamebanana.com/mods/553302)
    pub async fn extract_mod_url(mod_url: String) -> Option<Self> {
        let mod_id = mod_url.split("/").last()?;
        Self::get_mod_info(mod_id).await
    }

    // return the download link (latest in the files list)
    pub async fn get_download_link(&self) -> Option<(String, String)> {
        let file = self.files.values().next()?;
        let file_id = self.files.keys().next()?;
        Some((file_id.clone(), file.download_link.clone()))
    }

    // download a gamebanana mod
    pub async fn download_mod(window: Option<&Window>, in_path: String, mod_url: String, mod_json: Option<converter::ModJson>) -> String {
        // now status updates are optional!
        let emitter = EventEmitter::new(window);
        emitter.emit("status", "Downloading the mod! Please wait, this may take a moment..");
        if let Some(mod_instance) = GamebananaMod::extract_mod_url(mod_url).await {
            // get the first download link, and, well, download it..
            if let Some((file_id, download_link)) = mod_instance.get_download_link().await {
                // download the file
                let sanitized_name = standalone::sanitize_mod_folder_name(&mod_instance.name);
                let download_path = downloads::downloads_folder().join(&sanitized_name);
                fs::create_dir_all(&download_path).unwrap();
                let _download_result = connection::download_file(&download_link, &download_path.to_str().unwrap().to_string()).await;
                // extract it from the archive
                emitter.emit("status", "Extracting the mod into the game directory..");
                let archive_path = download_path.join(file_id);
                let extraction_path = downloads::downloads_folder().join(format!("{}_extracted", &sanitized_name));
                files::validate_path(extraction_path.to_str().unwrap());
                let _extraction_result = compression::decompress_archive(&archive_path, &extraction_path, false);
                // delete the archive path
                emitter.emit("status", "Cleaning up..");
                if let Err(e) = fs::remove_file(&archive_path) {
                    eprintln!("Failed to remove file: {}", e);
                }
                // and then we an install as a regular standalone mod
                let standalone_installation = standalone::install_generic(window, in_path, extraction_path.to_str().unwrap().to_string(), mod_json);
                cache::clear_temp();
                emitter.emit("status", "Mod installed!");
                return standalone_installation;
            }
        }
        "unsuccesful".to_string()
    }

}