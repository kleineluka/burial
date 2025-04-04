use crate::config::settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum MajorGameVersions {
    V3_0_3, // Chapter 3, Decay
    V2_0_14, // Chapter 1 + 2
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum MinorGameVersions {
    V3_0_3, // Chapter 3, Decay
    V3_0_2, // Chapter 3, Decay
    V3_0_1, // Chapter 3, Decay
    V3_0_0, // Chapter 3, Decay
    V2_0_14, // 2.0.14
}

// define latest here 
const MAJOR_LATEST_VERSION: MajorGameVersions = MajorGameVersions::V3_0_3;
const MINOR_LATEST_VERSION: MinorGameVersions = MinorGameVersions::V3_0_2;

// basically, what actually we differentiate between when modding
pub fn get_functional_version() -> MajorGameVersions {
    let version = settings::read_settings().gametarget;
    match version.as_str() {
        "latest" => MAJOR_LATEST_VERSION,
        "3.0.3" => MajorGameVersions::V3_0_3, // Chapter 3, Decay
        "3.0.2" => MajorGameVersions::V3_0_3, // Chapter 3, Decay
        "3.0.1" => MajorGameVersions::V3_0_3, // Chapter 3, Decay
        "3.0.0" => MajorGameVersions::V3_0_3, // Chapter 3, Decay
        "2.0.14" => MajorGameVersions::V2_0_14, // Chapter 1 + 2
        _ => MAJOR_LATEST_VERSION
    }
}

// specific version (ex. 3.0.3 vs 3.0.2) that shouldn't matter for most purposes
pub fn get_actual_version() -> MinorGameVersions {
    let version = settings::read_settings().gametarget;
    match version.as_str() {
        "latest" => MINOR_LATEST_VERSION,
        "3.0.3" => MinorGameVersions::V3_0_3, // Chapter 3, Decay
        "3.0.2" => MinorGameVersions::V3_0_2, // Chapter 3, Decay
        "3.0.1" => MinorGameVersions::V3_0_1, // Chapter 3, Decay
        "3.0.0" => MinorGameVersions::V3_0_0, //// Chapter 3, Decay
        "2.0.14" => MinorGameVersions::V2_0_14, // Chapter 1 + 2
        _ => MINOR_LATEST_VERSION
    }
}