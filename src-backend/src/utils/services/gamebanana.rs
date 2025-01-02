// imports
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

// constant endpoints for GameBanana
const IMAGE_ENDPOINT: &str = "https://images.gamebanana.com/img/ss/mods/";
const MOD_INFO_ENDPOINT: &str = "https://api.gamebanana.com/Core/Item/Data?itemtype=Mod&itemid=<MOD_ID>&fields=Game%28%29.name%2Cname%2COwner%28%29.name%2Ctext%2CFiles%28%29.aFiles%28%29%2Cscreenshots%2CUpdates%28%29.aGetLatestUpdates%28%29";

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
    pub fn parse_json(game_id: &str, text: &str) -> Option<Self> {
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

    // return a sanitized name for the mod (for folder creation)
    pub fn get_sanitized_name(&self) -> String {
        let clean_name = self.name.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
        if clean_name.is_empty() {
            "unnamed_mod".to_string()
        } else {
            clean_name
        }
    }

    // fill in the mod's information via id
    pub async fn get_mod_info(game_id: &str, mod_id: &str) -> Option<Self> {
        let url = MOD_INFO_ENDPOINT.replace("<MOD_ID>", mod_id);
        let response = reqwest::get(&url).await.ok()?; 
        let json = response.text().await.ok()?; 
        let mut mod_instance = Self::parse_json(game_id, &json)?; 
        mod_instance.mod_id = Some(mod_id.to_string());
        Some(mod_instance)
    }

    // fill in mod info from a url rather than id (extract id from https://gamebanana.com/mods/553302)
    //pub async fn extract_mod_url(in)

}