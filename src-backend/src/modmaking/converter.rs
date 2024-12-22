// imports
use serde::{Deserialize, Serialize};
use tauri::{command, Window};
use crate::config::cache;
use crate::utils::game;
use crate::utils::rpgmaker;
use crate::utils::modmaker;
use crate::utils::files;
use crate::utils::compression;

#[derive(Serialize, Deserialize, Debug)]
pub struct Burial {
    pub is_tomb: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub last_update: String, 
    pub url: String,
    pub source: String,
    pub sha256: String,
    pub id: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependencies {
    pub game: String,
    pub spec: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModJson {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub version: String,
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForeignRepo {
    pub burial: Burial,
    pub data: Data,
    pub mod_json: ModJson,
}

// act differently based on what kind of mod is provided
fn mod_file_type(in_path: &String) -> String {
    // either a folder or .zip, if it exists
    let path = std::path::Path::new(in_path);
    if path.is_dir() {
        return "folder".to_string();
    } else if path.is_file() {
        return "zip".to_string();
    } else {
        return "na".to_string();
    }
}

// act differently based on how mod is structured
fn mod_directory_type(in_path: &String) -> String {
    // see if in the current directory, there is a www directory
    let path = std::path::Path::new(in_path);
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.ends_with("www") {
            return "www".to_string();
        }
    }
    // assume main directory has what we need (until i make a unified way to check..)
    "main".to_string()
}

// convert a mod that doesn't use tomb to use tomb
pub fn convert_to_tomb(in_path: String, game_path: String, out_path: String,
    mod_name: String, mod_id: String, mod_authors: Vec<String>, 
    mod_description: String, mod_version: String) -> String {
    // verify the game installation is valid
    let is_game = game::verify_game(&game_path).unwrap();
    if !is_game {
        // note: string not error for easier front-end passing (applies to others)  
        return "error:game_path".to_string(); 
    }
    // verify that the mod is valid (either a .zip or a folder)
    let mod_type = mod_file_type(&in_path);
    if mod_type == "na" {
        return "error:mod_path".to_string();  
    }
    // set up a temp folder in cache
    let mod_temp_folder = cache::create_temp_with_name(&mod_id);
    // branch based on the mod type
    if mod_type == "zip" {
        // extract the contents of the zip to the temp folder
        let source_path = std::path::Path::new(&in_path);
        let destination_path = std::path::Path::new(&mod_temp_folder);
        let _ = compression::decompress_directory_nosub(&source_path, &destination_path).unwrap();
    } else if mod_type == "folder" {
        // copy the contents of the folder to the temp folder
        let _ = files::copy_directory(&in_path, mod_temp_folder.to_str().unwrap()).unwrap();
    }
    // next, we need to generate an rpg project 
    let project_name = format!("{}_{}", mod_id, "project");
    let project_temp_folder = cache::create_temp_with_name(&project_name);
    let _rpg_project = rpgmaker::game_to_rpg(game_path.clone(), project_temp_folder.to_str().unwrap().to_string());
    // merge files from the mod into the rpg project (to do: clean up typing here..)
    let folder_type = mod_directory_type(&mod_temp_folder.to_str().unwrap().to_string());
    let folder_to_copy = match folder_type.as_str() {
        "www" => format!("{}/www", mod_temp_folder.display()),
        _ => mod_temp_folder.to_str().unwrap().to_string(),
    };
    let folder_to_copy_path = std::path::Path::new(&folder_to_copy);
    let _ = files::merge_directories(folder_to_copy_path, &project_temp_folder).unwrap();
    // delete the temp mod folder
    let _ = files::delete_folder(&mod_temp_folder.to_str().unwrap());
    // finally, convert it to a mod
    let mod_path = cache::create_temp_with_name(&format!("{}_tomb_mod", mod_id));
    let mod_authors_str = mod_authors.join(", "); // to do: support multiple authors
    let _ = modmaker::project_to_mod(&project_temp_folder.to_str().unwrap().to_string(), &mod_path.to_str().unwrap().to_string(), &game_path, &mod_name, &mod_id, &mod_authors_str, &mod_description, &mod_version);
    // move the mod to the out path (make it if it doesn't exist)
    let out_path = std::path::Path::new(&out_path);
    let _ = files::validate_path(out_path.to_str().unwrap());
    // make the output path + mod id folder
    let out_path = out_path.join(format!("{}_tomb_mod", mod_id));
    let _ = files::copy_directory(&mod_path.to_str().unwrap(), out_path.to_str().unwrap());
    // clear temp
    let _ = cache::clear_temp();
    // return folder name
    return format!("{}_tomb_mod", mod_id);
}

#[command]
pub fn convert_mod(window: Window, in_path: String, game_path: String, out_path: String,
    mod_name: String, mod_id: String, mod_authors: String, mod_description: String, mod_version: String) {
    window.emit("status", "Converting your mod.. please be patient, this may take a bit!").unwrap();
    let conversion_result = convert_to_tomb(in_path, game_path, out_path, mod_name, mod_id, vec![mod_authors], mod_description, mod_version);
    // switch based on conversion result
    match conversion_result.as_str() {
        "error:game_path" => {
            window.emit("status", "Error: Game path is invalid.").unwrap();
        },
        "error:mod_path" => {
            window.emit("status", "Error: Mod path is invalid.").unwrap();
        },
        _ => {
            let status = format!("Conversion successful! Your mod can be found at {}", conversion_result);
            window.emit("status", &status).unwrap();
        }
    }
}