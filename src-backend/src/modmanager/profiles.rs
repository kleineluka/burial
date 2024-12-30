// imports
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use dirs::config_dir;
use tauri::{command, Window};
use crate::modmanager::installed;
use crate::utils::{files, game};

// structure containing mod folder and mod.json file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profiles {
    pub profiles: Vec<Profile>,
    pub current: String,
    pub copy_version: String, 
}

impl Default for Profiles {
    fn default() -> Self {
        Self {
            profiles: vec![Profile::default()],
            current: String::from("Default"),
            copy_version: String::from("notset"),
        }
    }
}

// the actual profile contents
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub mods: Vec<ProfileMod>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            mods: vec![], 
        }
    }
}

// a slimmed down mod info for the profile
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileMod {
    pub id: String,
    pub folder: String,
    pub status: bool,
}

// structure representing the profiles folder
pub struct ProflesPath {
    pub base_dir: PathBuf,
}

impl ProflesPath {

    // make it with the default path
    pub fn new() -> Self {
        let base_dir = config_dir().unwrap_or_else(|| PathBuf::from("."));
        ProflesPath {
            base_dir: base_dir.join("burial").join("profiles"),
        }
    }

    // get the profiles folder
    pub fn profiles_folder(&self) -> PathBuf {
        self.base_dir.clone()
    }

    // make sure it exists
    pub fn verify_profiles(&self) -> std::io::Result<()> {
        files::verify_folder(&self.base_dir)?;
        Ok(())
    }

    // clear the profiles folder
    pub fn clear_profiles(&self) -> std::io::Result<()> {
        files::delete_folder(&self.base_dir.to_string_lossy().to_string());
        Ok(())
    }

    // get the path to the game copy
    pub fn game_copy(&self) -> PathBuf {
        self.base_dir.join("game_copy")
    }

    // verify the game copy folder eists
    pub fn verify_game_copy(&self) -> std::io::Result<()> {
        files::verify_folder(&self.game_copy())?;
        Ok(())
    }

    // get the path to the profiles.json file
    pub fn profiles_json(&self) -> PathBuf {
        self.base_dir.join("profiles.json")
    }

}

// update what mods are (and aren't) in the profile
fn update_profile(profile: Profile, installed_mods: Vec<installed::ModFolder>) -> Profile {
    let old_mods = profile.mods.clone();
    let mut new_mods: Vec<ProfileMod> = vec![];
    // go through the current mods present in the profile
    for profile_mod in &old_mods {
        // check if the mod is still present via id
        if let Some(installed_mod) = installed_mods.iter().find(|m| m.modjson.id == profile_mod.id) {
            new_mods.push(ProfileMod {
                id: installed_mod.modjson.id.clone(),
                folder: installed_mod.folder.clone(),
                status: profile_mod.status,
            });
        }
    }
    // check if there are any new installed mods
    for installed_mod in installed_mods {
        if !old_mods.iter().any(|m| m.id == installed_mod.modjson.id) {
            new_mods.push(ProfileMod {
                id: installed_mod.modjson.id.clone(),
                folder: installed_mod.folder.clone(),
                status: true, // default to enabled
            });
        }
    }
    // build the new profile
    Profile {
        name: profile.name,
        mods: new_mods,
    }
}

// write profiles to disk
fn write_profiles(profiles: Profiles) {
    let profiles_path = ProflesPath::new();
    let profiles_json = profiles_path.profiles_json();
    let profiles_json_str = serde_json::to_string(&profiles).unwrap();
    fs::write(profiles_json, profiles_json_str).unwrap();
}

// read profiles from disk
fn read_profiles(in_path: String) -> Profiles {
    let profiles_path = ProflesPath::new();
    profiles_path.verify_profiles().unwrap();
    let profiles_json = profiles_path.profiles_json();
    if !profiles_json.exists() {
        init_profiles_db(in_path);
    }
    let profiles_json_str = fs::read_to_string(profiles_json).unwrap();
    let profiles: Profiles = serde_json::from_str(&profiles_json_str).unwrap();
    profiles
}

// init profiles database
fn init_profiles_db(in_path: String) {
    // verify the profiles folder
    let profiles_path = ProflesPath::new();
    profiles_path.verify_profiles().unwrap();
    // see if the game path is set up or not
    let is_game = game::verify_game(&in_path).unwrap();
    let version: String = if is_game {
        game::game_version(in_path.clone())
    } else {
        "notset".to_string()
    };
    // create the profiles database
    let mut profiles = Profiles::default();
    let is_game = game::verify_game(&in_path).unwrap();
    if is_game {
        let installed = installed::get_installed_mods(in_path.clone());
        let updated_profile = update_profile(profiles.profiles[0].clone(), installed);
        profiles.profiles[0] = updated_profile;
    }
    profiles.copy_version = version;
    write_profiles(profiles);
}

// add a profile to the database
fn add_profile_db(in_path: String, profile_name: String) -> String {
    // if file name exists
    let mut profiles = read_profiles(in_path.clone());
    if profiles.profiles.iter().any(|p| p.name == profile_name) {
        return "exists".to_string();
    }
    let new_profile = Profile {
        name: profile_name,
        mods: vec![],
    };
    let installed_mods = installed::get_installed_mods(in_path.clone());
    let updated_profile = update_profile(new_profile, installed_mods);
    profiles.profiles.push(updated_profile);
    write_profiles(profiles);
    "success".to_string()
}

// remove a profile from the database
fn remove_profile_db(in_path: String, profile_name: String) -> String {
    let mut profiles = read_profiles(in_path);
    if profile_name == "Default" {
        return "default".to_string();
    }
    if profiles.profiles.iter().any(|p| p.name == profile_name) {
        profiles.profiles.retain(|p| p.name != profile_name);
        write_profiles(profiles);
        return "success".to_string();
    }
    "notfound".to_string()
}

// update the whole database
fn update_db(in_path: String) {
    let mut profiles = read_profiles(in_path.clone());
    let installed_mods = installed::get_installed_mods(in_path.clone());
    for profile in &mut profiles.profiles {
        let updated_profile = update_profile(profile.clone(), installed_mods.clone());
        *profile = updated_profile;
    }
    write_profiles(profiles);
}

// get the version of the installation
fn get_installation() -> String {
    // see if the game copy exists
    let installation_path = ProflesPath::new().game_copy();
    if installation_path.exists() {
        let game_version = game::game_version(installation_path.to_string_lossy().to_string());
        if game_version != "Unknown" {
            return game_version;
        }
    }
    "notset".to_string()
}

// delete the installation
fn delete_installation() -> String {
    let installation_path = ProflesPath::new().game_copy();
    if installation_path.exists() {
        files::delete_folder(&installation_path.to_string_lossy().to_string());
        return "success".to_string();
    }
    "notfound".to_string()
}

// build the installation
fn init_installation(in_path: String) -> String {
    // verify that the game is installed
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        return "nogame".to_string();
    }
    // get the profiles folder (and if it exists, delete it)
    let installation_path = ProflesPath::new().game_copy();
    let _ = delete_installation();
    // copy the game to the profiles folder
    files::copy_folder(&in_path, &installation_path.to_string_lossy().to_string());
    // get the game version
    let mut version = game::game_version(in_path.clone());
    if version == "Unknown" {
        version = "notset".to_string();
    }
    // update the profiles database
    let mut profiles = read_profiles(in_path.clone());
    profiles.copy_version = version;
    write_profiles(profiles);
    "success".to_string()
}

// reset the installation
fn reset_installation(in_path: String) -> String {
    let deletion = delete_installation();
    let init = init_installation(in_path);
    if deletion == "success" && init == "success" {
        return "success".to_string();
    }
    "failed".to_string()
}

// public-facing command for reading the profiles
#[command]
pub fn load_profiles(window: Window, in_path: String) {
    let profiles = read_profiles(in_path.clone());
    let profiles_str = serde_json::to_string(&profiles).unwrap();
    update_db(in_path); // pass ownership here..
    window.emit("profiles-loaded", Some(profiles_str)).unwrap();
}

// public-facing command for adding a profile
#[command]
pub fn add_profile(window: Window, in_path: String, profile_name: String) {
    let result = add_profile_db(in_path, profile_name);
    window.emit("profile-added", Some(result)).unwrap();
}

// public-facing command for removing a profile
#[command]
pub fn remove_profile(window: Window, in_path: String, profile_name: String) {
    let result = remove_profile_db(in_path, profile_name);
    window.emit("profile-removed", Some(result)).unwrap();
}

// public-facing command for resetting the profiles
#[command]
pub fn reset_profile(window: Window, in_path: String, reset_game_copy: bool) {
    init_profiles_db(in_path.clone());
    if reset_game_copy {
        let result = reset_installation(in_path.clone());
        window.emit("profile-reset", Some(result)).unwrap();
    } else {
        window.emit("profile-reset", Some("success".to_string())).unwrap();
    }
    window.emit("profiles-reset", Some("success".to_string())).unwrap();
}

// public-facing command for installing the game copy
#[command]
pub fn install_game_copy(window: Window, in_path: String) {
    let result = init_installation(in_path);
    window.emit("game-copy-installed", Some(result)).unwrap();
}

// public-facing command for deleting the game copy
#[command]
pub fn delete_game_copy(window: Window) {
    let result = delete_installation();
    window.emit("game-copy-deleted", Some(result)).unwrap();
}

// public-facing command for launching a specific profile
#[command]
pub fn launch_profile(window: Window, in_path: String, profile_name: String) {
    // to-do
}