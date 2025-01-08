// imports
use tauri::Window;
use tauri::command;
use std::fs;
use crate::config::cache;
use crate::utils::files;
use crate::utils::services::gamebanana;
use crate::utils::services::standalone;
use crate::utils::connection;
use crate::utils::compression;
use crate::config::downloads;
use crate::utils::game;

// download a gamebanana mod
pub async fn download_gamebanana_mod(in_path: String, mod_url: String) -> String {
    if let Some(mod_instance) = gamebanana::GamebananaMod::extract_mod_url(mod_url).await {
        // get the first download link, and, well, download it..
        if let Some((file_id, download_link)) = mod_instance.get_download_link().await {
            // download the file
            let sanitized_name = standalone::sanitize_mod_folder_name(&mod_instance.name);
            let download_path = downloads::downloads_folder().join(&sanitized_name);
            fs::create_dir_all(&download_path).unwrap();
            let _download_result = connection::download_file(&download_link, &download_path.to_str().unwrap().to_string()).await;
            // extract it from the archive
            let archive_path = download_path.join(file_id);
            let extraction_path = downloads::downloads_folder().join(format!("{}_extracted", &sanitized_name));
            files::validate_path(extraction_path.to_str().unwrap());
            let _extraction_result = compression::decompress_archive(&archive_path, &extraction_path, false);
            // delete the archive path
            if let Err(e) = fs::remove_file(&archive_path) {
              eprintln!("Failed to remove file: {}", e);
            }
            // and then we an install as a regular standalone mod
            let standalone_installation = standalone::install_generic(None, in_path, extraction_path.to_str().unwrap().to_string());
            cache::clear_temp();
            return standalone_installation;
        }
    }
    "unsuccesful".to_string()
}

// download a zip mod
pub async fn download_zip_mod(in_path: String, mod_url: String) -> String {
    // download the file
    let download_path = downloads::downloads_folder().join("zip_mod");
    fs::create_dir_all(&download_path).unwrap();
    let _download_result = connection::download_file(&mod_url, &download_path.to_str().unwrap().to_string()).await;
    // extract it from the archive
    let extraction_path = downloads::downloads_folder().join("zip_mod_extracted");
    files::validate_path(extraction_path.to_str().unwrap());
    let _extraction_result = compression::decompress_archive(&download_path, &extraction_path, false);
    // delete the archive path
    if let Err(e) = fs::remove_file(&download_path) {
      eprintln!("Failed to remove file: {}", e);
    }
    // and then we an install as a regular standalone mod
    let standalone_installation = standalone::install_generic(None, in_path, extraction_path.to_str().unwrap().to_string());
    cache::clear_temp();
    standalone_installation
}

// download a mod from a url (determine which kind of mmod it is then act accordingly)
pub async fn download_mod_url(in_path: String, mod_url: String, mod_source: standalone::ModSource) -> String {
    if mod_source == standalone::ModSource::Gamebanana {
        return download_gamebanana_mod(in_path, mod_url).await;
    } else if mod_source == standalone::ModSource::ZipUrl {
        return download_zip_mod(in_path, mod_url).await;
    }
    "unsupported".to_string()
}

// command to download a mod
#[command]
pub async fn download_external_mod(window: Window, in_path: String, mod_url: String) {
    // verify that it is a game first
    if !game::verify_game(&in_path).unwrap() {
        window.emit("error", "Your TCOAAL installation is not valid. Please set it in the settings page!".to_string()).unwrap();
        return;
    }
    // get the mod source
    let mod_source = standalone::ModSource::from_url(&mod_url);
    window.emit("external-mod-source", mod_source.clone()).unwrap();
    let mod_downloaded = download_mod_url(in_path, mod_url, mod_source).await;
    window.emit("external-mod-downloaded", mod_downloaded).unwrap();
}