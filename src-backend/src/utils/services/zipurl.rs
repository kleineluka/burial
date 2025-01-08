// imports
use serde::Deserialize;
use serde::Serialize;
use tauri::Window;
use std::fs;
use crate::config::cache;
use crate::config::downloads;
use crate::utils::compression;
use crate::utils::connection;
use crate::utils::files;
use crate::utils::services::standalone;
use crate::utils::emitter::EventEmitter;

// shell just for an easy wrapper
#[derive(Serialize, Deserialize, Debug)]
pub struct ZipUrl {
    pub mod_url: String,
}

impl ZipUrl {

    // download direct .zip files
    pub async fn install_mod(window: Option<&Window>, in_path: String, mod_url: String) -> String {
        // now, statuses are optional!
        let emitter = EventEmitter::new(window);
        // download the file
        emitter.emit("status", "Downloading mod.. this may take a moment!");
        let download_path = downloads::downloads_folder().join("zip_mod");
        fs::create_dir_all(&download_path).unwrap();
        let _download_result = connection::download_file(&mod_url, &download_path.to_str().unwrap().to_string()).await;
        // extract it from the archive
        emitter.emit("status", "Extracting mod..");
        let extraction_path = downloads::downloads_folder().join("zip_mod_extracted");
        files::validate_path(extraction_path.to_str().unwrap());
        let _extraction_result = compression::decompress_archive(&download_path, &extraction_path, false);
        // delete the archive path
        emitter.emit("status", "Cleaning up...");
        if let Err(e) = fs::remove_file(&download_path) {
            eprintln!("Failed to remove file: {}", e); // shouldn't
        }
        // and then we an install as a regular standalone mod
        emitter.emit("status", "Installing mod..");
        let standalone_installation = standalone::install_generic(None, in_path, extraction_path.to_str().unwrap().to_string());
        cache::clear_temp();
        emitter.emit("status", "Done!");
        standalone_installation
    }

}