// imports
use tauri::Window;
use tauri::command;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;
use crate::config::downloads;
use crate::utils::codeberg;
use crate::utils::game;
use crate::utils::compression;
use crate::utils::files;

// there should be a better way to store/fetch these
const MODLOADER_REPO: &str = "https://codeberg.org/basil/tomb";
const MODLOADER_FILE : &str = "tomb.zip";

// check if a modloader is present
pub fn modloader_prescence(in_path: String) -> bool {
    // there should be a "mods" and "tomb" folder in the game directory
    let mods_dir = format!("{}\\www\\mods", in_path);
    let tomb_dir = format!("{}\\www\\tomb", in_path);
    let mods_exists = Path::new(&mods_dir).exists();
    let tomb_exists = Path::new(&tomb_dir).exists();
    return mods_exists && tomb_exists;
}

// download the latest release of tomb
#[command]
pub async fn install_modloader(window: Window, in_path: String) {
    // make sure that the provided path is a valid game folder
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "That is not a valid TCOAAL folder!").unwrap();
        return;
    }
    // ensure that there is a valid downloads folder in the cache
    window.emit("status", "Downloading mod loader..").unwrap();
    let downloads_dir = downloads::downloads_folder().to_string_lossy().to_string();
    downloads::verify_downloads().unwrap();
    // download the latest release of tomb
    let download_result = codeberg::download_latest_release(MODLOADER_REPO, MODLOADER_FILE, &downloads_dir).await;
    if !download_result {
        window.emit("error", "Failed to download the mod loader!").unwrap();
        return;
    }
    // extract the tomb modloader
    window.emit("status", "Extracting mod loader..").unwrap();
    let tomb_file_location_str = format!("{}\\{}", downloads_dir, MODLOADER_FILE);
    let tomb_file_location = Path::new(&tomb_file_location_str);
    let extraction_destination_str = format!("{}\\{}", downloads_dir, "modloader");
    let extraction_destination = Path::new(&extraction_destination_str);
    compression::decompress_directory(&tomb_file_location, &extraction_destination).unwrap();
    // backup index.html in game\\www, then delete it
    let game_index_html = format!("{}\\www\\index.html", in_path);
    files::backup_file_multiple(&game_index_html);
    files::delete_file(&game_index_html);
    // copy the tomb modloader to the game directory
    window.emit("status", "Copying to game installation..").unwrap();
    let game_destination = format!("{}\\www", in_path);
    files::copy_directory(&extraction_destination.to_string_lossy(), &game_destination).unwrap();
    // cleanup
    window.emit("status", "Cleaning up..").unwrap();
    files::delete_file(&tomb_file_location_str);
    files::delete_folder(&extraction_destination.to_string_lossy());
    files::delete_folder(&downloads_dir);
    // done + reload installed modloader version
    window.emit("status", "Mod loader installed!").unwrap();
    modloader_version(window, in_path);
}

// uninstall the modloader
#[allow(dead_code)]
#[command]
pub fn uninstall_modloader(window: Window, in_path: String) {
    // make sure that the provided path is a valid game folder
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "That is not a valid TCOAAL folder!").unwrap();
        return;
    }
    // check if the modloader is present
    let modloader_present = modloader_prescence(in_path.clone());
    if !modloader_present {
        window.emit("error", "No mod loader installed!").unwrap();
        return;
    }
    // delete the tomb modloader
    let tomb_dir = format!("{}\\www\\tomb", in_path);
    files::delete_folder(&tomb_dir);
    // restore index.html in game\\www
    let _game_index_html = format!("{}\\www\\index.html", in_path);
    //files::restore_file_multiple(&game_index_html);
    // done + reload installed modloader version
    window.emit("status", "Mod loader uninstalled!").unwrap();
    modloader_version(window, in_path);
}

// get all available versions of tomb
#[command]
pub async fn modloader_versions(window: Window) {
    // fetch all releases from the codeberg repo
    let codeberg_api = codeberg::get_codeberg_api(MODLOADER_REPO);
    let releases = codeberg::get_all_releases(&codeberg_api).await;
    let mut versions = Vec::new();
    if let Ok(releases) = releases {
        for release in releases {
            versions.push(release);
        }
    } else {
        // handle the error case
        window.emit("error", Some("Failed to fetch modloader versions!".to_string())).unwrap();
        return;
    }
    window.emit("modloader-versions", Some(versions)).unwrap();
}

// get the current installed version of tomb
#[command]
pub fn modloader_version(window: Window, in_path: String) {
    // verify it is a game path, first..
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("modloader-version", Some("Tomb not installed")).unwrap();
        return;
    }
    // is SOME mod loader installed?
    let modloader_present = modloader_prescence(in_path.clone());
    if !modloader_present {
        window.emit("modloader-version", Some("Tomb not installed.")).unwrap();
        return;
    }
    // open in_path + tomb/tomb.js
    let tomb_js_path = format!("{}\\www\\tomb\\tomb.js", &in_path);
    let tomb_js_file = File::open(tomb_js_path).unwrap();
    let tomb_js_reader = io::BufReader::new(tomb_js_file);
    // find the first line before the version is declared, as it is unique, then extract the version..
    let initial_re = Regex::new(r"window.\$tomb = this;").unwrap();
    let version_re = Regex::new(r#"this.version = "(.*?)";"#).unwrap();
    let mut lines = tomb_js_reader.lines();
    let mut version = "Unknown".to_string();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if initial_re.is_match(&line) {
            // Fetch the next line after matching
            if let Some(next_line) = lines.next() {
                if let Ok(next_line) = next_line {
                    if let Some(captures) = version_re.captures(&next_line) {
                        version = captures.get(1).unwrap().as_str().to_string();
                    }
                }
            }
            break;
        }
    }
    window.emit("modloader-version", Some(format!("{}", version))).unwrap();
}