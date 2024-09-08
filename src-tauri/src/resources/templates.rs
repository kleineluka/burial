// imports
use tauri::Window;
use tauri::command;
use std::fs;
use std::path::Path;
use crate::utils::cipher;
use crate::utils::game;
use crate::utils::files;
use crate::utils::bytes;

// get old file, copy to new folder, decrypt the new file, then patch the new file
#[command]
pub fn make_sprite(window: Window, game_path: String, sprite_path: String, out_path: String, sprite_name: String, byte_list: String) {
    // verify the path is the game path
    if !game::verify_game(&game_path).unwrap() {
        window.emit("error", Some("Invalid game path!".to_string())).unwrap();
        return;
    }
    // navigate to game_path + sprite_path
    let sprite_path_new = Path::new(&game_path).join(&sprite_path);
    // decrypt the asset file
    window.emit("status", "Decrypting the asset file.").unwrap();
    let (decrypted_data, file_name_with_extension) = cipher::decrypt_file(&sprite_path_new.to_string_lossy());
    let sprite_out = Path::new(&out_path).join(&file_name_with_extension);
    files::write_file(&sprite_out.to_string_lossy(), &decrypted_data);
    window.emit("status", Some(format!("File {} has been decrypted.", file_name_with_extension))).unwrap();
    // patch the file
    window.emit("status", "Patching the bytes of the asset file.").unwrap();
    bytes::patch_file(&sprite_out.to_string_lossy(), &byte_list).unwrap();
    // rename the file to the sprite name
    window.emit("status", Some("Renaming the written file.")).unwrap();
    let sprite_extension = files::file_extension(&sprite_out.to_string_lossy());
    let sprite_out_new = Path::new(&out_path).join(&sprite_name).with_extension(sprite_extension);
    fs::rename(&sprite_out, &sprite_out_new).unwrap();
    window.emit("status", Some("Template created successfully!")).unwrap();
}