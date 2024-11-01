
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

// (other) imports
use tauri::Manager;

// (local) imports
use utils::files;
use utils::commands;
use config::app;
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
use reversing::dev;
use tutorial::setup;
use tutorial::finished;
use modmanager::modloader;
use modmanager::installed;
use modmanager::instances;
use modtools::differences;
use modtools::modjson;

// main
fn main() {
    // load the config for the app + fetch metadata (w/ blocking..) + user settings
    let app_config = app::load_config();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let metadata = rt.block_on(metadata::get_metadata(&app_config)).unwrap();
    let user_settings = config::settings::read_settings();
    // build tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            // set the baseline persistent storage 
            config::storage::clear_store(&app.handle()).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-first-run", serde_json::Value::Bool(config::settings::first_run())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-hwid", serde_json::Value::String(utils::environment::get_hwid())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-local-version", serde_json::Value::String(metadata::get_local_version())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-operating-system", serde_json::Value::String(utils::environment::get_os().to_owned())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-game-instance", serde_json::Value::String("default".to_string())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-bundled-resources", serde_json::Value::String(utils::environment::get_resources(app).to_string_lossy().to_string())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-starting-page", serde_json::Value::String("home".to_string())).unwrap();
            // set user settings
            config::storage::insert_into_store(&app.handle(), "settings-tcoaal", serde_json::Value::String(user_settings.tcoaal)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-output", serde_json::Value::String(user_settings.output)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-launcher-hotload", serde_json::Value::Bool(user_settings.hotload)).unwrap();
            // set the config settings
            config::storage::insert_into_store(&app.handle(), "config-metadata-server", serde_json::Value::String(app_config.metadata_server)).unwrap();
            config::storage::insert_into_store(&app.handle(), "config-metadata-timeout", serde_json::Value::Number(serde_json::Number::from(app_config.metadata_timeout))).unwrap();
            config::storage::insert_into_store(&app.handle(), "config-mods-repository", serde_json::Value::String(app_config.mods_repository)).unwrap();
            // set the metadata
            config::storage::insert_into_store(&app.handle(), "metadata-version", serde_json::Value::String(metadata.version)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-discord", serde_json::Value::String(metadata.discord)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-github", serde_json::Value::String(metadata.github)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-website", serde_json::Value::String(metadata.website)).unwrap();
            Ok(())
        }).on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                // clear the store to prevent outdated information from being saved in case of an error
                config::storage::clear_store(&event.window().app_handle()).unwrap();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // register all commands (a bit tedious)
            settings::load_settings,
            settings::save_settings,
            settings::reset_settings,
            settings::remove_deno,
            settings::remove_hausmaerchen,
            settings::install_dev_tools,
            metadata::get_local_version,
            commands::navigate,
            commands::folder_dialog,
            commands::file_dialog,
            commands::open_browser,
            setup::setup_game,
            setup::setup_settings,
            setup::auto_find_game,
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
            sdk::sdk_presence_wrapper,
            injection::injection_backup,
            injection::injection_open_file,
            injection::injection_open_folder,
            injection::injection_preview,
            injection::injection_save,
            code::check_deno,
            code::extract_code,
            code::deobfuscate_code,
            code::beautify_code,
            dev::dev_presences,
            dev::toggle_devtools,
            modloader::install_modloader,
            modloader::uninstall_modloader,
            modloader::modloader_version,
            modloader::modloader_versions,
            dialogue::export_dialogue,
            dialogue::import_dialogue,
            dialogue::preview_export,
            dialogue::preview_import,
            differences::find_differences,
            modjson::load_modjson,
            modjson::save_modjson,
            info::general_info])
        .run(tauri::generate_context!())
        .expect("Error running Burial.");
}
