// imports
use std::collections::HashMap;
use std::{fs, path::Path};
use json_patch::diff;
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use super::files;
use super::cipher;
use super::nemlang;
use super::olid;
use super::game;
use crate::reversing::info;

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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "dataDeltas")]
    pub data_deltas: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "imageDeltas")]
    pub image_deltas: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Plugin {
    name: String,
    status: bool,
    parameters: std::collections::HashMap<String, String>,
}

// constants
pub const MOD_JSON_SPEC: &str = "0.1.0";

// format the path to posix for tomb compatibility (+ if it starts with slash, remove)
fn win_to_posix(win_path: String) -> String {
    let new_path = win_path.replace("\\", "/");
    if new_path.starts_with("/") {
        new_path[1..].to_string()
    } else {
        new_path
    }
}

// get original file path in the game
fn get_game_path(in_path: &String, file_path: &String, game_path: &String) -> String {
    // get file_path - in_path, then add it to game_path - if it doesn't exist, it's new
    let relative_path = file_path.split(*&in_path).collect::<Vec<&str>>()[1].to_string();
    let new_output = format!("{}/www{}", game_path, relative_path);
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

// convert an rpg maker path to a mod path
fn format_mod_path(in_path: &String, file_path: &String, out_path: &String) -> String {
    let relative_path = file_path.split(in_path).collect::<Vec<&str>>()[1].to_string();
    let new_output = format!("{}{}", out_path, relative_path);
    let path = Path::new(&new_output).parent().unwrap().to_str().unwrap();
    fs::create_dir_all(path).unwrap();
    new_output
}

// relative language path
fn relative_language_path(file_path: &String) -> String {
    let path = Path::new(&file_path);
    let folder = path.parent().unwrap().file_name().unwrap().to_str().unwrap();
    let new_output = format!("languages/{}.json", folder);
    new_output
}

// convert a project language path to a tomb language path
fn format_language_path(file_path: &String, out_path: &String) -> String {
    let path = Path::new(&file_path);
    let folder = path.parent().unwrap().file_name().unwrap().to_str().unwrap();
    let new_output = format!("{}/languages/{}.json", out_path, folder);
    new_output
}

// helper to clear mod json fields at end stage..
fn clear_if_empty<T>(vec: &mut Vec<T>) {
    if vec.is_empty() {
        vec.clear();
    }
}

// decrypt a file and then compare it to another file (returns: different check, game bytes decrypted, new bytes)
fn read_decrypt_compare(file_path: &String, game_path: &String) -> (bool, Vec<u8>, Vec<u8>) {
    // decrypt original game asset
    if !Path::new(&game_path).exists() || !Path::new(&file_path).exists() {
        return (false, vec![], vec![]);
    }
    let original_encrypted = fs::read(game_path).unwrap();
    let original_decrypted = cipher::decrypt(&original_encrypted, &game_path);
    let new_content = fs::read(file_path.clone()).unwrap();
    // compare blake3 hashes
    let original_hash = files::get_blake3_bytes(&original_decrypted);
    let new_hash = files::get_blake3_bytes(&new_content);
    let are_different = original_hash != new_hash;  // false = same, true = different
    (are_different, original_decrypted, new_content)
}

// verify that a folder contains an rpg maker project
pub fn verify_rpg_project(in_path: &String) -> bool {
    // check if the input folder exists
    if !Path::new(&in_path).exists() {
        return false;
    }
    // see if in the folder, the file Game.rpgproject exists
    let game_path = format!("{}/Game.rpgproject", in_path);
    if !Path::new(&game_path).exists() {
        return false;
    }
    true
}

// hardcode parameters into a plugin
fn hardcore_parameters(in_plugin: &Plugin, plugin_text: &String) -> String {
    let parameters_json = serde_json::to_string(&in_plugin.parameters).unwrap();
    let mut modified_text = plugin_text.clone();
    let find = format!("var parameters = PluginManager.parameters('{}');", in_plugin.name);
    let replace = format!("var parameters = {};", parameters_json);
    modified_text = modified_text.replace(&find, &replace);
    modified_text
}

// step one: generate difference between project data (aka the json files from rpg maker)
pub fn difference_data(in_path: &String, out_path: &String, game_path: &String, mut mod_json: ModJSON) -> ModJSON {
    // get all files (recursively) in in_path/data
    let files = files::collect_files_recursive(format!("{}/data", in_path));
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
    // what sections we are looking for changes in
    let game_path = format!("{}/www", game_path);
    let lang_folders = format!("{}/languages", in_path);
    let lang_files = files::collect_files_recursive(lang_folders.clone());
    // go through every entry 
    for file in lang_files {
        let file_path_str = file.to_str().unwrap().to_string();
        let file_path = Path::new(&file_path_str);
        let ext = file_path.extension().unwrap().to_str().unwrap();
        // skip if not .loc, .txt, or .csv
        if ext != "loc" && ext != "txt" && ext != "csv" {
            continue;
        }
        let relative_path = files::relative_path(&in_path, &file_path_str);
        let relative_language = relative_language_path(&file_path_str);
        let project_path = format!("{}{}", &in_path, relative_path);
        // if it's new, copy it over
        let game_loc = format!("{}{}", game_path, relative_path);
        let is_new = !Path::new(&game_loc).exists();
        if is_new {
            fs::copy(file_path_str.clone(), format_language_path(&file_path_str, out_path)).unwrap();
            mod_json.files.languages.push(win_to_posix(relative_language));
            continue;
        }
        // read both files and compare
        let game_lang = nemlang::load_nemlang(&game_loc).unwrap();
        let project_lang = nemlang::load_nemlang(&project_path).unwrap();
        // parse them to JSON
        let game_json = serde_json::to_value(&game_lang).unwrap();
        let project_json = serde_json::to_value(&project_lang).unwrap();
        // find differences
        let mut diff: HashMap<String, HashMap<String, Value>> = HashMap::new();
        let sections = vec![
            "sysLabel",
            "sysMenus",
            "labelLUT",
            "linesLUT",
        ];
        for section in sections {
            let game_section = game_json.get(section).unwrap().as_object().unwrap();
            let project_section = project_json.get(section).unwrap().as_object().unwrap();
            let mut section_diff = HashMap::new();
            for (key, project_value) in project_section {
                match game_section.get(key) {
                    Some(game_value) => {
                        if game_value != project_value {
                            section_diff.insert(key.clone(), project_value.clone());
                        }
                    }
                    None => {
                        section_diff.insert(key.clone(), project_value.clone());
                    }
                }
            }   
            if !section_diff.is_empty() {
                diff.insert(section.to_string(), section_diff);
            }
        }
        // make sure there are differences
        if diff.is_empty() {
            continue;
        }
        // push the relative path then write the patch (don't need to do jsond stuff here)
        mod_json.files.languages.push(win_to_posix(relative_language));
        let patch_str = to_string_pretty(&diff).unwrap();
        let patch_path = format_language_path(&file_path_str, out_path);
        let patch_pathbuf = Path::new(&patch_path);
        // make sure directory exists
        let patch_dir = patch_pathbuf.parent().unwrap().to_str().unwrap();
        fs::create_dir_all(patch_dir).unwrap();
        fs::write(patch_pathbuf, patch_str).unwrap();
    }
    mod_json
}

// step three: generate difference between project images
pub fn difference_images(in_path: &String, out_path: &String, game_path: &String, mut mod_json: ModJSON) -> ModJSON {
    // first, see if images even exist
    let images_path = format!("{}/img", in_path);
    if !Path::new(&images_path).exists() {
        return mod_json;
    }
    let files = files::collect_files_recursive(images_path);
    if files.is_empty() {
        return mod_json;
    }
    // what folders we will be working with
    let folders = [
        "animations", "battlebacks1", "battlebacks2", "characters", "enemies", "faces", "parallaxes",
        "pictures", "sv_actors", "sv_enemies", "system", "tilesets", "titles1", "titles2"
    ];
    // go through every entry
    for file in files {
        // ensure it is a .png file
        if let Some(extension) = file.extension() {
            if extension != "png" {
                continue;
            }
        }
        // ensure it is in a folder we care about
        let file_path_str = file.to_str().unwrap().to_string();
        let file_path = Path::new(&file_path_str);
        let relative_path = files   ::relative_path(&in_path, &file_path_str);
        let folder = file_path.parent().unwrap().file_name().unwrap().to_str().unwrap();
        if !folders.contains(&folder) {
            continue;
        }
        // new or modified file?
        let file_path_str = file.to_str().unwrap().to_string();
        let is_new = is_new_file(in_path, &file_path_str, &game_path);
        if is_new {
            fs::copy(file_path_str.clone(), format_mod_path(in_path, &file_path_str, out_path)).unwrap();
            mod_json.files.assets.push(win_to_posix(relative_path));
        } else {
            // is it even different? (warning: messy process..)
            let (is_different, original_bytes, new_bytes) = read_decrypt_compare(&file_path_str, &get_game_path(in_path, &file_path_str, &game_path));
            if !is_different {
                continue;
            }
            let image_game = image::load_from_memory(&original_bytes).unwrap();
            let image_project = image::load_from_memory(&new_bytes).unwrap();
            let image_game_buffer = image_game;
            let image_project_buffer = image_project;
            let diff = olid::compute_diff(&image_game_buffer, &image_project_buffer);
            // to utf str
            let patch_str = unsafe { String::from_utf8_unchecked(diff) }; // has to be a better way to do this
            let patch_path = format_mod_path(in_path, &file_path_str, out_path);
            let patch_pathbuf = Path::new(&patch_path);
            let patch_olid = patch_pathbuf.with_extension("png.olid");
            fs::write(patch_olid, patch_str).unwrap();
            // push the relative path, but remove the .olid
            //let patch_png =files::relative_path(in_path, &file_path_str).replace(".png.olid", ".png");
            let delta_path = format!("{}.olid", files::relative_path(in_path, &file_path_str));
            mod_json.files.image_deltas.push(win_to_posix(delta_path));
        }
    }
    mod_json
}

// step four: generate difference between audio
pub fn difference_audio(in_path: &String, out_path: &String, game_path: &String, mut mod_json: ModJSON) -> ModJSON {
    // collect all files in the paths
    let mut files = Vec::new();
    // the folders to traverse
    let folders = [
        "bgm", "bgs", "me", "se"
    ];
    // go through every entry
    for folder in folders.iter() {
        let folder_path = format!("{}/audio/{}", in_path, folder);
        if !Path::new(&folder_path).exists() {
            continue;
        }
        let folder_files = files::collect_files_recursive(folder_path);
        files.extend(folder_files.clone());
        // if there are no files, skip
        if folder_files.is_empty() {
            continue;
        }
        // go through every entry
        for file in folder_files {
            // ensure it is a .ogg file
            if let Some(extension) = file.extension() {
                if extension != "ogg" {
                    continue;
                }
            }
            // ensure it is in a folder we care about
            let file_path_str = file.to_str().unwrap().to_string();
            let relative_path = files::relative_path(&in_path, &file_path_str);
            // new or modified file?
            let file_path_str = file.to_str().unwrap().to_string();
            let is_new = is_new_file(in_path, &file_path_str, &game_path);
            let (is_different, _original_bytes, _new_bytes) = read_decrypt_compare(&file_path_str, &get_game_path(in_path, &file_path_str, &game_path));
            if is_new || is_different {
                // copy the file over (no patches for audio)
                fs::copy(file_path_str.clone(), format_mod_path(in_path, &file_path_str, out_path)).unwrap();
                mod_json.files.assets.push(win_to_posix(relative_path));
            }
        }
    }
    mod_json
}

// step five: generate difference between videos
pub fn difference_videos(in_path: &String, out_path: &String, game_path: &String, mut mod_json: ModJSON) -> ModJSON {
    // collect all files inside of the movies folder
    let movies_path = format!("{}/movies", in_path);
    if !Path::new(&movies_path).exists() {
        return mod_json;
    }
    let files = files::collect_files_recursive(movies_path);
    if files.is_empty() {
        return mod_json;
    }
    // go through every entry
    for file in files {
        // ensure it is a .webm file
        if let Some(extension) = file.extension() {
            if extension != "webm" {
                continue;
            }
        }
        // new or modified file?
        let file_path_str = file.to_str().unwrap().to_string();
        let is_new = is_new_file(in_path, &file_path_str, &game_path);
        let (is_different, original_bytes, new_bytes) = read_decrypt_compare(&file_path_str, &get_game_path(in_path, &file_path_str, &game_path));
        if is_new || is_different {
            // copy the file over (no patches for videos)
            fs::copy(file_path_str.clone(), format_mod_path(in_path, &file_path_str, out_path)).unwrap();
            mod_json.files.assets.push(win_to_posix(files::relative_path(in_path, &file_path_str)));
        }
    }
    mod_json
}

// step six: generate differences between plugins
pub fn difference_plugins(in_path: &String, out_path: &String, game_path: &String, mut mod_json: ModJSON) -> ModJSON {
    // parse plugins from the project and from the game
    let project_plugins_path = format!("{}/js/plugins.js", in_path);
    let project_plugins = if let Ok(plugins) = info::parse_plugins(project_plugins_path) {
        plugins
    } else {
        eprintln!("Failed to parse project plugins");
        return mod_json; // or handle the error appropriately
    };
    let project_array: Vec<Plugin> = serde_json::from_value(project_plugins).unwrap();
    let game_plugins_path = format!("{}/www/js/plugins.js", game_path);
    let game_plugins = if let Ok(plugins) = info::parse_plugins(game_plugins_path) {
        plugins
    } else {
        eprintln!("Failed to parse game plugins");
        return mod_json; // or handle the error appropriately
    };
    let game_array: Vec<Plugin> = serde_json::from_value(game_plugins).unwrap();
    // create a list of plugin names we want to skip if they are present for some reason..
    let skip_plugins = vec!["No Tomb Code", "deobfuscated"];
    // go through all plugins in the project
    for plugin in project_array {
        // see if its disabled
        if !plugin.status {
            continue;
        }
        // skip plugins if their name matches a banned token
        if skip_plugins.contains(&plugin.name.as_str()) {
            continue;
        }
        // see if the plugin is in the game
        let is_in_game = game_array.iter().any(|old_plugin| plugin.name == old_plugin.name);
        if is_in_game {
            continue;
        }
        // copy the file over
        let file = format!("{}/js/plugins/{}.js", in_path, plugin.name);
        let file_path = Path::new(&file);
        let file_path_str = file_path.to_str().unwrap().to_string();
        fs::copy(file_path_str.clone(), format_mod_path(in_path, &file_path_str, out_path)).unwrap();
        // if the plugin has parameters, we need to hard-core them (for now)
        if plugin.parameters.len() > 0 {
            let plugin_text = fs::read_to_string(file_path_str.clone()).unwrap();
            let modified_text = hardcore_parameters(&plugin, &plugin_text);
            fs::write(format_mod_path(in_path, &file_path_str, out_path), modified_text).unwrap();
        }
        mod_json.files.plugins.push(win_to_posix(files::relative_path(in_path, &file_path_str)));
    }
    mod_json
}

// step seven: sanitize the json
pub fn sanitize_json(mut mod_json: ModJSON) -> ModJSON {
    clear_if_empty(&mut mod_json.files.assets);
    clear_if_empty(&mut mod_json.files.data_deltas);
    clear_if_empty(&mut mod_json.files.image_deltas);
    clear_if_empty(&mut mod_json.files.plugins);
    clear_if_empty(&mut mod_json.files.languages);
    mod_json
}

// reusable packager for other classes
pub fn project_to_mod(in_path: &String, out_path: &String, game_path: &String, 
    mod_name: &String, mod_id: &String, mod_author: &String, mod_description: &String,
    mod_version: &String) {
    // get the game version, but if can't, leave as *
    let game_version = if game::game_version(game_path.clone()) == "Unknown" {
        "*".to_string()
    } else {
        game::game_version(game_path.clone())
    };
    // make empty mod.json (to do: make it support multiple authors)
    let mod_json = ModJSON {
        id: mod_id.clone(),
        name: mod_name.clone(),
        authors: vec![mod_author.clone()],
        description: mod_description.clone(),
        version: mod_version.clone(),
        dependencies: Dependencies {
            game: game_version,
            spec: MOD_JSON_SPEC.to_string(),
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
    let mut mod_json = difference_data(&in_path, &out_path, &game_path, mod_json);
    mod_json = difference_languages(&in_path, &out_path, &game_path, mod_json);
    mod_json = difference_images(&in_path, &out_path, &game_path, mod_json);
    mod_json = difference_audio(&in_path, &out_path, &game_path, mod_json);
    mod_json = difference_videos(&in_path, &out_path, &game_path, mod_json);
    mod_json = difference_plugins(&in_path, &out_path, &game_path, mod_json);
    mod_json = sanitize_json(mod_json);
    // in the mod output folder, write the mod.json
    let mod_json_path = format!("{}/mod.json", out_path);
    let mod_json_str = to_string_pretty(&mod_json).unwrap();
    fs::write(mod_json_path, mod_json_str).unwrap();
}