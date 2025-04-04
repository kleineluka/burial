
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
mod modmaking;

// (other) imports
use tauri::Manager;

// (local) imports
use utils::helpers::files;
use utils::frontend::commands;
use utils::operating::protocol;
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
use modmanager::installed;
use modmanager::modloader;
use modmanager::modpacks;
use modmanager::browser;
use modmanager::profiles;
use modmanager::external;
use modmaking::differences;
use modmaking::modjson;
use modmaking::repojson;
use modmaking::bundler;
use modmaking::converter;
use modmaking::decompiler;

// accepting cli arguments (and deeplinking)
#[derive(Clone, serde::Serialize, Debug)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

fn main() {
    // load the config for the app + user settings + (optional) fetch metadata (w/ blocking..) + set up protocol (if not already)
    let app_config = app::load_config();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let user_settings = config::settings::read_settings();
    let user_hash = utils::operating::environment::get_hwid();
    let metadata = rt.block_on(metadata::get_metadata(&app_config, &user_settings, &user_hash)).unwrap();
    let _protocol_init = protocol::register_protocol(user_settings.deeplinks);
    // build tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            let payload = Payload {
                args: argv.to_vec(),
                cwd: cwd.to_string(),
            };
            println!("payload: {:?}", payload);
            // to-do
        }))
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            // set the baseline persistent storage 
            config::storage::clear_store(&app.handle()).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-first-run", serde_json::Value::Bool(config::settings::first_run())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-local-version", serde_json::Value::String(metadata::get_local_version())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-operating-system", serde_json::Value::String(utils::operating::environment::get_os().to_owned())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-bundled-resources", serde_json::Value::String(utils::operating::environment::get_resources(app).to_string_lossy().to_string())).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-user-hash", serde_json::Value::String(user_hash)).unwrap();
            config::storage::insert_into_store(&app.handle(), "state-app-ver", serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string())).unwrap();
            // set user settings
            config::storage::insert_into_store(&app.handle(), "settings-tcoaal", serde_json::Value::String(user_settings.tcoaal)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-output", serde_json::Value::String(user_settings.output)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-updates", serde_json::Value::Bool(user_settings.updates)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-theme", serde_json::Value::String(user_settings.theme)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-animations", serde_json::Value::Bool(user_settings.animations)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-tooltips", serde_json::Value::Bool(user_settings.tooltips)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-modname", serde_json::Value::String(user_settings.modname)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-modid", serde_json::Value::String(user_settings.modid)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-modauthor", serde_json::Value::String(user_settings.modauthor)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-moddescription", serde_json::Value::String(user_settings.moddescription)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-deeplinks", serde_json::Value::Bool(user_settings.deeplinks)).unwrap();
            config::storage::insert_into_store(&app.handle(), "settings-gametarget", serde_json::Value::String(user_settings.gametarget)).unwrap();
            // set the config settings
            config::storage::insert_into_store(&app.handle(), "config-api-server", serde_json::Value::String(app_config.api_server)).unwrap();
            config::storage::insert_into_store(&app.handle(), "config-api-backup-server", serde_json::Value::String(app_config.api_backup_server)).unwrap();
            config::storage::insert_into_store(&app.handle(), "config-metadata-timeout", serde_json::Value::Number(serde_json::Number::from(app_config.metadata_timeout))).unwrap();
            // set the metadata
            config::storage::insert_into_store(&app.handle(), "metadata-version", serde_json::Value::String(metadata.version)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-discord", serde_json::Value::String(metadata.discord)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-github", serde_json::Value::String(metadata.github)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-website", serde_json::Value::String(metadata.website)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-itchio", serde_json::Value::String(metadata.itchio)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-gamebanana", serde_json::Value::String(metadata.gamebanana)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-nexusmods", serde_json::Value::String(metadata.nexusmods)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-repo-server", serde_json::Value::String(metadata.repo_server)).unwrap();
            config::storage::insert_into_store(&app.handle(), "metadata-news-server", serde_json::Value::String(metadata.news_server)).unwrap();
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
            settings::settings_auto_find,
            settings::output_auto_find,
            settings::uninstall_deeplinks,
            metadata::get_local_version,
            commands::navigate,
            commands::folder_dialog,
            commands::file_dialog,
            commands::open_browser,
            commands::launch_game,
            commands::open_folder,
            setup::setup_game,
            setup::setup_settings,
            setup::setup_auto_find,
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
            modpacks::install_modpack,
            modpacks::current_modpack,
            modpacks::uninstall_modpack,
            installed::installed_mods,
            installed::disable_mod,
            installed::enable_mod,
            browser::mod_ready,
            browser::install_mod,
            browser::uninstall_mod,
            external::download_external_mod,
            profiles::load_profiles,
            profiles::add_profile,
            profiles::remove_profile,
            profiles::reset_profile,
            profiles::install_game_copy,
            profiles::delete_game_copy,
            profiles::game_copy_version,
            profiles::toggle_profile_mod,
            profiles::set_profile,
            profiles::compare_install,
            profiles::init_install,
            profiles::launch_profile,
            dialogue::export_dialogue,
            dialogue::import_dialogue,
            dialogue::preview_export,
            dialogue::preview_import,
            differences::find_differences,
            modjson::load_modjson,
            modjson::save_modjson,
            repojson::load_repojson,
            repojson::save_repojson,
            bundler::export_rpg_project,
            bundler::export_mod_folder,
            converter::convert_mod,
            decompiler::decompile_mod,
            info::general_info,
            info::plugins_info])
        .run(tauri::generate_context!())
        .expect("Error running Burial.");
}
