// imports
use tauri::Window;
use serde::{Deserialize, Serialize};
use crate::modmaking::converter;
use crate::utils::emitter::EventEmitter;
use crate::utils::files;
use crate::utils::connection;
use crate::utils::compression;
use crate::config::downloads;
use crate::utils::game;
use crate::utils::services::standalone;

//mod_json.name, mod_json.id, mod_json.authors, mod_json.description, mod_json.version
#[derive(Serialize, Deserialize, Debug)]
pub struct GithubMod {
    pub mod_url: String,
    pub mod_name: String,
    pub mod_id: String,
    pub mod_authors: Vec<String>,
    pub mod_description: String,
    pub mod_version: String,
}

impl GithubMod {

    pub fn get_download(url: &str) -> Result<String, String> {
        // make sure we are downloading the github as a zip
        if url.ends_with("/archive/refs/heads/main.zip") {
            return Ok(url.to_string());
        }
        // parse the input URL and check its validity
        let base_url = url.trim_end_matches('/'); 
        if !base_url.starts_with("https://github.com/") {
            return Err("Invalid GitHub URL.".to_string());
        }
        // construct the download url
        let download_url = format!("{}/archive/refs/heads/main.zip", base_url);
        Ok(download_url)
    }

    // download a github mod, expecting "www" format (assume non-tomb)
    pub async fn download_mod(window: Option<&Window>, in_path: String, github_mod: GithubMod) -> String {
        // now status updates are optional!
        let emitter = EventEmitter::new(window);
        // format the url
        emitter.emit("status", "Preparing to download the mod..");
        let mod_url = GithubMod::get_download(&github_mod.mod_url).unwrap();
        // download the mod!
        emitter.emit("status", "Downloading the mod! Please wait, this may take a moment..");
        let sanitized_name = standalone::sanitize_mod_folder_name(&github_mod.mod_name);
        let temp_path = downloads::downloads_folder().join(&sanitized_name);
        if let Err(e) = connection::download_file(&mod_url, &temp_path.to_string_lossy()).await {
            emitter.emit("error", "There was a problem downloading the mod..");
            return "connection_timeout".to_string();
        }
        // open the file
        let mod_file = match temp_path.read_dir().unwrap().next() {
                Some(f) => f.unwrap().path(),
                None => {
                    emitter.emit("error", "There was a problem opening the mod file..");
                    return "error_file_open".to_string();
            }
        };
        // extract that mod into a new folder in the same directory as the mod_file called "non_tomb"
        emitter.emit("status", "Extracting the mod..");
        let mod_folder = temp_path.join("non_tomb");
        compression::decompress_zip_nosub(&mod_file, &mod_folder).unwrap();
        // and convert it
        emitter.emit("status", "Compiling the mod..");
        let tomb_mod_folder = temp_path.join("tomb");
        let converted_mod = converter::convert_to_tomb(mod_folder.to_str().unwrap().to_string(), in_path.clone(), tomb_mod_folder.to_str().unwrap().to_string(), github_mod.mod_name.clone(), github_mod.mod_id, github_mod.mod_authors, github_mod.mod_description, github_mod.mod_version);
        if converted_mod == "error:game_path" || converted_mod == "error:mod_path" {
            return converted_mod.to_string().replace("error:", "error_");
        }
        // copy the converted mod to the game folder
        emitter.emit("status", "Installing the mod..");
        let mod_folder = match tomb_mod_folder.read_dir().unwrap().next() {
            Some(f) => f.unwrap().path(),
            None => {
                return "error_copy".to_string();
            }
        };
        // move it to the game folder
        let game_path = std::path::Path::new(&in_path);
        let mod_path = game_path.join("tomb").join("mods");
        let sanitized_name = standalone::sanitize_mod_folder_name(github_mod.mod_name.as_str());
        std::fs::rename(&mod_folder, &mod_path.join(sanitized_name)).unwrap();
        // clear the temp folder n done!
        emitter.emit("status", "Cleaning up..");
        downloads::clear_downloads().unwrap();
        emitter.emit("status", "Mod installed!");
        return "success".to_string();
    } 

}