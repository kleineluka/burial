// imports
use tauri::Window;
use tauri::command;
use crate::utils::game;

// command to download a mod
#[command]
pub async fn download_external_mod(window: Window, in_path: String, mod_url: String) {
    // verify that it is a game first
    if !game::verify_game(&in_path).unwrap() {
        window.emit("error", "Your TCOAAL installation is not valid. Please set it in the settings page!".to_string()).unwrap();
        return;
    }
    // get the mod source
    //let mod_source = standalone::ModSource::from_url(&mod_url);
    //window.emit("external-mod-source", mod_source.clone()).unwrap();
    //let mod_downloaded = download_mod_url(in_path, mod_url, mod_source).await;
    //window.emit("external-mod-downloaded", mod_downloaded).unwrap();
}