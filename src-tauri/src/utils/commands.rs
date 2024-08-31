// imports
use tauri::api::dialog::FileDialogBuilder;
use tauri::command;
use tauri::Window;

// navigate to different pages
#[command]
pub async fn navigate(window: Window, page: String) {
    let _ = window.eval(&format!("window.location.replace('{}')", page));
}

// open a folder dialog to select a folder (and accept a string input for the emit event)
#[command]
pub async fn folder_dialog(window: Window, emit_event: String) {
    tauri::async_runtime::spawn(async move {
        FileDialogBuilder::new()
            // let user pick a directory
            .pick_folder(move |folder_path| {
                if let Some(path) = folder_path {
                    println!("Selected folder: {}", path.display().to_string());
                    // send back to front-end
                    if let Err(e) = window.emit(&emit_event, Some(path.display().to_string())) {
                        println!("Error emitting event: {:?}", e);
                    } else {
                        println!("Event emitted successfully");
                    }
                } else {
                    println!("No folder selected");
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
                } else {
                    println!("Event emitted successfully");
                }
            } else {
                println!("No file selected");
                window.emit("error", Some("No file selected!".to_string())).unwrap();
            }
        });
    });
}
