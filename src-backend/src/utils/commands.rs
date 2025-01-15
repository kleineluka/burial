use std::process::Command;

// imports
use tauri::api::dialog::FileDialogBuilder;
use tauri::command;
use tauri::Window;
use webbrowser;
use super::game;

// navigate to different pages
#[command]
pub async fn navigate(window: Window, page: String) {
    let _ = window.eval(&format!("window.location.replace('{}')", page));
}

// open a web page in default browser
#[command]
pub async fn open_browser(url: String) {
    if let Err(e) = webbrowser::open(&url) {
        println!("Failed to open browser: {:?}", e);
    }
}

// open a folder dialog to select a folder (and accept a string input for the emit event)
#[command]
pub async fn folder_dialog(window: Window, emit_event: String) {
    tauri::async_runtime::spawn(async move {
        FileDialogBuilder::new()
            // let user pick a directory
            .pick_folder(move |folder_path| {
                if let Some(path) = folder_path {
                    window.emit(&emit_event, Some(path.display().to_string())).unwrap();
                } else {
                    window.emit("error", Some("No folder selected!".to_string())).unwrap();
                }
            });
    });
}

// open a file dialog to select a file (and accept a string input for the emit event and a string for the type of file to pick (if 'all', allow all))
#[command]
pub async fn file_dialog(window: Window, emit_event: String, file_type: String) {
    tauri::async_runtime::spawn(async move {
        let mut dialog = FileDialogBuilder::new();
        // check the file type and set the appropriate filter
        if file_type.to_lowercase() != "all" {
            // .add_filter("TCOAAL Files", &[file_type])
            dialog = dialog.add_filter("TCOAAL Files", &[&file_type]);
        }
        // let the user pick a file
        dialog.pick_file(move |file_path| {
            if let Some(path) = file_path {
                println!("Selected file: {}", path.display().to_string());
                // Send back to front-end
                if let Err(e) = window.emit(&emit_event, Some(path.display().to_string())) {
                    println!("Error emitting event: {:?}", e);
                } 
            } else {
                window.emit("error", Some("No file selected!".to_string())).unwrap();
            }
        });
    });
}

// launch the game from the launcher
#[command]
pub fn launch_game(window: Window, in_path: String) {
    // verify the game path
    let is_game = game::verify_game(&in_path);
    if is_game.is_err() {
        window.emit("error", "Please set your TCOAAL game path in settings!").unwrap();
        return;
    }
    let game_path = format!("{}/Game.exe", in_path);
    match std::process::Command::new(&game_path).spawn() {
        Ok(_) => {
            window.emit("status", "TCOAAL has been launched!").unwrap();
        }
        Err(err) => {
            let error_message = format!("Failed to launch the game! :( [{}]", err);
            window.emit("error", &error_message).unwrap();
        }
    }
}

// open a path in the file explorer
#[command]
pub async fn open_folder(window: Window, in_path: String) {
    #[cfg(target_os = "windows")]
    {
        let win_path = in_path.replace("/", "\\");
        Command::new("explorer").arg(win_path).spawn().unwrap();
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(in_path).spawn().unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(in_path).spawn().unwrap();
    }
}

// helper function to (potentially) emit a status update to a window
#[allow(dead_code)]
pub fn optional_status(message: &str, window: &Option<&tauri::Window>) {
    if let Some(w) = window {
        let _ = w.emit("status", message);
    }
}

// helper function to (potentially) emit an error to a window
#[allow(dead_code)]
pub fn optional_error(message: &str, window: &Option<&tauri::Window>) {
    if let Some(w) = window {
        let _ = w.emit("error", message);
    }
}


