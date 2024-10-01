
// prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// modules (see more about them in their mod.rs)
mod settings;
mod utils;
mod config;
mod resources;
mod reversing;
mod tutorial;
mod modmanager;
mod modtools;

// imports
use utils::files;
use utils::commands;
use config::metadata;
use resources::decryption;
use resources::encryption;
use resources::templates;
use resources::save;
use resources::sift;
use resources::dialogue;
use reversing::backups;
use reversing::sdk;
use reversing::injection;
use reversing::info;
use reversing::code;
use tutorial::setup;
use tutorial::finished;
use modmanager::modloader;
use modtools::differences;

// main
fn main() {
    // load the metadata with blocking before starting (version, discord, github, website)
    let rt = tokio::runtime::Runtime::new().unwrap();
    let metadata = rt.block_on(metadata::get_metadata()).unwrap();
    // build tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // set the baseline persistent storage (first run check + user settings)
            let user_settings = config::settings::read_settings();
            config::storage::clear_store(&app.handle()).unwrap();
            config::storage::insert_into_store(&app.handle(), "first-run", serde_json::Value::Bool(config::settings::first_run())).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-tcoaal", serde_json::Value::String(user_settings.tcoaal)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-output", serde_json::Value::String(user_settings.output)).unwrap();
            // set the metadata
            config::storage::insert_into_store(&app.handle(), "metadata-version", serde_json::Value::String(metadata.version)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-discord", serde_json::Value::String(metadata.discord)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-github", serde_json::Value::String(metadata.github)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-website", serde_json::Value::String(metadata.website)).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // register all commands (a bit tedious)
            settings::load_settings,
            settings::save_settings,
            settings::reset_settings,
            metadata::get_local_version,
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
            info::game_version,
            code::extract_code,
            modloader::install_modloader,
            modloader::modloader_version,
            modloader::modloader_versions,
            dialogue::export_dialogue,
            dialogue::preview_export,
            differences::find_differences])
        .run(tauri::generate_context!())
        .expect("Error running Burial.");
}
