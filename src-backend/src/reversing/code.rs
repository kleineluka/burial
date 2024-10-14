// imports
use tauri::Window;
use tauri::command;
use std::path::Path;
use crate::utils::hausmaerchen;
use crate::utils::files;
use crate::utils::game;
use crate::utils::needle;
use crate::utils::process::kill_process;
use crate::utils::process::ProcessHandle;
use crate::utils::deno;

// check deno status
#[command]
pub fn check_deno(window: Window, operating_system: String) {
    let deno_stauts = deno::deno_presence(&operating_system);
    window.emit("deno_presence", Some(deno_stauts)).unwrap();
}

// auto-run code extraction
#[command]
pub fn extract_code(window: Window, in_path: String, in_file: String, 
    old_text: String, new_text: String, auto_run: bool, 
    auto_restore: bool, auto_deobfuscate: bool, 
    extraction_method: String, deobfuscate_method: String,
    requires_deno: bool, operating_system: String,
    deno_info: deno::DenoInfo, out_path: String) {
    // sanity checks..
    let in_path = Path::new(&in_path);
    let is_game = game::verify_game(&in_path.to_string_lossy()).unwrap();
    if !in_path.exists() || !is_game {
        window.emit("error", Some("That is not a valid TCOAAL folder!".to_string())).unwrap();
        return;
    }
    // install deno first if required to get it out of the way
    if requires_deno {
        window.emit("status", Some("Installing Deno..".to_string())).unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(deno::install_deno(&operating_system, deno_info));
    }
    // combine inPath and inFile to get the full path
    let in_file = in_path.join(in_file);
    // backup the file
    window.emit("status", Some("Backing up original file..".to_string())).unwrap();
    files::backup_file(&in_file.to_string_lossy());
    // inject the code into the file
    window.emit("status", format!("Injecting code via {}..", extraction_method)).unwrap();
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
        // try and kill any remaining processes
        kill_process("Game.exe").unwrap();
        // if auto restore, delete in_file and rename in_file.bak to in_file
        if auto_restore {
            window.emit("status", Some("Restoring original file..".to_string())).unwrap();
            files::restore_backup(&in_file.to_string_lossy());
        }
    }
    if auto_deobfuscate {
        match deobfuscate_method.as_str() {
            "hausmaerchen" => {
                // run hausmaerchen on the file
                window.emit("status", Some("Running hausmaerchen..".to_string())).unwrap();
                let code_file = Path::new(&out_path).join("tcoaal_code.js");
                let hausmaerchen_result = hausmaerchen::run_hausmaerchen(&window, code_file.to_string_lossy().to_string(), code_file.to_string_lossy().to_string(), true, true);
            },
            _ => {
                // run the deobfuscation method
                window.emit("status", Some("Skipping deobfuscation step.. (method not found)".to_string())).unwrap();
            }
        }
    }
    // done
    window.emit("status", Some("Code extraction complete!".to_string())).unwrap();
}

#[command]
pub fn deobfuscate_code(window: Window, in_path: String, deobfuscate_method: String) {
    match deobfuscate_method.as_str() {
        "hausmaerchen" => {
            // run hausmaerchen on the file
            window.emit("status", "Running hausmaerchen...").unwrap();
            let _ = hausmaerchen::run_hausmaerchen(&window, in_path.clone(), in_path.clone(), true, true);
            window.emit("status", "Deobfuscation complete!").unwrap();

        },
        _ => {
            // run the deobfuscation method
            window.emit("status", Some("Deobfuscation method not found, skipping..".to_string())).unwrap();
        }
    }
}

#[command]
pub fn beautify_code(window: Window, in_path: String, beautify_method: String) {
    match beautify_method.as_str() {
        "hausmaerchen" => {
            // run hausmaerchen on the file
            window.emit("status", Some("Running hausmaerchen..".to_string())).unwrap();
            let _ = hausmaerchen::run_hausmaerchen(&window, in_path.clone(), in_path.clone(), false, false);
            window.emit("status", Some("Beautification complete!".to_string())).unwrap();
        },
        _ => {
            // run the beautification method
            window.emit("status", Some("Beautification method not found, skipping..".to_string())).unwrap();
        }
    }
}