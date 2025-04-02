// imports
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use dirs::config_dir;
use tauri::{command, Window};
use crate::modmanager::installed;
use crate::utils::{helpers::files, operating::game};
use crate::modmanager::modloader;
use crate::utils::frontend::commands;

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
        name: profile_name.clone(),
        mods: vec![],
    };
    let installed_mods = installed::get_installed_mods(in_path.clone());
    let updated_profile = update_profile(new_profile, installed_mods);
    profiles.profiles.push(updated_profile);
    profiles.current = profile_name;
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
        if profiles.current == profile_name {
        profiles.current = "Default".to_string();
        }
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

// toggle the status of a mod in a profile in the db
fn toggle_mod_db(in_path: String, profile_name: String, mod_id: String) -> String {
    let mut profiles = read_profiles(in_path.clone());
    let profile = profiles.profiles.iter_mut().find(|p| p.name == profile_name);
    if let Some(profile) = profile {
        if let Some(mod_index) = profile.mods.iter().position(|m| m.id == mod_id) {
            profile.mods[mod_index].status = !profile.mods[mod_index].status;
            write_profiles(profiles);
            return "success".to_string();
        } else {
            return "mod_not_found".to_string(); 
        }
    }
    "profile_not_found".to_string() 
}

// update current profilt in db
fn update_current_profile(in_path: String, profile_name: String) -> String {
    let mut profiles = read_profiles(in_path.clone());
    if profiles.profiles.iter().any(|p| p.name == profile_name) {
        profiles.current = profile_name;
        write_profiles(profiles);
        return "success".to_string();
    }
    "notfound".to_string()
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
    // set the current version to notset
    let mut profiles = read_profiles("".to_string());
    profiles.copy_version = "notset".to_string();
    write_profiles(profiles);
    // and then delete the folder
    let installation_path = ProflesPath::new().game_copy();
    if installation_path.exists() {
        files::delete_folder(&installation_path.to_string_lossy().to_string());
        return "success".to_string();
    }
    "notfound".to_string()
}

// build the installation
async fn init_installation(in_path: String) -> String {
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
    // see if tomb is installed, and if not, install it
    let is_modded = modloader::modloader_prescence(installation_path.to_string_lossy().to_string());
    if !is_modded {
        let install_modloader = modloader::install_latest(installation_path.to_string_lossy().to_string()).await;
        if install_modloader != "success" {
            return install_modloader;
        }
    }
    // update the profiles database
    let mut profiles = read_profiles(in_path.clone());
    profiles.copy_version = version;
    write_profiles(profiles);
    "success".to_string()
}

// reset the installation
async fn reset_installation(in_path: String) -> String {
    let deletion = delete_installation();
    let init = init_installation(in_path).await;
    if deletion == "success" && init == "success" {
        return "success".to_string();
    }
    "failed".to_string()
}

// need to update?
fn compare_installation(in_path: String) -> String {
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        return "nogame".to_string();
    }
    let copy_ver = get_installation();
    if copy_ver == "notset" {
        return "notset".to_string();
    }
    let game_ver = game::game_version(in_path.clone());
    if game_ver == copy_ver {
        return "same".to_string();
    }
    "different".to_string()
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
pub async fn reset_profile(window: Window, in_path: String, reset_game_copy: bool) {
    init_profiles_db(in_path.clone());
    if reset_game_copy {
        let result = reset_installation(in_path.clone()).await;
        window.emit("profile-reset", Some(result)).unwrap();
    } else {
        window.emit("profile-reset", Some("success".to_string())).unwrap();
    }
    window.emit("profiles-reset", Some("success".to_string())).unwrap();
}

// public-facing command for installing the game copy
#[command]
pub async fn install_game_copy(window: Window, in_path: String) {
    let result = init_installation(in_path).await;
    window.emit("game-copy-installed", Some(result)).unwrap();
}

// public-facing command for deleting the game copy
#[command]
pub fn delete_game_copy(window: Window) {
    window.emit("status", "Deleting game copy...").unwrap();
    let result = delete_installation();
    window.emit("game-copy-deleted", Some(result)).unwrap();
    window.emit("status", "Game copy deleted! You can always remake it to use the Profiles feature again.").unwrap();
}

// public-facing command for getting the game copy version
#[command]
pub fn game_copy_version(window: Window) {
    let version = get_installation();
    window.emit("game-copy-version", Some(version)).unwrap();
}

// public-facing command for toggling a mod in a profile
#[command]
pub fn toggle_profile_mod(window: Window, in_path: String, profile_name: String, mod_id: String) {
    let result = toggle_mod_db(in_path, profile_name, mod_id);
    window.emit("profile-mod-toggled", Some(result)).unwrap();
}

// public-facing command for updating the current profile
#[command]
pub fn set_profile(window: Window, in_path: String, profile_name: String) {
    let result = update_current_profile(in_path, profile_name);
    window.emit("current-profile-updated", Some(result)).unwrap();
}

// public-facing command for comparing the installation
#[command]
pub fn compare_install(window: Window, in_path: String) {
    let result = compare_installation(in_path);
    window.emit("installation-compared", Some(result.clone())).unwrap();
    // switch statement to match result
    match result.as_str() {
        "nogame" => {
            window.emit("status", "Please set your TCOAAL game path in the settings to use the Profiles feature!").unwrap();
        }, "notset" => {
            window.emit("status", "Please set up your game copy to use the Profiles feature!").unwrap();
        }, "different" => {
            window.emit("status", "Your game copy is different from your game installation!").unwrap();
        }, "same" => {
            // nothing to see here..
        }, _ => {
            window.emit("status", "An error occurred while comparing your game copy to your game installation!").unwrap();
        }
    }
}

// public-facing command for initializing the installation
#[command]
pub async fn init_install(window: Window, in_path: String) {
    window.emit("status", "Copying game files to profiles folder...").unwrap();
    let result = init_installation(in_path).await;
    window.emit("installation-initialized", Some(result.clone())).unwrap();
    match result.as_str() {
        "success" => {
            window.emit("status", "The Profiles feature is ready to use now!").unwrap();
        }, "nogame" => {
            window.emit("status", "Please set your TCOAAL game path in the settings to use the Profiles feature!").unwrap();
        }, _ => {
            window.emit("status", "An error occurred while copying game files to the profiles folder!").unwrap();
        }
    }
}

// public-facing command for launching a specific profile
#[command]
pub fn launch_profile(window: Window, in_path: String, profile_name: String) {
    // verify that the game is installed
    window.emit("status", "Setting up your profile environment...").unwrap();
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "Please set your TCOAAL game path in the settings to use the Profiles feature!").unwrap();
        return;
    }
    // are we just launching the default profile?
    if profile_name == "Default" {
        commands::launch_game(window, in_path);
        return;
    }
    // read the profile
    let profiles = read_profiles(in_path.clone());
    let is_profile_set = get_installation();
    if is_profile_set == "notset" {
        window.emit("profile-launched", "notset").unwrap();
        return;
    }
    // clear out the mods folder in the profile (NOT main installation)
    let game_copy = ProflesPath::new().game_copy();
    let mods_folder = game_copy.join("tomb").join("mods");
    files::delete_folder(&mods_folder.to_string_lossy().to_string());
    files::verify_folder(&mods_folder).unwrap();
    // for each mod enabled in the profile, copy the folder to the mods folder
    window.emit("status", "Copying mods to the profile game copy...").unwrap();
    let profile = profiles.profiles.iter().find(|p| p.name == profile_name);
    if let Some(profile) = profile {
        for profile_mod in &profile.mods {
            if profile_mod.status {
                let mod_folder = game_copy.join("tomb").join("mods").join(&profile_mod.folder);
                let destination_folder = mods_folder.join(profile_mod.id.clone());
                files::verify_folder(&destination_folder).unwrap();
                files::copy_folder(&mod_folder.to_string_lossy().to_string(), &destination_folder.to_string_lossy().to_string());
            }
        }
    }
    window.emit("status", "Launching your profile!").unwrap();
    // launch the game
    commands::launch_game(window, game_copy.to_string_lossy().to_string());
}