use std::fs;
use std::os::windows::fs::symlink_dir;
use std::os::windows::fs::symlink_file;
// imports
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::command;
use tauri::Manager;
use tauri::Window;
use crate::config;
use crate::tutorial::setup;
use crate::utils::dates;
use crate::utils::game;
use crate::utils::files;
use crate::config::cache;
use crate::config::appdata;
use crate::resources::save;
use crate::modmanager::modloader;
use crate::modmanager::installed;

/* 
INSTANCE APPDATA FILE STRUCTURE (SYMLINKED TO GAME FILES)
=========================================================
- .instance (contains the instance name)
- package.json (default or vanilla)
- tomb/ (if modded)
- saves/ (don't move other files in saves like .config, use name checking!)
*/

// structures representing game instance and game instances
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInstance {
    pub name: String,
    pub last_played: i64,
    pub date_created: String,
    pub game_version: String,
    pub is_modded: bool,
    pub mod_count: usize,
    pub is_modpack: bool,
    pub modpack_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInstances {
    pub instances: Vec<GameInstance>
}

// constants
const DEFAULT_INSTANCE: &str = "Default";
const INSTANCE_NAME_LENGTH: usize = 10;

// get the instances folder
pub fn verify_instances() {
    // make the path to use for instances
    let cache = cache::cache_folder();
    let instances_path = cache.join("instances");
    // create if it doesn't exist
    if !instances_path.exists() {
        std::fs::create_dir_all(&instances_path).unwrap();
    }
}

// return instances path
pub fn instances_path() -> PathBuf {
    verify_instances();
    let cache = cache::cache_folder();
    let instances_path = cache.join("instances");
    instances_path
}

// tiny wrapper for symlinking files
fn symlink_file_wrapper(src: &PathBuf, dst: &PathBuf) -> bool {
    println!("Symlinking file: {} -> {}", src.display(), dst.display());
    if let Err(e) = symlink_file(src, dst) {
        eprintln!("Failed to create symlink for file: {}", e);
        return false;
    }
    true
}

// tiny wrapper for symlinking directories
fn symlink_dir_wrapper(src: &PathBuf, dst: &PathBuf) -> bool {
    println!("Symlinking directory: {} -> {}", src.display(), dst.display());
    if let Err(e) = symlink_dir(src, dst) {
        eprintln!("Failed to create symlink for directory: {}", e);
        return false;
    }
    true
}

// check if an instance name is available
fn instance_name_available(instances: &GameInstances, name: String) -> bool {
    let instance = instances.instances.iter().find(|x| x.name == name);
    if instance.is_none() {
        return true;
    }
    false
}

// package.json writer
fn package_writer(is_modded: bool) -> String {
    // entry points differ
    let var_value = if is_modded {
        "tomb/index.html"  // modded
    } else {
        "www/index.html" // vanilla
    };
    // maintain original formatting otherwise..
    format!(
        r#"{{
    "name": "The Coffin of Andy and Leyley",
    "main": "{}",
    "js-flags": "--expose-gc",
    "window": {{
        "title": "",
        "toolbar": false,
        "width": 816,
        "height": 624,
        "icon": "www/icon/icon.png"
    }}
}}"#,
        var_value
    )
}

// create basic instance folder structure
fn structurize_instance(instance_name: String, instance: GameInstance) -> bool {
    // get the instances path
    let instances_path = instances_path();
    let instance_path = instances_path.join(&instance_name);
    // create the instance folder
    if !instance_path.exists() {
        std::fs::create_dir(&instance_path).unwrap();
    } 
    // in that path, we need one file called .instance with the instance name
    let instance_file = instance_path.join(".instance");
    std::fs::write(instance_file, &instance_name).unwrap();
    // create the package.json file
    let package_file = instance_path.join("package.json");
    let package_json = package_writer(instance.is_modded);
    std::fs::write(package_file, package_json).unwrap();
    // ALWAYS create the tomb folder (if not modded, it will be empty)
    let tomb_path = instance_path.join("tomb");
    if !tomb_path.exists() {
        std::fs::create_dir(&tomb_path).unwrap();
    }
    // create the saves folder
    let saves_path = instance_path.join("saves");
    if !saves_path.exists() {
        std::fs::create_dir(&saves_path).unwrap();
    }
    true
}

// move files from the game path to the instance path
fn pack_instance(in_path: String, game_instance: GameInstance, move_files: bool) -> bool {
    // two things we need: tomb and saves (not in game path)
    let instances_path = instances_path();
    let instance_path = instances_path.join(&game_instance.name);
    // if it already exists, delete it and remake it
    if instance_path.exists() {
        fs::remove_dir_all(&instance_path).unwrap();
        structurize_instance(game_instance.name.clone(), game_instance.clone());
    }
    // copy (or move) everything inside the tomb folder (recursively)
    let tomb_path = instance_path.join("tomb");
    let game_tomb = PathBuf::from(&in_path).join("tomb");
    if game_tomb.exists() {
        if move_files {
           fs::rename(&game_tomb, &tomb_path).unwrap();
        } else {
            let _ = files::copy_directory(&game_tomb.to_str().unwrap(), &tomb_path.to_str().unwrap());
        }
    }
    // copy (or move) everything inside the saves folder (recursively)
    let saves_path = instance_path.join("saves");
    let game_saves = appdata::save_folder();
    if game_saves.exists() {
        if move_files {
            fs::rename(&game_saves, &saves_path).unwrap();
        } else {
            let _ = files::copy_directory(&game_saves.to_str().unwrap(), &saves_path.to_str().unwrap());
        }
    }
    true
}

// erase all existing folders/symlinks in the game path
fn clean_game(in_path: String) -> bool {
    // files to remove: .instance and package.json
    let game_instance = PathBuf::from(&in_path).join(".instance");
    if game_instance.exists() {
        fs::remove_file(&game_instance).unwrap();
    }
    let game_package = PathBuf::from(&in_path).join("package.json");
    if game_package.exists() {
        fs::remove_file(&game_package).unwrap();
    }
    // folders to remove: tomb and saves
    let game_tomb = PathBuf::from(&in_path).join("tomb");
    if game_tomb.exists() {
        fs::remove_dir_all(&game_tomb).unwrap();
    }
    let game_saves = appdata::save_folder();
    if game_saves.exists() {
        fs::remove_dir_all(&game_saves).unwrap();
    }
    true
}

// symlink files and folders in the instance path to the game path
fn link_instance(instance_name: String, game_path: String) -> bool {
    // get instance path
    let instances_path = instances_path();
    let instance_path = instances_path.join(&instance_name);
    // symlink: .instance in root game dir
    let instance_file = instance_path.join(".instance");
    let game_instance_file = PathBuf::from(&game_path).join(".instance");
    if !symlink_file_wrapper(&instance_file, &game_instance_file) {
        return false;
    }
    // symlink: package.json in root game dir
    let package_file = instance_path.join("package.json");
    let game_package_file = PathBuf::from(&game_path).join("package.json");
    if !symlink_file_wrapper(&package_file, &game_package_file) {
        return false;
    }
    // symlink: tomb folder
    let instance_tomb = instance_path.join("tomb");
    let game_tomb = PathBuf::from(&game_path).join("tomb");
    if !symlink_dir_wrapper(&instance_tomb, &game_tomb) {
        return false;
    }
    // symlink: saves folder (not in the gqmae path!)
    let instance_saves = instance_path.join("saves");
    let game_saves = appdata::save_folder();
    if !symlink_dir_wrapper(&instance_saves, &game_saves) {
        return false;
    }
    // yay!
    true
}

// read all instances from the instances file (instances)
fn read_instances() -> GameInstances {
    // get the instances path
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // if the file doesn't exist, gracefully return (maybe better error handling here?)
    if !instances_file.exists() {
        return GameInstances {
            instances: Vec::new()
        };
    }
    // read the file
    let instances_raw = std::fs::read_to_string(&instances_file).unwrap();
    // parse the JSON
    let instances: GameInstances = serde_json::from_str(&instances_raw).unwrap();
    // return the instances
    instances
}

// create a default instance
fn create_default(in_path: String) {
    create_instance(in_path, DEFAULT_INSTANCE.to_string(), false, None, false, true);
}

// create a new instance
fn create_instance(in_path: String, instance_name: String, is_modpack: bool, 
    modpack_name: Option<String>, fresh_instance: bool, instance_linked: bool) -> String {
    // verify that the game path, is well, a game path
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        return "error_game_path".to_string();
    }
    // make sure the instance name is not too long
    if instance_name.len() > INSTANCE_NAME_LENGTH {
        return "error_instance_name".to_string();
    }
    // make sure the instance name is available (if the json even exists)
    let instances_file = instances_path().join("instances.json");
    if !instances_file.exists() {
        if !instance_name_available(&read_instances(), instance_name.clone()) {
            return "error_instance_name".to_string();
        }
    }
    // create the file structure
    let game_version = game::game_version(in_path.clone());
    let is_modded = modloader::modloader_prescence(in_path.clone());
    let mod_count = if is_modded { installed::get_installed_mods(in_path.clone()).len() } else { 0 };
    let instance = GameInstance {
        name: instance_name.clone(),
        last_played: 0, // never played, but normally days_passed
        date_created: chrono::Local::now().to_string(),
        game_version,
        is_modded,
        mod_count,
        is_modpack,
        modpack_name,
    };
    // add the instance to the instances file
    let mut instances = read_instances();
    instances.instances.push(instance.clone());
    // write the instances file
    let instances_json = serde_json::to_string(&instances).unwrap();
    std::fs::write(instances_file, instances_json).unwrap();
    // create the instance folder structure
    let structure = structurize_instance(instance_name.clone(), instance.clone());
    if !structure {
        return "error_instance_structure".to_string();
    }
    // pack the instance (if desired)
    if !fresh_instance {
        let pack = pack_instance(in_path.clone(), instance.clone(), instance_linked.clone());
        if !pack {
            return "error_instance_packing_notfresh".to_string();
        }
    } else {
        // if this is a fresh install, then we wanna pack the current instance (but ignore for when it is the very first instance!)
        if instance_name != DEFAULT_INSTANCE {
            let current_instance = active_instance(in_path.clone(), true);
            let current_instance_json = get_instance(&read_instances(), current_instance.clone()).unwrap();
            let current_instance_path = instances_path().join(&current_instance);
            let pack = pack_instance(current_instance_path.to_str().unwrap().to_string(), current_instance_json, true);
            if !pack {
                return "error_instance_packing_fresh".to_string();
            }
        }
    }
    // link the instance (if desired)
    if instance_linked {
        clean_game(in_path.clone());
        let link = link_instance(instance_name.clone(), in_path.clone());
        if !link {
            // save may not link, so ignore: return "error_instance_linking".to_string();
        }
    }
    // yay!
    "success".to_string()
}

// delete an instance
fn delete_instance(in_path: String, instance_name: String) -> bool {
    // get the instances path
    let instances_path = instances_path();
    let instance_path = instances_path.join(&instance_name);
    // remove the instance folder
    fs::remove_dir_all(&instance_path).unwrap();
    // remove the instance from the instances file
    let mut instances = read_instances();
    instances.instances.retain(|x| x.name != instance_name);
    // write the instances file
    let instances_file = instances_path.join("instances.json");
    let instances_json = serde_json::to_string(&instances).unwrap();
    std::fs::write(instances_file, instances_json).unwrap();
    // if the instance was active, set the default instance
    let active_instance = active_instance(in_path.clone(), true);
    if active_instance == instance_name {
        let _ = clean_game(in_path.clone());
        let _ = link_instance(DEFAULT_INSTANCE.to_string(), in_path.clone());
    }
    // yay!
    true
}

// swap the active instance
fn swap_instance(in_path: String, instance_name: String) -> bool {
    // get the instances path
    let instances_path = instances_path();
    // pack the current instance
    let current_instance = active_instance(in_path.clone(), true);
    let current_instance_json = get_instance(&read_instances(), current_instance.clone()).unwrap();
    let pack = pack_instance(in_path.clone(), current_instance_json, true);
    if !pack {
        return false;
    }
    // clean the game path
    let clean = clean_game(in_path.clone());
    if !clean {
        return false;
    }
    // link the new instance
    let link = link_instance(instance_name.clone(), in_path.clone());
    if !link {
        return false;
    }
    // yay!
    true
}

// find the active instance
pub fn active_instance(in_path: String, using_instances: bool) -> String {
    // see if instances are enabled first, if not, just call it default
    if !using_instances {
        return DEFAULT_INSTANCE.to_string();
    }
    // check if the .instance file exists in the game path
    let game_instance = PathBuf::from(&in_path).join(".instance");
    if game_instance.exists() {
        // read the instance name
        let instance_name = std::fs::read_to_string(&game_instance).unwrap();
        return instance_name;
    }
    // if not, return the default instance
    create_default(in_path.clone());
    DEFAULT_INSTANCE.to_string()
}

// take an instance name and return the instance
fn get_instance(instances: &GameInstances, instance_name: String) -> Option<GameInstance> {
    let instance = instances.instances.iter().find(|x| x.name == instance_name);
    if instance.is_none() {
        return None;
    }
    Some(instance.unwrap().clone())
}