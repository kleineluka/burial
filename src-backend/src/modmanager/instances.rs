// imports
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::command;
use tauri::Manager;
use tauri::Window;
use crate::config::cache;
use crate::config::storage;
use crate::modmanager::modloader;
use crate::modmanager::installed;
use crate::utils::dates;
use crate::utils::game;

#[derive(Serialize, Deserialize, Debug)]
struct GameInstance {
    name: String,
    last_played: i64,
    index_kind: String,
    mod_count: usize,
    date_created: String,
    game_version: String,
    is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameInstances {
    manifest: String,
    instances: Vec<GameInstance>,
}

// constant for the manifest
const MANIFEST: &str = "1.0.0";

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

// create the default instances.json (of current instance)
fn default_instances(window: &Window, in_path: String) -> String {
    // make sure it's even a game folder (?)
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        return "error:gamepath".to_string();
    } 
    // gather required data about the instance
    let is_modded = modloader::modloader_prescence(in_path.clone());
    let index_kind = if is_modded {
        "Tomb Modloader 🪦".to_string()
    } else {
        "Vanilla Game 🍦".to_string()
    };
    let mods = if is_modded {
        installed::get_installed_mods(in_path.clone())
    } else {
        Vec::new()
    };
    let total_mods = mods.len();
    let date = dates::get_date();
    let days_passed = dates::days_passed(date.clone());
    let game_version = game::game_version(in_path.clone());
    let instance = GameInstance {
        name: "Default".to_string(),
        last_played: days_passed,
        index_kind: index_kind,
        mod_count: total_mods,
        date_created: date,
        game_version: game_version,
        is_active: true
    };
    let instances = GameInstances {
        manifest: MANIFEST.to_string(),
        instances: vec![instance]
    };
    let instances_json = serde_json::to_string(&instances).unwrap();
    // write to file
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    std::fs::write(instances_file, instances_json).unwrap();
    // branch based on if we are hot loading or not
    let hotload = storage::read_from_store(&window.app_handle(), "settings-hotload")
        .map(|v| v.as_bool().unwrap_or(false))
        .unwrap_or(false);
    // in the game path, make a file called .instance with the instance name
    let instance_file = PathBuf::from(in_path).join(".instance");
    std::fs::write(instance_file, "default").unwrap();
    "success".to_string()
}

// load instances
#[command]
pub fn load_instances(window: Window, in_path: String) {
    // get the instances path
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // check if instances.json exists
    if !instances_file.exists() {
        // create the default instances.json (if it can)
        let result = default_instances(&window, in_path);
        if result == "error:gamepath" {
            window.emit("instances-errored", "gamepath").unwrap();
            return;
        }
    }
    // read the file
    let instances_json = std::fs::read_to_string(instances_file).unwrap();
    // parse the json
    let instances: GameInstances = serde_json::from_str(&instances_json).unwrap();
    // to hold off errors for now
    let instances = serde_json::to_string(&instances).unwrap();
    // return the instances
    window.emit("instances-loaded", instances).unwrap();
}

// load current instance
pub fn load_current_instance(window: &Window, in_path: String) -> String {
    // get the instances path
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // check if instances.json exists
    if !instances_file.exists() {
        // create the default instances.json (if it can)
        let result = default_instances(window, in_path);
        if result == "error:gamepath" {
            return "error:gamepath".to_string();
        }
    }
    // read the file
    let instances_json = std::fs::read_to_string(instances_file).unwrap();
    // parse the json
    let instances: GameInstances = serde_json::from_str(&instances_json).unwrap();
    // get the current instance
    let current_instance = instances.instances.iter().find(|&x| x.is_active == true).unwrap();
    // to hold off errors for now
    let current_instance = serde_json::to_string(&current_instance).unwrap();
    // return the current instance
    current_instance
}

// rename an instance
#[command]
pub fn rename_instance(window: Window, old_name: String, new_name: String) {
    // to-do
}

// clone an instance
#[command]
pub fn clone_instance(window: Window, in_path: String, instance_name: String) {
    // to do
}