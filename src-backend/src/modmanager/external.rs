// imports
use tauri::Window;
use tauri::command;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use crate::utils::files;
use crate::utils::services::gamebanana;
use crate::utils::services::standalone;
use crate::utils::connection;
use crate::utils::compression;
use crate::config::cache;
use crate::config::downloads;

// sanitize a mod folder name (a-Z, 0-9, " " = _)
fn sanitize_mod_folder_name(name: &str) -> String {
    let mut sanitized_name = name.replace(" ", "_");
    sanitized_name.retain(|c| c.is_alphanumeric() || c == '_');
    sanitized_name
}

// download a gamebanana mod
pub async fn download_gamebanana_mod(in_path: String, mod_url: String) {
    if let Some(mod_instance) = gamebanana::GamebananaMod::extract_mod_url(mod_url).await {
        // get the first download link, and, well, download it..
        if let Some((file_id, download_link)) = mod_instance.get_download_link().await {
            // download the file
            let sanitized_name = sanitize_mod_folder_name(&mod_instance.name);
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
            let standalone_installation = standalone::install_standalone(in_path, extraction_path.to_str().unwrap().to_string());
            //println!("{:?}", standalone_installation);
            //cache::clear_temp();
            //println!("{:?}", standalone_installation);
        }
    }
}

// download a mod from a url (determine which kind of mmod it is then act accordingly)
pub async fn download_mod_url(in_path: String, mod_url: String) {
    // gamebanana format: https://gamebanana.com/mods/502919
    // url format: https://example.com/mod.zip
    // first, check if it is a gamebanana mod
    if mod_url.contains("gamebanana.com") {
        // download the gamebanana mod
        download_gamebanana_mod(in_path, mod_url.clone()).await;
    }
    if mod_url.contains(".zip") {
        // download the mod from the url
        //download_url_mod(window, in_path, mod_url);
    }
}