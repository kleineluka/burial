use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Wry;
use tauri_plugin_store::StoreCollection;
use tauri_plugin_store::with_store;
use serde_json::Value;

const STORE_PATH: &str = ".cache.json";

// insert a value into the persistent storage
pub fn insert_into_store(app: &AppHandle, key: &str, value: serde_json::Value) -> Result<(), String> {
    let stores = app.state::<StoreCollection<Wry>>();
    let path = PathBuf::from(STORE_PATH);
    with_store(app.clone(), stores, path, |store| {
        store.insert(key.to_string(), value)?;
        store.save()?;
        Ok(())
    }).map_err(|e| e.to_string())
}

// read a value from the persistent storage
pub fn read_from_store(app: &AppHandle, key: &str) -> Result<Value, String> {
    let stores = app.state::<StoreCollection<Wry>>();
    let path = PathBuf::from(STORE_PATH);
    with_store(app.clone(), stores, path, |store| {
        let value = store.get(key).ok_or("Key not found").unwrap();
        Ok(value.clone())
    }).map_err(|e| e.to_string())
}

// clear the persistent storage
pub fn clear_store(app: &AppHandle) -> Result<(), String> {
    let stores = app.state::<StoreCollection<Wry>>();
    let path = PathBuf::from(STORE_PATH);
    with_store(app.clone(), stores, path, |store| {
        store.clear()?;
        store.save()?;
        Ok(())
    }).map_err(|e| e.to_string())
}