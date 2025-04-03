use crate::config::settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum GameVersions {
    V3_0_2, // 3.0.2
    V2_0_14, // 2.0.14
}

pub fn get_user_version() -> GameVersions {
    let version = settings::read_settings().gametarget;
    match version.as_str() {
        "latest" => GameVersions::V3_0_2, // To-Be Updated
        "3.0.2" => GameVersions::V3_0_2, // Chapter 3, Decay
        "3.0.1" => GameVersions::V3_0_2, // minor patch, map up
        "3.0.0" => GameVersions::V3_0_2, // minor patch, map up
        "2.0.14" => GameVersions::V2_0_14, // Chapter 1 + 2
        _ => GameVersions::V3_0_2, // default to 3.0.2 if not found
    }
}