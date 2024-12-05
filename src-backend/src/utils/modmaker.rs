// imports
use std::io;
use std::{fs, path::Path};
use json_patch::diff;
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use super::files;
use super::cipher;

// mod.json structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ModJSON {
    pub id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub version: String,
    pub dependencies: Dependencies,
    pub files: Files,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependencies {
    pub game: String,
    pub spec: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub assets: Vec<String>,
    pub data_deltas: Vec<String>,
    pub image_deltas: Vec<String>,
    pub plugins: Vec<String>,
    pub languages: Vec<String>,
}

// format the path to posix for tomb compatibility
fn win_to_posix(win_path: String) -> String {
    win_path.replace("\\", "/")
}

// get original file path in the game
fn get_game_path(in_path: &String, file_path: &String, game_path: &String) -> String {
    // get file_path - in_path, then add it to game_path - if it doesn't exist, it's new
    let relative_path = file_path.split(*&in_path).collect::<Vec<&str>>()[1].to_string();
    let new_output = format!("{}\\www{}", game_path, relative_path);
    // BUT! the original would be in k9a, so.. cut off the extension then add .k9a
    let new_output = new_output.split(".").collect::<Vec<&str>>()[0].to_string();
    let new_output = format!("{}.k9a", new_output);
    new_output
}

// convert an rpg maker path to a mod path
fn is_new_file(in_path: &String, file_path: &String, game_path: &String) -> bool {
    let new_output = get_game_path(in_path, file_path, game_path);
    !Path::new(&new_output).exists()
}

// parse a language file
fn parse_language_file(path_to_file: &Path) {
    let data = fs::read(path_to_file).unwrap();
}

// convert an rpg maker path to a mod path
fn format_mod_path(in_path: &String, file_path: &String, out_path: &String) -> String {
    let relative_path = file_path.split(in_path).collect::<Vec<&str>>()[1].to_string();
    let relative_path_no_ext = relative_path.split(".").collect::<Vec<&str>>()[0].to_string();
    let new_output = format!("{}\\{}.{}", out_path, relative_path_no_ext, "k9a");
    let path = Path::new(&new_output).parent().unwrap().to_str().unwrap();
    fs::create_dir_all(path).unwrap();
    new_output
}

// step one: generate difference between project data (aka the json files from rpg maker)
pub fn difference_data(in_path: &String, out_path: &String, game_path: &String, mut mod_json: ModJSON) -> ModJSON {
    // get all files (recursively) in in_path/data
    let files = files::collect_files_recursive(format!("{}\\data", in_path));
    for file in files {
        // only handle .json files
        if let Some(extension) = file.extension() {
            if extension != "json" {
                continue;
            }
        }
        // new or modified file
        let file_path_str = file.to_str().unwrap().to_string();
        let is_new = is_new_file(in_path, &file_path_str, &game_path);
        if is_new {
            fs::copy(file_path_str.clone(), format_mod_path(in_path, &file_path_str, out_path)).unwrap();
            mod_json.files.assets.push(win_to_posix(files::relative_path(in_path, &file_path_str)));
        } else {
            // is it even different? (warning: messy process..)
            let original_game_path = get_game_path(in_path, &file_path_str, &game_path);
            let original_encrypted = fs::read(original_game_path.clone()).unwrap();
            let original_decrypted = cipher::decrypt(&original_encrypted, &original_game_path);
            let original_content = String::from_utf8(original_decrypted).unwrap();
            let new_content = fs::read_to_string(file_path_str.clone()).unwrap();
            let original_json = serde_json::from_str(&original_content).unwrap();
            let new_json = serde_json::from_str(&new_content).unwrap();
            let diff = diff(&original_json, &new_json);
            if diff.is_empty() {
                continue;
            }
            // now get the relative path to push to mod.json
            let relative_path = files::relative_path(in_path, &file_path_str);
            let relative_pathbuf = Path::new(&relative_path);
            let relative_jsond = relative_pathbuf.with_extension("jsond");
            mod_json.files.data_deltas.push(win_to_posix(relative_jsond.to_str().unwrap().to_string()));
            // then write the patch
            let patch_str = to_string_pretty(&diff).unwrap();
            let patch_path = format_mod_path(in_path, &file_path_str, out_path);
            let patch_pathbuf = Path::new(&patch_path);
            let patch_jsond = patch_pathbuf.with_extension("jsond");
            fs::write(patch_jsond, patch_str).unwrap();
        }
    }
    mod_json
}

// step two: generate difference between project languages
pub fn difference_languages(in_path: &String, out_path: &String, game_path: &String, mut mod_json: ModJSON) -> ModJSON {
    mod_json
}

pub fn project_to_mod(in_path: &String, out_path: &String, game_path: &String) {
    // make empty mod.json
    let mod_json = ModJSON {
        id: "example_mod".to_string(),
        name: "Example Mod".to_string(),
        authors: vec!["Your name!".to_string()],
        description: "An example mod.".to_string(),
        version: "1.0.0".to_string(),
        dependencies: Dependencies {
            game: "*".to_string(),
            spec: "0.1.0".to_string(),
        },
        files: Files {
            assets: vec![],
            data_deltas: vec![],
            image_deltas: vec![],
            plugins: vec![],
            languages: vec![],
        },
    };
    // generate differences
    let mod_json = difference_data(&in_path, &out_path, &game_path, mod_json);
    // print it out for testing
    println!("{:?}", mod_json);
}