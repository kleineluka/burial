// imports
use serde::Deserialize;
use serde::Serialize;
use tauri::Window;
use std::fs::File;
use std::io::{BufReader, Read};
use sha2::{Sha256, Digest};
use crate::config::downloads;
use crate::utils::helpers::compression;
use crate::utils::helpers::connection;
use crate::utils::services::standalone;
use crate::utils::frontend::emitter::EventEmitter;

// shell just for an easy wrapper
#[derive(Serialize, Deserialize, Debug)]
pub struct LlamawareMod {
    pub mod_url: String,
}

impl LlamawareMod {

    // for mods in the llamaware repository (not hosted on llamaware itself!)
    pub async fn install_mod(window: Option<&Window>, in_path: String, mod_path: String, mod_hash: String) -> String {
        // now status updates are optional!
        let emitter = EventEmitter::new(window);
        // start downloading the mod
        emitter.emit("status", "Downloading the mod! Please wait, this may take a moment..");
        let downloads = downloads::downloads_folder();
        if let Err(e) = connection::download_file(&mod_path, &downloads.to_string_lossy()).await {
            emitter.emit("mod-install", "error_connection");
            return "connection_timeout".to_string();
        }
        // get the name of the zip file and then open it
        emitter.emit("status", "Verifying the hash of the mod..");
        let mod_name = mod_path.split('/').last().unwrap();
        let mod_file = downloads.join(mod_name);
        let file = match File::open(&mod_file) {
            Ok(f) => f,
            Err(_) => {
                emitter.emit("mod-install", "error_file_open");
                return "file_open_error".to_string();
            }
        };
        // verify the hash of the newly downloaded zip
        let mut hasher = Sha256::new();
        let mut buf_reader = BufReader::new(file);
        let mut buffer = vec![0; 8192];
        while let Ok(read_bytes) = buf_reader.read(&mut buffer) {
            if read_bytes == 0 {
                break;
            }
            hasher.update(&buffer[..read_bytes]);
        }
        let computed_hash = format!("{:x}", hasher.finalize());
        if computed_hash != mod_hash {
            emitter.emit("mod-install", "error_hash_mismatch");
            return "hash_mismatch".to_string();
        }
        // extract the mod contents into the game folder/tomb/mods/<mod name>
        emitter.emit("status", "Extracting the mod into the game directory..");
        let sanitized_name = standalone::sanitize_mod_folder_name(mod_name);
        let mod_folder = format!("{}/tomb/mods/{}", in_path.clone(), sanitized_name);
        let mod_folder_path = std::path::Path::new(&mod_folder);
        // if the mod folder already exists, delete it
        if mod_folder_path.exists() {
            emitter.emit("status", "Removing the previous installation of the mod..");
            std::fs::remove_dir_all(&mod_folder).unwrap();
        }
        compression::decompress_zip(&mod_file, &mod_folder_path).unwrap();
        // delete the downloads folder
        emitter.emit("status", "Cleaning up..");
        std::fs::remove_file(&mod_file).unwrap();
        downloads::clear_downloads().unwrap();
        // all done!
        emitter.emit("status", "Mod installed successfully!");
        "success".to_string()
    }

}
