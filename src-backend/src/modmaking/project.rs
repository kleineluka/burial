use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use serde_json::{from_str, to_string_pretty, Value};
use tauri::{command, Window};
use crate::resources::dialogue;
use crate::utils::cipher;
use crate::utils::game;
use crate::utils::files;

// take a file from the game directory and format it for the rpg directory
fn format_rpg_path(in_path: &String, out_path: &String, extension: &String) -> String {
    let relative_path = in_path.split("www").collect::<Vec<&str>>()[1].to_string();
    let new_output = format!("{}\\{}.{}", out_path, relative_path, extension);
    let path = Path::new(&new_output).parent().unwrap().to_str().unwrap();
    fs::create_dir_all(path).unwrap();
    new_output
}

// TO RPG PROJECT, step one: copy the game's folder
fn copy_game(in_path: &String, out_path: &String) {
    // get all files (recursively) in in_path/wwww
    let files = files::collect_files_recursive(format!("{}\\www", in_path));
    // print everything in files
    for file in files {
        // get the file's path and ext
        let path = Path::new(&file);
        let path_str = path.to_str().unwrap().to_string();
        let path_no_ext = path_str.split(".").collect::<Vec<&str>>()[0].to_string();
        let ext = path.extension().unwrap().to_str().unwrap();
        match ext {
            "k9a" => {
                // get the file extension via header
                let file_data = fs::read(&path).unwrap();
                let extension = cipher::parse_header(&file_data);
                // read and decrypt the file
                let encrypted_data = fs::read(&path).unwrap(); 
                let decrypted_data = cipher::decrypt(&encrypted_data, path.to_str().unwrap());
                // get the new file path and write to it
                let rpg_file = format_rpg_path(&path_no_ext, &out_path, &extension);
                let mut file = File::create(&rpg_file).unwrap();
                file.write_all(&decrypted_data).unwrap();
            },
            "loc" => {
                // get the raw data to remove the signature
                let raw_loc = fs::read(&path).unwrap();
                let raw_loc_string = String::from_utf8_lossy(&raw_loc);
                let new_loc = dialogue::remove_loc_header(&raw_loc_string);
                // make it a json and parse it
                let loc_bytes = new_loc.as_bytes();
                let loc_json: Value = serde_json::from_slice(&loc_bytes).expect("Invalid JSON data");
                let loc_format = serde_json::to_string_pretty(&loc_json).expect("Failed to format JSON");
                // create a new file at the output path
                let rpg_file = format_rpg_path(&path_str, &out_path, &"loc".to_string());
                let mut file = File::create(&rpg_file).unwrap();
                file.write_all(loc_format.as_bytes()).unwrap();
            },
            _ => {
                // just get the relative file pth to rewrite it to (and manually reattach the extension cos reusing code)
                let ext = path.extension().unwrap().to_str().unwrap();
                let rpg_file = format_rpg_path(&path_no_ext, &out_path, &ext.to_string());
                fs::copy(&path, &rpg_file).unwrap();
            }
        }
    }
}

// TO RPG PROJECT, step two: generate the RPG project file
fn generate_project(out_path: &String) {
    let path = Path::new(&out_path).join("Game.rpgproject");
    let mut file = File::create(&path).expect("Failed to create RPG Project file");
    file.write_all(b"RPGMV 1.6.2").expect("Failed to write to RPG Project file");
}

// TO RPG PROJECT, step three: update the package.json
fn update_package(out_path: &String) {
    let pkg_path = Path::new(&out_path).join("package.json");
    let raw_pkg = fs::read_to_string(&pkg_path).expect("Failed to read RPG package.json");
    let mut pkg: Value = from_str(&raw_pkg).expect("Failed to parse RPG package JSON");
    if let Some(obj) = pkg.as_object_mut() {
        obj.insert("name".to_string(), Value::String("tcoaal".to_string()));
    }
    let new_pkg = to_string_pretty(&pkg).expect("Failed to serialize RPG package JSON");
    fs::write(&pkg_path, new_pkg).expect("Failed to write RPG package.json");
}

// TO RPG PROJECT, step four: patch the index.html (using tomb's patches)
fn patch_index(out_path: &String) {
    // The script to insert
    let script = r#"
        <script>
            // Some patches provided by Tomb

            // Patch language loading to support loading base-game language files without the header
            const orig = window.onload;
            window.onload = () => {
                const readFile = Utils.readFile;
                Utils.readFile = (arg) => {
                    if (Utils.ext(arg) === '.loc') {
                        // We pad the response with empty data, which the game cuts off
                        return ' '.repeat(Buffer.byteLength(SIGNATURE, 'utf8') + 4)
                            + readFile(arg);
                    }

                    return readFile(arg);
                };

                // Stop asset decryption calls
                if (Crypto.resolveURL) Crypto.resolveURL = url => url;

                // Call the original onload
                orig();
            }
        </script>
    "#.trim(); 
    // read file
    let index_path = Path::new(&out_path).join("index.html");
    let index = fs::read_to_string(&index_path).expect("Failed to read index.html");
    // patch and write
    let updated_index = index.replace("</body>", &format!("{}\n</body>", script));
    fs::write(&index_path, updated_index).expect("Failed to write index.html");
}

// reusable packaqge for other classes
pub fn game_to_rpg(in_path: String, out_path: String) {
    copy_game(&in_path, &out_path);
    generate_project(&out_path);
    update_package(&out_path);
    patch_index(&out_path);
}

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
    copy_game(&in_path, &out_path);
    // step two: generate the RPG project file
    window.emit("status", "Generating your RPG project file..").unwrap();
    generate_project(&out_path);
    // step three: update the package.json
    window.emit("status", "Updating your package.json..").unwrap();
    update_package(&out_path);
    // step four: patch the index.html
    window.emit("status", "Applying patches to your game..").unwrap();
    patch_index(&out_path);
    // done
    window.emit("status", "Your RPG Maker MV project has been created!").unwrap();
}