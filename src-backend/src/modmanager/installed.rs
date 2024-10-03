// imports
use tauri::Window;
use tauri::command;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

// structure containing mod folder and mod.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct ModFolder {
    pub folder: String,
    pub modjson: ModJson,
}

// structure containing mod information (keep in mind some might miss some fields.. sigh)
#[derive(Serialize, Deserialize, Debug)]
pub struct ModJson {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(flatten)] // for other properties we don't need when deserializing
    pub extra: Option<HashMap<String, serde_json::Value>>,
}     

// get all of the installed mods
#[command]
pub fn get_installed_mods(window: Window, in_path: String) {
    // mods will be present: in_path + www + mods and there each folder will be a mod, and inside that folder, a mod.json file
    let mods_path = format!("{}/www/mods", in_path);
    let mut mods: Vec<ModFolder> = Vec::new();
    // read all the folders in the mods folder
    for entry in std::fs::read_dir(mods_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // check if the entry is a folder
        if path.is_dir() {
            // read the mod.json file
            let modjson_path = format!("{}/mod.json", path.display());
            // safely attempt to read the mod.json file
            if let Ok(modjson_raw) = std::fs::read_to_string(modjson_path) {
                // parse the JSON, ignoring extra fields
                if let Ok(modjson) = serde_json::from_str::<ModJson>(&modjson_raw) {
                    mods.push(ModFolder {
                        folder: path.display().to_string(),
                        modjson,
                    });
                } else {
                    eprintln!("Error parsing mod.json in folder: {:?}", path);
                }
            } else {
                eprintln!("Could not read mod.json in folder: {:?}", path);
            }
        }
    }
    // print out the mods
    println!("{:?}", mods);
}