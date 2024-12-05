use std::{fs, path::Path};
use tauri::{command, Window};
use crate::utils::files;
use crate::utils::game;
use crate::utils::rpgmaker;

// public facing game to rpg command for project.html
#[command]
pub fn export_rpg_project(window: Window, in_path: String, out_path: String) {
    window.emit("status", "Getting everything ready...").unwrap();
    // check if the input path is a valid game
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "Your input path was not a valid TCOAAL game..").unwrap();
        return;
    }
    // check if the output path is valid (if not, create it)
    if !Path::new(&out_path).exists() {
        fs::create_dir_all(&out_path).unwrap();
    }
    // and then make output out_path/exported_project (or whatever name is available)
    let out_path = files::verify_folder_multiple(&format!("{}/exported_project", out_path));
    // step one: copy the game's folder
    window.emit("status", "Copying your game's files..").unwrap();
    rpgmaker::copy_game(&in_path, &out_path);
    // step two: generate the RPG project file
    window.emit("status", "Generating your RPG project file..").unwrap();
    rpgmaker::generate_project(&out_path);
    // step three: update the package.json
    window.emit("status", "Updating your package.json..").unwrap();
    rpgmaker::update_package(&out_path);
    // step four: patch the index.html
    window.emit("status", "Applying patches to your game..").unwrap();
    rpgmaker::patch_index(&out_path);
    // done
    window.emit("status", "Your RPG Maker MV project has been created!").unwrap();
}