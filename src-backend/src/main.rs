
// prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// modules (see more about them in their mod.rs)
mod settings;
mod utils;
mod config;
mod resources;
mod reversing;
mod tutorial;

// imports
use utils::files;
use utils::commands;
use config::version;
use resources::decryption;
use resources::encryption;
use resources::templates;
use resources::save;
use resources::sift;
use reversing::backups;
use reversing::sdk;
use reversing::injection;
use reversing::info;
use reversing::code;
use tutorial::setup;
use tutorial::finished;

// main
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            settings::load_settings,
            settings::save_settings,
            settings::reset_settings,
            version::get_version,
            commands::navigate,
            commands::folder_dialog,
            commands::file_dialog,
            commands::open_browser,
            setup::setup_game,
            setup::setup_settings,
            finished::setup_finish,
            decryption::decrypt,
            encryption::encrypt,
            templates::make_sprite,
            templates::make_preview,
            sift::export_resources,
            save::find_saves,
            save::backup_saves,
            save::open_saves,
            save::read_save,
            save::write_save,
            save::copy_save,
            save::delete_all,
            save::delete_auto,
            backups::create_backup,
            backups::get_backups,
            backups::delete_backup,
            backups::clean_backups,
            backups::restore_backup,
            backups::open_backups,
            sdk::install_sdk,
            injection::injection_backup,
            injection::injection_open_file,
            injection::injection_open_folder,
            injection::injection_preview,
            injection::injection_save,
            info::edit_package,
            code::extract_code])
        .run(tauri::generate_context!())
        .expect("Error running Burial.");
}
