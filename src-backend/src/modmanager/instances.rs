// imports
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::config::cache;
use crate::modmanager::modloader;
use crate::modmanager::installed;
use crate::utils::dates;
use crate::utils::game;

#[derive(Serialize, Deserialize, Debug)]
struct GameInstance {
    name: String,
    last_played: i64,
    is_modded: bool,
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

// return deno path
pub fn instances_path() -> PathBuf {
    verify_instances();
    let cache = cache::cache_folder();
    let instances_path = cache.join("instances");
    instances_path
}

// create the default instances.json (of current instance)
fn create_instances(in_path: String) -> String {
    // make sure it's even a game folder (?)
    let is_game = game::verify_game(&in_path).unwrap();
    if (!is_game) {
        return "error:gamepath".to_string();
    } 
    // gather required data about the instance
    let is_modded = modloader::modloader_prescence(in_path.clone());
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
        name: "default".to_string(),
        last_played: days_passed,
        is_modded: is_modded,
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
    "success".to_string()
}

// determine if a new instance