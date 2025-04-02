// imports
use std::fs;
use std::path::Path;
use tauri::{command, Window};
use crate::utils::frontend::emitter::EventEmitter;
use crate::utils::operating::game;
use crate::utils::tomb::rpgmaker;
use crate::utils::helpers::files;

// use game + rpg maker export to decompile a mod
pub async fn decompile_mod_to_project(window: Option<&Window>, in_path: String, mod_path: String, out_path: String) -> String {
    let emitter = EventEmitter::new(window);
    emitter.emit("status", "Setting everything up..");
    // first, see if the game path is valid
    let game_path = game::verify_game(&in_path).unwrap();
    if !game_path {
        emitter.emit("error", "Please set a valid TCOAAL path in the settings page first!");
        return "nogame".to_string();
    }
    // then, see if the mod is a valid folder
    let mod_folder_valid = fs::metadata(&mod_path).is_ok();
    if !mod_folder_valid {
        emitter.emit("error", "The mod folder is invalid. It should either have a www folder or a mod.json file inside it.");
        return "nomod".to_string();
    }
    // it should have a mod.json file or a www folder inside it
    emitter.emit("status", "Checking the mod folder..");
    let mod_json = Path::new(&mod_path).join("mod.json");
    let mod_json_valid = fs::metadata(&mod_json).is_ok();
    let www_folder = Path::new(&mod_path).join("www");
    let www_folder_valid = fs::metadata(&www_folder).is_ok();
    if !mod_json_valid && !www_folder_valid {
        emitter.emit("error", "The mod folder is invalid. It should either have a www folder or a mod.json file inside it.");
        return "nomod".to_string();
    }
    // generate an rpg maker project
    emitter.emit("status", "Decompiling the mod..");
    let mod_folder_name = Path::new(&mod_path).file_name().unwrap().to_str().unwrap().to_string();
    let rpgmaker_path = Path::new(&out_path).join(format!("{}-decompiled", mod_folder_name));
    let _rpgmaker = rpgmaker::game_to_rpg(in_path.clone(), rpgmaker_path.to_str().unwrap().to_string());
    // determine how to proceed based on the mod type
    let is_tomb = mod_json_valid;
    if is_tomb {
        // everything in folder except mod.json we merge
        files::copy_directory(&mod_path, &rpgmaker_path.to_str().unwrap()).unwrap();
    } else {
        // copy everything in the www folder
        let www_folder = Path::new(&mod_path).join("www");
        files::copy_directory(&www_folder.to_str().unwrap(), rpgmaker_path.to_str().unwrap()).unwrap();
    }
    emitter.emit("status", "Decompilation all done!");
    return "success".to_string();
}

// decompiler wrapper
#[command]
pub async fn decompile_mod(window: Window, in_path: String, mod_path: String, out_path: String) {
    let _result = decompile_mod_to_project(Some(&window), in_path, mod_path, out_path).await;
}