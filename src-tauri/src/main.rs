
// prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// modules (see more about them in their mod.rs)
mod utils;
mod config;
mod resources;
mod reversing;

// imports
use utils::files;
use utils::commands;
use config::version;
use resources::decryption;
use resources::encryption;
use resources::sprite;
use resources::save;
use reversing::backups;
use reversing::sdk;
use reversing::info;
use reversing::code;

// main
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            version::get_version,
            commands::navigate,
            commands::folder_dialog,
            commands::file_dialog,
            decryption::decrypt,
            encryption::encrypt,
            sprite::make_sprite,
            save::find_saves,
            save::backup_saves,
            save::open_saves,
            save::read_save,
            backups::create_backup,
            backups::get_backups,
            backups::delete_backup,
            backups::clean_backups,
            backups::restore_backup,
            backups::open_backups,
            sdk::install_sdk,
            info::edit_package,
            code::extract_code])
        .run(tauri::generate_context!())
        .expect("Error running Burial.");
}
