use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ModPack {
    pub lastUpdate: String,
    pub modloaderVersion: String,
    pub mods: HashMap<String, ModInfo>, 
    pub icon: Option<String>,  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModInfo {
    pub version: String,
}