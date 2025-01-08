use std::fs;
// imports
use std::path::Path;
use chrono::format;
use serde::{Deserialize, Serialize};
use tauri::Window;
use crate::modmaking::converter;
use crate::config::{cache, downloads};
use crate::utils::emitter::EventEmitter;
use crate::utils::game;
use crate::modmanager::modloader;

// mod type enum
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ModSource {
    LLamaware, // assign-only for now
    Gamebanana,
    ZipUrl,
    Github,
    Unsupported,
}

impl ModSource {
    pub fn from_url(url: &str) -> Self {
        if url.contains("gamebanana.com") {
            ModSource::Gamebanana
        } else if url.contains("github.com") { // even for .zip on github, prioritize this method..
            ModSource::Github
        } else if url.ends_with(".zip") {
            ModSource::ZipUrl
        } else {
            ModSource::Unsupported
        }
    }
}

// identify what kind of mod is being installed from the folder. the cases we have are:
// - contains a "tomb" folder (aka top-level)
// - contains a "mods" folder (mid-level, inbetween game folder and mod)
// - contains a mod.json (bottom-level, mod itself and all contents are here)
#[derive(Debug, PartialEq, Eq)]
pub enum ModType {
    TopLevel,
    UpperMidLevel,
    LowerMidLevel,
    BottomLevel,
    Unknown,
}

// keep a relative path related to the modtype handy
impl ModType {
    pub fn relative_path(&self) -> &'static str {
        match self {
            ModType::TopLevel => "tomb/mods/%mod_name%",
            ModType::UpperMidLevel => "mods/%mod_name%",
            ModType::LowerMidLevel => "%mod_name%",
            ModType::BottomLevel => "",
            ModType::Unknown => "",
        }
    }

    pub fn formatted_path(&self) -> String {
        // get where we will look, don't bother with bottom level paths
        let mut upper_level = self.relative_path().to_string();
        if (upper_level.is_empty()) {
            return upper_level.to_string();
        }
        upper_level = upper_level.replace("%mod_name%", "");
        // upper_path has two folders: MyMod1 and tomb. Find the first folder that is not tomb and get the name of the folder and save it as mod_name
        let upper_path = upper_level.split("/").collect::<Vec<&str>>();
        let mut mod_name = "";
        for (_i, folder) in upper_path.iter().enumerate() {
            if *folder != "tomb" {
                mod_name = folder;
                break;
            }
        }
        // get the path of the mod
        let final_format = self.relative_path().replace("%mod_name%", mod_name);
        final_format
    }

}

// make sure to note any special circumstances
#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialCases {
    pub special_cases: Vec<SpecialCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialCase {
    pub special_case: Conditions,
    pub fatal_issue: bool,
    pub error_message: String,
}

// - tomb_only: the user is installing tomb itself
// - burial_only: the user is installing burial itself
// - contains_tomb: the mod has tomb inside of it
// - unsafe_mod: the mod contains an executable file
// - not_tomb: the mod is not for tomb
// - unsupported: we just can't work with it (sowwy!)
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Conditions {
    TombOnly,
    BurialOnly,
    ContainsTomb,
    UnsafeMod,
    NotTomb,
    Unsupported,
}
// sanitize a mod folder name (a-Z, 0-9, " " = _)
pub fn sanitize_mod_folder_name(name: &str) -> String {
    // remove file extension (maybe!)
    let name = name.split('.').next().unwrap_or(name);
    // standard filtering
    let mut sanitized_name = name.replace(" ", "_");
    sanitized_name.retain(|c| c.is_alphanumeric() || c == '_');
    // remove numbers (if safe to do so)
    let without_numbers: String = sanitized_name.chars().filter(|c| !c.is_numeric()).collect();
    if without_numbers.is_empty() {
        sanitized_name
    } else {
        without_numbers
    }
}

// get what kind of mod a folder is
pub fn get_mod_type(in_path: String) -> ModType {
    let path = Path::new(&in_path);
    // check for tomb folder
    if path.join("tomb").is_dir() {
        return ModType::TopLevel;
    }
    // check for mods folder
    if path.join("mods").is_dir() {
        return ModType::UpperMidLevel;
    }
    // if there is a ONLY a single folder in the directory
    if path.read_dir().unwrap().count() == 1 {
        return ModType::LowerMidLevel;
    }
    // check for mod.json
    if path.join("mod.json").is_file() {
        return ModType::BottomLevel;
    }
    // check for "www"
    if path.join("www").is_dir() {
        return ModType::BottomLevel;
    }
    // or else.. treat it like we don't know
    ModType::Unknown
}

// determine if a mod has any issues
pub fn get_mod_issues(in_path: String, mod_type: ModType) -> SpecialCases {
    let mut special_cases = SpecialCases {
        special_cases: Vec::new(),
    };
    // we have two cases where we can see the user just downloaded tomb
    // - if the user used gamebanana, that just has a file called "Gex2Cover.jpg" in it
    // - if the user used codeberg, in which case we have "index.html", "tomb", and "mods" folder
    if Path::new(&in_path).join("Gex2Cover.jpg").is_file() {
        special_cases.special_cases.push(SpecialCase {
            special_case: Conditions::TombOnly,
            fatal_issue: true,
            error_message: "Please download Tomb inside of the Modloader 🪦 tab instead!".to_string(),
        });
    }
    if Path::new(&in_path).join("index.html").is_file() {
        if Path::new(&in_path).join("tomb").is_dir() && Path::new(&in_path).join("mods").is_dir() {
            special_cases.special_cases.push(SpecialCase {
                special_case: Conditions::TombOnly,
                fatal_issue: true,
                error_message: "Please download Tomb inside of the Modloader 🪦 tab instead!".to_string(),
            });
        }
    }
    // see if it contains Burial-Installer.exe, and if so, it's Burial itself
    if Path::new(&in_path).join("Burial-Installer.exe").is_file() {	
        special_cases.special_cases.push(SpecialCase {
            special_case: Conditions::BurialOnly,
            fatal_issue: true,
            error_message: "Please update Burial from the Github or other official links in the Burial Links 🔗 tab inside of the Knowledge 📔 section!".to_string(),
        });
    }
    // if in the formatted path, see if there is a tomb folder
    if mod_type.formatted_path().contains("tomb") {
        special_cases.special_cases.push(SpecialCase {
            special_case: Conditions::ContainsTomb,
            fatal_issue: false,
            error_message: "This mod contains a tomb folder and will be automatically stripped of it when installing (in favour of your tomb version)..".to_string(),
        });
    }
    // go through EVERY file in EVERY folder and see if there is an executable file
    for entry in Path::new(&in_path).read_dir().unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            if entry.path().extension().unwrap() == "exe" {
                special_cases.special_cases.push(SpecialCase {
                    special_case: Conditions::UnsafeMod,
                    fatal_issue: true,
                    error_message: "This mod contains an executable file. This is not allowed for security reasons.".to_string(),
                });
            }
        }
    }
    // see if the folder just contains a "www" folder, which is a sign it's not for tomb
    if Path::new(&in_path).join("www").is_dir() {
        special_cases.special_cases.push(SpecialCase {
            special_case: Conditions::NotTomb,
            fatal_issue: false,
            error_message: "This mod is not for Tomb and will be automatically re-compiled by Burial - there may be issues. Please reach out to the mod author for a native Tomb port!".to_string(),
        });
    }
    special_cases
}

// get the folder name of the mod to use (w/ sanitization)
fn get_mod_name(in_path: String) -> String {
    let path = Path::new(&in_path);
    let folder_name = path.file_name().unwrap().to_str().unwrap();
    let sanitized_name = folder_name.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '_').collect::<String>()
        .replace("extracted", "")
        .replace("_", " ")
        .trim_end_matches('_') // remove trailing _
        .to_string(); // (probably the stupidest daisy chain i've wrote)
    sanitized_name
}

// based on a mod name, get the mod id
fn get_mod_id(mod_path: String) -> String {
    let mod_name = get_mod_name(mod_path.clone());
    let mod_id = mod_name.to_lowercase()
        .replace(" ", "_")
        .trim_end_matches('_') // remove trailing _
        .to_string();
    mod_id
}

// install a standalone mod from a folder
// note: don't pass emitter to keep it optional and simple..
pub fn install_generic(window: Option<&Window>, in_path: String, mod_path: String) -> String {
    // verify the game path 
    let emitter = EventEmitter::new(window);
    emitter.emit("status", "Getting everything set up..");
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        emitter.emit("error", "Please set your game path in the settings first!");
        return "nogame".to_string();
    }
    // verify that tomb is installed
    let is_tomb = modloader::modloader_prescence(in_path.clone());
    if !is_tomb {
        emitter.emit("error", "Please install Tomb inside of the Modloader 🪦 tab first!");
        return "notomb".to_string();
    }
    // first, get the type of mod and the formatted path
    let mod_type = get_mod_type(mod_path.clone());
    if mod_type == ModType::Unknown {
        emitter.emit("error", "This mod is not supported. Please ask the developers for a Tomb native version!");
        return "unsupported".to_string();
    }
    let formatted_path = mod_type.formatted_path();
    // get any issues with the mod
    emitter.emit("status", "Checking the mod for any issues..");
    let issues = get_mod_issues(mod_path.clone(), mod_type);
    // CASE: TombOnly~ we can delete the mod folder and return the message
    if issues.special_cases.iter().any(|x| x.special_case == Conditions::TombOnly) {
        std::fs::remove_dir_all(&mod_path).unwrap();
        emitter.emit("error", &issues.special_cases.iter().find(|x| x.special_case == Conditions::TombOnly).unwrap().error_message.clone());
        return issues.special_cases.iter().find(|x| x.special_case == Conditions::TombOnly).unwrap().error_message.clone();
    }
    // CASE: BurialOnly~ we can delete the mod folder and return the message
    if issues.special_cases.iter().any(|x| x.special_case == Conditions::BurialOnly) {
        std::fs::remove_dir_all(&mod_path).unwrap();
        emitter.emit("error", &issues.special_cases.iter().find(|x| x.special_case == Conditions::BurialOnly).unwrap().error_message.clone());
        return issues.special_cases.iter().find(|x| x.special_case == Conditions::BurialOnly).unwrap().error_message.clone();
    }
    // CASE: UnsafeMod~ we can delete the mod folder and return the message
    if issues.special_cases.iter().any(|x| x.special_case == Conditions::UnsafeMod) {
        std::fs::remove_dir_all(&mod_path).unwrap();
        emitter.emit("error", &issues.special_cases.iter().find(|x| x.special_case == Conditions::UnsafeMod).unwrap().error_message.clone());
        return issues.special_cases.iter().find(|x| x.special_case == Conditions::UnsafeMod).unwrap().error_message.clone();
    }
    // CASE: NotTomb~ we need to convert the mod folder first !
    let mut working_mod_path = mod_path.clone();
    if formatted_path != "" {
        working_mod_path = Path::new(&mod_path).join(formatted_path).to_str().unwrap().to_string();
    }
    let mod_name = get_mod_name(working_mod_path.clone());
    let mod_id = get_mod_id(working_mod_path.clone());
    if issues.special_cases.iter().any(|x| x.special_case == Conditions::NotTomb) {
        emitter.emit("status", "Compiling the mod..");
        let mod_authors = vec![format!("{} Creator(s)", mod_name)];
        let mod_desscription = format!("For a better experience, reach out to the creators of {} for a native Tomb port!", mod_name.clone());
        let mod_version = "1.0.0".to_string();
        let out_path = downloads::downloads_folder();
        let conversion_result = converter::convert_to_tomb(mod_path.clone(), in_path.clone(), out_path.to_str().unwrap().to_string(), mod_name.clone(), mod_id.clone(), mod_authors, mod_desscription, mod_version);
        let tomb_mod_path = Path::new(&out_path).join(conversion_result);
        working_mod_path = tomb_mod_path.to_str().unwrap().to_string();
        fs::remove_dir_all(&mod_path).unwrap();
    }
    // now, after converting the mod or if we are just doing a normal installation, install it!
    emitter.emit("status", "Installing the mod..");
    let game_mods_folder = Path::new(&in_path.clone()).join("tomb").join("mods");
    let final_mod_path = game_mods_folder.join(mod_id.clone());
    std::fs::create_dir_all(&final_mod_path).map_err(|e| println!("Failed to create directory: {:?}", e)).ok();
    // copy all files inside working mod path to final mod path
    emitter.emit("status", "Moving the mod files..");
    for entry in fs::read_dir(&working_mod_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let new_path = final_mod_path.join(path.file_name().unwrap());
        fs::rename(path, new_path).map_err(|e| println!("Failed to move file: {:?}", e)).ok();
    }
    // delete the original mod folder
    emitter.emit("status", "Cleaning up..");
    std::fs::remove_dir_all(&working_mod_path).unwrap();
    downloads::clear_downloads().unwrap();
    "success".to_string()
}