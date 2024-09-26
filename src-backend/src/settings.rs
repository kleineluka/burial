// imports
use tauri::Window;
use tauri::command;
use crate::config::settings;

// load settings
#[command]
pub fn load_settings(window: Window) {
    // first, ensure it exists + read
    settings::check_settings();
    let settings = settings::read_settings();
    let settings_json = serde_json::to_string(&settings)
        .expect("Failed to serialize settings");
    // send the settings to the frontend
    window.emit("settings-loaded", settings_json).unwrap();
}

// write settings
#[command]
pub fn save_settings(window: Window, tcoaal: String, output: String) {
    // read the current settings
    let mut settings = settings::read_settings();
    // update the settings
    settings.tcoaal = tcoaal;
    settings.output = output;
    // write the updated settings
    settings::write_settings(settings);
    window.emit("settings-saved", {}).unwrap(); 
}

// reset settings
#[command]
pub fn reset_settings(window: Window) {
    settings::delete_settings();
    settings::check_settings();
    window.emit("settings-reset", {}).unwrap();
}