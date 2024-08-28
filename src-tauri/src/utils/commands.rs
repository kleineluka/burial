// imports
use tauri::api::dialog::FileDialogBuilder;
use tauri::command;
use tauri::Window;

// navigate to different pages
#[command]
pub async fn navigate(window: Window, page: String) {
    let _ = window.eval(&format!("window.location.replace('{}')", page));
}
