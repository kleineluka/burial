// imports
use tauri::Window;
use tauri::command;
use std::path::Path;
use crate::config::downloads;
use crate::utils::files;
use crate::utils::game;
use crate::utils::connection;
use crate::utils::compression;

// detect if the developer or player sdk is installed
pub fn sdk_prescence(in_path: String) -> String {
    // folder "pnad" and file "payload.exe" only exist in the developer sdk
    let is_dev_sdk = Path::new(&in_path).join("pnad").exists() && Path::new(&in_path).join("payload.exe").exists();
    if is_dev_sdk {
        return "SDK".to_string();
    }
    return "Player".to_string();
}

// wrapper for the sdk page to call the above, but let it be used by other back-end functions
#[command]
pub fn sdk_presence_wrapper(window: Window, in_path: String) {
    if game::verify_game(&in_path).unwrap() {
        let sdk = sdk_prescence(in_path);
        window.emit("sdk-presence", Some(sdk)).unwrap();
        return;
    }
    window.emit("status", Some("Invalid TCOAAL folder!".to_string())).unwrap();
}

// install an sdk from a given url and into a file path
#[command]
pub async fn install_sdk(window: Window, in_url: String, in_path: String) {
    // sanity checks..
    let in_path = Path::new(&in_path);
    let is_game = game::verify_game(&in_path.to_string_lossy()).unwrap();
    if !in_path.exists() || !is_game {
        window.emit("error", Some("That is not a valid TCOAAL folder!".to_string())).unwrap();
        return;
    }
    window.emit("status", Some("Setting up download environment..".to_string())).unwrap();
    // in the game folder, rename "Game.exe to "Game.exe.bak"
    window.emit("status", Some("Backing up game executable..".to_string())).unwrap();
    let game_exe = in_path.join("Game.exe");
    let game_exe_bak = in_path.join("Game.exe.bak");
    if game_exe.exists() {
        files::rename_file(&game_exe.to_string_lossy(), &game_exe_bak.to_string_lossy());
    }
    // ensure that there is a valid downloads folder in the cache
    window.emit("status", Some("Downloading desired SDK.. (this may take some time)".to_string())).unwrap();
    let downloads_dir = downloads::downloads_folder().to_string_lossy().to_string();
    downloads::verify_downloads().unwrap();
    // download the sdk
    if let Err(e) = connection::download_file(&in_url, &downloads_dir).await {
        window.emit("error", Some(format!("Download failed: {}", e))).unwrap();
        return;
    }
    // extract the sdk
    window.emit("status", Some("Download complete, extracting.. (this may take some time)".to_string())).unwrap();
    let sdk_name = in_url.split("/").last().unwrap();
    let sdk_file_location = format!("{}/{}", downloads_dir, sdk_name);
    let path_sdk = Path::new(&sdk_file_location);
    let path_extracted = Path::new(&downloads_dir);
    compression::decompress_zip(&path_sdk, &path_extracted).unwrap();
    // move all files in folder to game folder (in_path)
    window.emit("status", Some("Moving SDK files to game folder..".to_string())).unwrap();
    let sdk_extracted = path_extracted.join(sdk_name.replace(".zip", ""));
    let sdk_zip = path_extracted.join(sdk_name);
    files::copy_directory(&sdk_extracted.to_string_lossy(), &in_path.to_string_lossy()).unwrap();
    // rename nw.exe to Game.exe
    window.emit("status", Some("Renaming SDK executable..".to_string())).unwrap();
    let sdk_exe = in_path.join("nw.exe");
    if sdk_exe.exists() {
        files::rename_file(&sdk_exe.to_string_lossy(), &game_exe.to_string_lossy());
    }
    // cleanup
    window.emit("status", Some("Deleting downloaded zip..".to_string())).unwrap();
    files::delete_file(&sdk_zip.to_string_lossy());
    window.emit("status", Some("Deleting extracted files..".to_string())).unwrap();
    files::delete_folder(&sdk_extracted.to_string_lossy());
    // done + reload the installed sdk
    let sdk_status = sdk_prescence(in_path.to_string_lossy().to_string());
    window.emit("sdk-presence", Some(sdk_status)).unwrap();
    window.emit("status", Some("SDK installation complete!".to_string())).unwrap();
}