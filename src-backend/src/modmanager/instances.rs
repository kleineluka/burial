// imports
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::command;
use tauri::Manager;
use tauri::Window;
use crate::config;
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

// check if an instance name is available
fn instance_name_available(instances: &GameInstances, name: String) -> bool {
    let instance = instances.instances.iter().find(|x| x.name == name);
    if instance.is_none() {
        return true;
    }
    false
}

// create the default instances.json (of current instance)
fn default_instances(in_path: String) -> String {
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
        name: DEFAULT_INSTANCE.to_string(),
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

    // in the game path, make a file called .instance with the instance name
    let instance_file = PathBuf::from(in_path).join(".instance");
    std::fs::write(instance_file, DEFAULT_INSTANCE).unwrap();
    "success".to_string()
}

// load active instance
pub fn active_instance(in_path: String) -> String {
    // first of, see if the game path is valid
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        return DEFAULT_INSTANCE.to_string();
    }
    // first - try and load the instance from the .instance file
    let instance_file = PathBuf::from(in_path.clone()).join(".instance");
    if instance_file.exists() {
        let instance = std::fs::read_to_string(instance_file).unwrap();
        return instance;
    }
    // if the file doesn't exist, let's look at the instances.json
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    if !instances_file.exists() {
        // create the default instances.json (if it can)
        let result = default_instances(in_path.clone());
        if result == "error:gamepath" {
            return DEFAULT_INSTANCE.to_string();
        }
    }
    // get the active instance
    let instances_json = std::fs::read_to_string(instances_file).unwrap();
    let instances: GameInstances = serde_json::from_str(&instances_json).unwrap();
    let active_instance = instances.instances.iter().find(|&x| x.is_active == true).unwrap();
    active_instance.name.clone()
}

// load current instance
pub fn load_current_instance(window: &Window, in_path: String) -> String {
    // get the instances path
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // check if instances.json exists
    if !instances_file.exists() {
        // create the default instances.json (if it can)
        let result = default_instances(in_path);
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

// load instances
#[command]
pub fn load_instances(window: Window, in_path: String) {
    // get the instances path
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // check if instances.json exists
    if !instances_file.exists() {
        // create the default instances.json (if it can)
        let result = default_instances(in_path);
        if result == "error:gamepath" {
            window.emit("instances-errored", "gamepath").unwrap();
            return;
        } else {
            // set in the local cache the active instance (Default)
            let _ = config::storage::insert_into_store(&window.app_handle(), "active-instance", serde_json::Value::String(DEFAULT_INSTANCE.to_string())).unwrap();
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

// instance verifier
#[command]
pub fn verify_instance(window: Window, instance: String) {
    // get the instances path
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // check if instances.json exists, if not, error + return
    if !instances_file.exists() {
        window.emit("instances-verification", "error").unwrap();
        return;
    }
    // read the file
    let instances_json = std::fs::read_to_string(instances_file).unwrap();
    // parse the json
    let instances: GameInstances = serde_json::from_str(&instances_json).unwrap();
    // see if the instance exists
    let instance_exists = instances.instances.iter().find(|&x| x.name == instance);
    if instance_exists.is_none() {
        window.emit("instances-verification", "error").unwrap();
        return;
    }
    window.emit("instances-verification", "success").unwrap();
}

// refresh active instance
#[command]
pub fn refresh_active(window: Window, in_path: String) {
    // wrapper for active_instance to update on load
    let active = active_instance(in_path);
    window.emit("active-instance", active).unwrap();
}

// rename an instance
#[command]
pub fn rename_instance(window: Window, in_path: String, old_name: String, new_name: String) {
    // there are two places to rename the instance:.instance file, instances.json
    window.emit("status", "Loading instances from storage..").unwrap();
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // read the file
    let instances_json = std::fs::read_to_string(&instances_file).unwrap();
    // parse the json
    let mut instances: GameInstances = serde_json::from_str(&instances_json).unwrap();
    // make sure 1) the instance exists
    window.emit("status", "Making sure the name is not taken..").unwrap();
    if instances.instances.iter().find(|x| x.name == old_name).is_none() {
        window.emit("instances-error", "current-nonexistant").unwrap();
        return;
    }
    // and 2) the new name doesn't already exist
    if !instance_name_available(&instances, new_name.clone()) {
        window.emit("instances-error", "name-taken").unwrap();
        return;
    }
    // 3) make sure the new name is not too long
    if new_name.len() > INSTANCE_NAME_LENGTH {
        window.emit("instances-error", "name-toolong").unwrap();
        return;
    }
    // find the instance
    window.emit("status", "Renaming instance~").unwrap();
    let instance = instances.instances.iter_mut().find(|x| x.name == old_name).unwrap();
    // rename the instance
    instance.name = new_name.clone();
    // write the file
    let instances_json = serde_json::to_string(&instances).unwrap();
    std::fs::write(instances_file, instances_json).unwrap();
    // rename the .instance file IF it is the old name (aka the active instance)
    window.emit("status", "Checking active instance..").unwrap();
    let active_file = PathBuf::from(in_path.clone()).join(".instance");
    if active_file.exists() {
        let active_instance = std::fs::read_to_string(&active_file).unwrap();
        if active_instance == old_name {
            window.emit("status", "Changing active instance to new name..").unwrap();
            std::fs::write(active_file, new_name.clone()).unwrap();
        }
    }
    // update the active instance
    let active = active_instance(in_path.clone());
    window.emit("active-instance", active).unwrap();
    // update the instances
    let instances = serde_json::to_string(&instances).unwrap();
    window.emit("instances-loaded", instances).unwrap();
    // return success
    window.emit("status", format!("Instance renamed to {}!", &new_name)).unwrap();
    window.emit("instances-renamed", "success").unwrap();
}

// clone an instance
#[command]
pub fn clone_instance(window: Window, in_path: String, old_instance: String, new_instance: String) {
    // load instances
    window.emit("status", "Loading instances from storage..").unwrap();
    let instances_path = instances_path();
    let instances_file = instances_path.join("instances.json");
    // read the file
    let instances_json = std::fs::read_to_string(&instances_file).unwrap();
    // parse the json
    let mut instances: GameInstances = serde_json::from_str(&instances_json).unwrap();
    // make sure 1) the instance exists
    window.emit("status", "Making sure the name is not taken..").unwrap();
    if instances.instances.iter().find(|x| x.name == old_instance).is_none() {
        window.emit("instances-error", "old-nonexistant").unwrap();
        return;
    }
    // and 2) the new name doesn't already exist
    if !instance_name_available(&instances, new_instance.clone()) {
        window.emit("instances-error", "name-taken").unwrap();
        return;
    }
    // 3) make sure the new name is not too long
    if new_instance.len() > INSTANCE_NAME_LENGTH {
        window.emit("instances-error", "name-toolong").unwrap();
        return;
    }
    // and now, copy all files in instances_path/old/ to instances_path/new/
    window.emit("status", "Cloning instance files..").unwrap();
    let old_instance_path = instances_path.join(old_instance.clone());
    // make sure old instance exists (just called empty for easy error handling)
    if !old_instance_path.exists() {
        window.emit("instances-cloned", "error-empty").unwrap();
        return;
    }
    let new_instance_path = instances_path.join(new_instance.clone());
    std::fs::create_dir(&new_instance_path).unwrap();
    // copy all files
    let _ = std::fs::copy(&old_instance_path, &new_instance_path).unwrap();
    // find the instance
    window.emit("status", "Adding new instance to the registry~").unwrap();
    let instance = instances.instances.iter().find(|x| x.name == old_instance).unwrap();
    // clone the instance
    let new_instance = GameInstance {
        name: new_instance.clone(),
        last_played: instance.last_played,
        index_kind: instance.index_kind.clone(),
        mod_count: instance.mod_count,
        date_created: instance.date_created.clone(),
        game_version: instance.game_version.clone(),
        is_active: false
    };
    instances.instances.push(new_instance);
    // write the file
    let instances_json = serde_json::to_string(&instances).unwrap();
    std::fs::write(instances_file, instances_json).unwrap();
}