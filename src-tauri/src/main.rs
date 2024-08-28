
// prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// modules (see more about them in their mod.rs)
mod utils;
mod config;
mod pages;

// imports
use utils::files;
use utils::commands;
use utils::cryptography;
use utils::cipher;
use pages::decryption;
use pages::sprite;

// main
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::navigate,
            decryption::decrypt,
            sprite::make_sprite])
        .run(tauri::generate_context!())
        .expect("Error running Burial.");
}
