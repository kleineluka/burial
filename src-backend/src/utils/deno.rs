// imports
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::cache;
use super::{compression, connection};

// distribution and deno structures for JSON parsing
#[derive(Debug, Deserialize, Serialize)]
pub struct Distributions {
    pub windows_x64: String,
    pub macos_x64: String,
    pub macos_arm64: String,
    pub linux_x64: String,
    pub linux_arm64: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DenoInfo {
    pub url: String,
    pub version: String,
    pub distributions: Distributions,
}

// return true if v1 is greater than v2
fn compare_semantic(v1: &str, v2: &str) -> bool {
    let parse_version = |v: &str| -> (i32, i32, i32) {
        let parts: Vec<i32> = v.split('.')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        (parts[0], parts[1], parts[2])
    };
    let (major1, minor1, patch1) = parse_version(v1);
    let (major2, minor2, patch2) = parse_version(v2);
    if major1 != major2 {
        return major1 > major2;
    } else if minor1 != minor2 {
        return minor1 > minor2;
    } else {
        return patch1 > patch2;
    }
}

// create deno path if it doesn't exist
pub fn verify_deno() {
    // get the cache + deno + deno.exe
    let cache = cache::cache_folder();
    let deno_path = cache.join("deno");
    // create the deno folder if it doesn't exist
    if !deno_path.exists() {
        std::fs::create_dir_all(&deno_path).unwrap();
    }
}

// return deno path
pub fn deno_path() -> PathBuf {
    verify_deno();
    let cache = cache::cache_folder();
    let deno_path = cache.join("deno");
    deno_path
}

// see if deno is currently installed
pub fn deno_presence(operating_system: &String) -> bool {
    // only windows is supported right now.. sowwy..
    if operating_system != "windows" {
        return false;
    }
    // see if the exe exists
    let deno_path = deno_path();
    let deno_exe = deno_path.join("deno.exe");
    if !deno_exe.exists() {
        return false;
    }
    // if deno.exe is below 10kb, return false (probably an error installing, so try to self-correct)
    let metadata = deno_exe.metadata().unwrap();
    if metadata.len() < 10000 {
        return false;
    }
    // if all checks pass, return true!
    true
}

// install deno
pub async fn install_deno(operating_system: &String, deno_info: DenoInfo) -> bool{
    // only windows is supported right now.. sowwy..
    if operating_system != "windows" {
        return false;
    }
    let deno_version = deno_info.distributions.windows_x64;
    // get the deno path
    let deno_path = deno_path();
    // see if deno already exists, if so, delete it
    if deno_presence(operating_system) {
        remove_deno();
        verify_deno();
    }
    // construct the download url
    let download_url = format!("{}/releases/download/v{}/{}", deno_info.url, deno_info.version, deno_version);
    if let Err(e) = connection::download_file(&download_url, &deno_path.to_string_lossy()).await {
        return false;
    }
    // now in deno_path + deno_version, extract the zip
    let deno_zip = deno_path.join(deno_version);
    compression::decompress_zip(&deno_zip, &deno_path).unwrap();
    // delete the zip
    std::fs::remove_file(&deno_zip).unwrap();
    // in the path, write a version.txt file with the version
    let version_file = deno_path.join("version.txt");
    std::fs::write(&version_file, deno_info.version).unwrap();
    // downloaded successfully
    return true;
}

// remove deno
pub fn remove_deno() {
    // get the deno path
    let deno_path = deno_path();
    let _ = std::fs::remove_dir_all(&deno_path);
}

// compare deno versions
pub fn compare_deno(deno_info: DenoInfo) -> bool {
    // get the deno path
    let deno_path = deno_path();
    // get the version file
    let version_file = deno_path.join("version.txt");
    // if the version file doesn't exist, return false
    if !version_file.exists() {
        return false;
    }
    // read the version file
    let current_version = std::fs::read_to_string(&version_file).unwrap();
    // compare the versions
    return compare_semantic(&deno_info.version, &current_version);
}

// get deno executable
pub fn deno_executable() -> PathBuf {
    let deno_path = deno_path();
    deno_path.join("deno.exe")
}