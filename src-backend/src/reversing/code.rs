// imports
use tauri::Window;
use tauri::command;
use std::path::Path;
use crate::utils::files;
use crate::utils::game;
use crate::utils::needle;
use crate::utils::process::ProcessHandle;

// create a backup
#[command]
pub fn extract_code(window: Window, in_path: String, in_file: String, 
    old_text: String, new_text: String, auto_run: bool, 
    auto_restore: bool, auto_deobfuscate: bool) {
    // sanity checks..
    let in_path = Path::new(&in_path);
    let is_game = game::verify_game(&in_path.to_string_lossy()).unwrap();
    if !in_path.exists() || !is_game {
        window.emit("error", Some("That is not a valid TCOAAL folder!".to_string())).unwrap();
        return;
    }
    // combine inPath and inFile to get the full path
    let in_file = in_path.join(in_file);
    // backup the file
    window.emit("status", Some("Backing up original file..".to_string())).unwrap();
    files::backup_file(&in_file.to_string_lossy());
    // inject the code into the file
    needle::replace_text(&in_file.to_string_lossy(), &old_text, &new_text).unwrap();
    // now, it depends on the user's choices
    if auto_run {
        // run the game, wait 30 seconds, then restore the file
        window.emit("status", Some("Running game.. (this may take some time)".to_string())).unwrap();
        let game_exe = in_path.join("Game.exe");
        let mut process_handle = ProcessHandle::new();
        process_handle.start_exe(&game_exe.to_string_lossy()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
        process_handle.stop_process().unwrap();
        // if auto restore, delete in_file and rename in_file.bak to in_file
        if auto_restore {
            window.emit("status", Some("Restoring original file..".to_string())).unwrap();
            files::restore_backup(&in_file.to_string_lossy());
        }
    }
    if auto_deobfuscate {
        // to-do
    }
    // done
    window.emit("status", Some("Code extraction complete!".to_string())).unwrap();
}