// imports
use tauri::Manager;
use tauri::Window;
use tauri::command;
use crate::utils::popup;

// open a new window
#[command]
pub fn edit_package(window: Window, in_path: String) {
    let app_handle = window.app_handle();
    // print out the current url of the window
    println!("Current URL: {}", window.url());
    //popup::create_popup(window, app, identifier, width, height, title, url)
    popup::create_popup(window, &app_handle, "edit_package", 800.0, 600.0, "Edit Package", "/sub/popup/texteditor.html");
}