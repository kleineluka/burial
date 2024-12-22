use std::{fs, path::Path};
use serde_json::to_string_pretty;
use tauri::{command, Window};
use crate::utils::compression;
use crate::utils::files;
use crate::utils::game;
use crate::utils::modmaker;
use crate::utils::rpgmaker;

// public facing game to rpg command for project.html
#[command]
pub fn export_rpg_project(window: Window, in_path: String, out_path: String, project_name: String) {
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
    let out_path = files::verify_folder_multiple(&format!("{}\\{}", out_path, project_name));
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

// public facing rpg to mod command for project.html
#[command]
pub fn export_mod_folder(window: Window, in_path: String, game_path: String, out_path: String, 
    folder_name: String, auto_zip: bool, mod_name: String, mod_id: String, mod_author: String, 
    mod_description: String, mod_version: String) {
    window.emit("status", "Getting everything ready...").unwrap();
    // make sure that the input folder is an rpg project
    let is_rpg = modmaker::verify_rpg_project(&in_path);
    if !is_rpg {
        window.emit("error", "Your input path was not a valid RPG Maker MV project..").unwrap();
        return;
    }
    // ensure that the game path is valid
    let is_game = game::verify_game(&game_path).unwrap();
    if !is_game {
        window.emit("error", "Your game path is not a valid TCOAAL installation.").unwrap();
        return;
    }
    // check if the output path is valid (if not, create it)
    if !Path::new(&out_path).exists() {
        fs::create_dir_all(&out_path).unwrap();
    }
    // mod out path = out_path + folder_name.. rebuild if it already exists
    let mod_out_path = files::verify_folder_multiple(&format!("{}\\{}", out_path, folder_name));
    if Path::new(&mod_out_path).exists() {
        fs::remove_dir_all(&mod_out_path).unwrap();
    }
    if !Path::new(&mod_out_path).exists() {
        fs::create_dir_all(&mod_out_path).unwrap();
    }
    // get the game version, but if can't, leave as *
    window.emit("status", "Getting your game's version..").unwrap();
    let game_version = if game::game_version(game_path.clone()) == "Unknown" {
        "*".to_string()
    } else {
        game::game_version(game_path.clone())
    };
    // construct the json
    let mod_json = modmaker::ModJSON {
        id: mod_id,
        name: mod_name,
        authors: vec![mod_author],
        description: mod_description,
        version: mod_version,
        dependencies: modmaker::Dependencies {
            game: game_version,
            spec: modmaker::MOD_JSON_SPEC.to_string(),
        },
        files: modmaker::Files {
            assets: vec![],
            data_deltas: vec![],
            image_deltas: vec![],
            plugins: vec![],
            languages: vec![],
        },
    };
    // generate differences
    window.emit("status", "Finding differences between data files..").unwrap();
    let mut mod_json = modmaker::difference_data(&in_path, &mod_out_path, &game_path, mod_json);
    window.emit("status", "Finding differences between language files..").unwrap();
    mod_json = modmaker::difference_languages(&in_path, &mod_out_path, &game_path, mod_json);
    window.emit("status", "Finding differences between image files..").unwrap();
    mod_json = modmaker::difference_images(&in_path, &mod_out_path, &game_path, mod_json);
    window.emit("status", "Finding differences between audio files..").unwrap();
    mod_json = modmaker::difference_audio(&in_path, &mod_out_path, &game_path, mod_json);
    window.emit("status", "Finding differences between video files..").unwrap();
    mod_json = modmaker::difference_videos(&in_path, &mod_out_path, &game_path, mod_json);
    window.emit("status", "Finding differences between plugin files..").unwrap();
    mod_json = modmaker::difference_plugins(&in_path, &mod_out_path, &game_path, mod_json);
    window.emit("status", "Sanitizing your mod.json..").unwrap();
    mod_json = modmaker::sanitize_json(mod_json);
    // in the mod output folder, write the mod.json
    window.emit("status", "Writing your mod.json..").unwrap();
    let mod_json_path = format!("{}\\mod.json", mod_out_path);
    let mod_json_str = to_string_pretty(&mod_json).unwrap();
    fs::write(mod_json_path, mod_json_str).unwrap();
    // if auto_zip is true, zip the folder
    if auto_zip {
        window.emit("status", "Zipping your mod folder..").unwrap();
        let zip_path = format!("{}\\{}.zip", out_path, folder_name);
        let zip_file = fs::File::create(&zip_path).unwrap();
        compression::compress_directory(Path::new(&mod_out_path), zip_file).unwrap();
        fs::remove_dir_all(&mod_out_path).unwrap();
    }
}