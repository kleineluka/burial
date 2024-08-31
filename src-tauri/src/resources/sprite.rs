// imports
use tauri::Window;
use tauri::command;
use std::fs;
use std::path::Path;
use crate::utils::game;
use crate::utils::files;
use crate::utils::bytes;
use crate::resources::decryption;

// get old file, copy to new folder, patch the new file
// please forgive me for this messy, daisy-chained code...
#[command]
pub fn make_sprite(window: Window, game_path: String, sprite_path: String, out_path: String, sprite_name: String, byte_list: String) {
    // verify the path is the game path
    /*if !game::verify_game(&game_path).unwrap() {
        window.emit("error", Some("Invalid game path!".to_string())).unwrap();
        return;
    }
    // print all recieved data
    println!("game_path: {}", game_path);
    println!("sprite_path: {}", sprite_path);
    println!("out_path: {}", out_path);
    println!("sprite_name: {}", sprite_name);
    //println!("byte_list: {}", byte_list);
    // navigate to game_path + sprite_path
    let sprite_path_new = Path::new(&game_path).join(&sprite_path);
    // copy the file to the out path
    let sprite_name = sprite_name.replace("#", "");
    let file_name = format!("{}.png", &sprite_name);
    let out_path_new = Path::new(&out_path).join(&file_name);
    let out_path_old = out_path;
    // decrypt the file to the out path
    let kind = "file".to_string();
    println!("sprite_path_new: {}", sprite_path_new.to_string_lossy());
    println!("out_path_old: {}", out_path_old);
    decryption::decrypt(window.clone(), kind, sprite_path_new.to_string_lossy().to_string(), out_path_old.clone(), false);
    // rename the file at out_path + old file name + .png (ALWAYS PNG) -> new file name + .png
    let old_file_name = files::file_name(&sprite_path);
    let old_out_path = Path::new(out_path_old.as_str()).join(format!("{}.png", old_file_name));
    // print old_out_path and out_path_new
    println!("old_out_path: {}", old_out_path.to_string_lossy());
    println!("out_path_new: {}", out_path_new.to_string_lossy());
    fs::rename(&old_out_path, &out_path_new).unwrap();
    // patch the file
    bytes::patch_file(&out_path_new.to_string_lossy(), &byte_list).unwrap();
    // log success
    window.emit("log", Some("Sprite created successfully!")).unwrap();*/
}