// imports
use tauri::Window;
use tauri::command;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;
use std::io::{self, BufRead, BufReader, Write};
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
    let tomb_dir = format!("{}\\tomb", in_path);
    let tomb_js_path = format!("{}\\tomb\\tomb\\tomb.js", in_path);
    let tomb_index_path = format!("{}\\tomb\\index.html", in_path);
    let tomb_exists = Path::new(&tomb_dir).exists();
    let tomb_js_exists = Path::new(&tomb_js_path).exists();
    let tomb_index_exists = Path::new(&tomb_index_path).exists();
    tomb_exists && tomb_js_exists && tomb_index_exists
}

// edit the package.json to either install or uninstall tomb
fn edit_package(package_path: String, direction: String) -> bool {
    // determine the target and replacement strings
    let (target, replacement) = match direction.as_str() {
        "install" => ("www/index.html", "tomb/index.html"),
        "uninstall" => ("tomb/index.html", "www/index.html"),
        _ => panic!("Invalid direction! Use 'install' or 'uninstall'."),
    };
    // read the package.json file
    let mut package_file = File::open(&package_path).expect("Failed to open package.json file");
    let mut package_content = String::new();
    package_file
        .read_to_string(&mut package_content)
        .expect("Failed to read package.json");
    // check if the target string exists
    if !package_content.contains(target) {
        println!("Target string not found in package.json");
        return false; // maybe a past manual edit?
    }
    // replace and rewrite
    let new_content = package_content.replace(target, replacement);
    let mut package_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&package_path)
        .expect("Failed to open package.json for writing");
    package_file
        .write_all(new_content.as_bytes())
        .expect("Failed to write to package.json");
    true // (yay!)
}

// download and install tomb
#[command]
pub async fn install_modloader(window: Window, in_path: String, in_name: String) {
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
    // download a specific or the latest release of tomb
    println!("{}", in_name);
    let download_result = if in_name == "latest" {
        codeberg::download_latest_release(MODLOADER_REPO, MODLOADER_FILE, &downloads_dir).await
    } else {
        codeberg::download_specific_release(MODLOADER_REPO, MODLOADER_FILE, &downloads_dir, &in_name).await
    };
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
    // edit the package.json file
    window.emit("status", "Rewriting package.json..").unwrap();
    let game_package_json = format!("{}\\package.json", in_path);
    println!("{}", game_package_json);
    let package_edited = edit_package(game_package_json, "install".to_string());
    if !package_edited {
        window.emit("error", "Failed to edit package.json!").unwrap();
        window.emit("status-clear", "").unwrap();
        return;
    }
    // in the game directory, make a "tomb" folder if it doesn't exist
    let tomb_dir = format!("{}\\tomb", in_path);
    if !Path::new(&tomb_dir).exists() {
        fs::create_dir_all(&tomb_dir).unwrap();
    }
    // copy the tomb modloader to the game directory
    window.emit("status", "Copying to game installation..").unwrap();
    let game_destination = format!("{}\\tomb", in_path);
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
#[command]
pub fn uninstall_modloader(window: Window, in_path: String) {
    // make sure that the provided path is a valid game folder
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "That is not a valid TCOAAL folder!").unwrap();
        return;
    }
    // check if the modloader is present
    window.emit("status", "Checking for mod loader..").unwrap();
    let modloader_present = modloader_prescence(in_path.clone());
    if !modloader_present {
        window.emit("error", "No mod loader installed!").unwrap();
        return;
    }
    // delete the tomb modloader
    window.emit("status", "Uninstalling mod loader..").unwrap();
    let tomb_dir = format!("{}\\tomb", in_path);
    files::delete_folder(&tomb_dir);
    // edit the package.json file
    window.emit("status", "Rewriting package.json..").unwrap();
    let game_package_json = format!("{}\\package.json", in_path);
    let package_edited = edit_package(game_package_json, "uninstall".to_string());
    if !package_edited {
        window.emit("error", "Failed to edit package.json!").unwrap();
        return;
    }
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
        window.emit("modloader-version", Some("Tomb not installed")).unwrap();
        return;
    }
    // open in_path + tomb/tomb.js
    let tomb_js_path = format!("{}\\tomb\\tomb\\tomb.js", &in_path);  
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
            // fetch the next line after matching
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