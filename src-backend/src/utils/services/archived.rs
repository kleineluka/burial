// imports
use serde::Deserialize;
use serde::Serialize;
use tauri::Window;
use std::fs;
use crate::config::cache;
use crate::config::downloads;
use crate::modmaking::converter;
use crate::utils::helpers::compression;
use crate::utils::helpers::connection;
use crate::utils::helpers::files;
use crate::utils::services::standalone;
use crate::utils::frontend::emitter::EventEmitter;

// shell just for an easy wrapper
#[derive(Serialize, Deserialize, Debug)]
pub struct ArchivedMod {
    pub mod_url: String,
}

impl ArchivedMod {

    // download direct .zip or .rar files
    pub async fn install_mod(window: Option<&Window>, in_path: String, mod_url: String, mod_json: Option<converter::ModJson>) -> String {
        // now, statuses are optional!
        let emitter = EventEmitter::new(window);
        // download the file
        emitter.emit("status", "Downloading mod.. this may take a moment!");
        let download_path = downloads::downloads_folder().join("archived_mod");
        fs::create_dir_all(&download_path).unwrap();
        let _download_result = connection::download_file(&mod_url, &download_path.to_str().unwrap().to_string()).await;
        // extract it from the archive
        emitter.emit("status", "Extracting mod..");
        let downloaded_file = fs::read_dir(&download_path).unwrap().next().unwrap().unwrap().path();
        let extraction_path = downloads::downloads_folder().join("archived_mod_extracted");
        files::validate_path(extraction_path.to_str().unwrap());
        let _extraction_result = compression::decompress_archive(&downloaded_file, &extraction_path, false);
        // delete the archive path
        emitter.emit("status", "Cleaning up...");
        if let Err(e) = fs::remove_file(&download_path) {
            eprintln!("Failed to remove file: {}", e); // shouldn't
        }
        // and then we an install as a regular standalone mod
        emitter.emit("status", "Installing mod..");
        let standalone_installation = standalone::install_generic(None, in_path, extraction_path.to_str().unwrap().to_string(), mod_json);
        cache::clear_temp();
        emitter.emit("status", "Done!");
        standalone_installation
    }

}