// imports
use serde_json::Value;
use reqwest::Error;
use crate::utils::connection;

// take in a repo and get the codeberg api for it
pub fn get_codeberg_api(url: &str) -> String {
    let mut url = url.replace("https://codeberg.org/", "https://codeberg.org/api/v1/repos/");
    url = format!("{}/releases", url);
    url
}

// get the latest release of a file name in a repo
pub async fn get_latest_release(url: &str, file_name: &str) -> Result<Option<String>, Error> {
    // get the latest release info
    let response = reqwest::get(url).await?;
    let releases: Vec<Value> = response.json().await?;
    // find the latest release
    if let Some(latest_release) = releases.first() {
        // look for the right asset to get the download url for
        if let Some(assets) = latest_release["assets"].as_array() {
            for asset in assets {
                if let Some(name) = asset["name"].as_str() {
                    if name == file_name {
                        if let Some(download_url) = asset["browser_download_url"].as_str() {
                            return Ok(Some(download_url.to_string()));
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

// get a specific release via release name in a repo
pub async fn get_specific_release(url: &str, file_name: &str, desired_name: &str) -> Result<Option<String>, Error> {
    // get the releases
    let response = reqwest::get(url).await?;
    let releases: Vec<Value> = response.json().await?;
    // find the release
    for release in releases {
        if let Some(release_name) = release["name"].as_str() {
            if release_name == desired_name {
                // look for the right asset to get the download url for
                if let Some(assets) = release["assets"].as_array() {
                    for asset in assets {
                        if let Some(name) = asset["name"].as_str() {
                            if name == file_name {
                                if let Some(download_url) = asset["browser_download_url"].as_str() {
                                    return Ok(Some(download_url.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

// get all release versions
pub async fn get_all_releases(url: &str) -> Result<Vec<String>, Error> {
    // get the releases
    let response = reqwest::get(url).await?;
    let releases: Vec<Value> = response.json().await?;
    // iterate through the releases and get the tag names
    let mut release_versions = Vec::new();
    for release in releases {
        if let Some(release_name) = release["name"].as_str() {
            release_versions.push(release_name.to_string());
        }
    }
    Ok(release_versions)
}

// take a repo, get the api, get the latest release, and download it to a location
pub async fn download_latest_release(url: &str, file_name: &str, download_dest: &str) -> bool {
    // get the latest release
    let codeberg_api = get_codeberg_api(url);
    let download_url = get_latest_release(&codeberg_api, file_name).await.unwrap();
    // download the release
    if let Some(download_url) = download_url {
        if let Err(_e) = connection::download_file(&download_url, &download_dest).await {
            return false;
        }
    }
    true
}

// download a specific release
pub async fn download_specific_release(url: &str, file_name: &str, download_dest: &str, release_name: &str) -> bool {
    // get the latest release
    let codeberg_api = get_codeberg_api(url);
    let download_url = get_specific_release(&codeberg_api, file_name, release_name).await.unwrap();
    // download the release
    if let Some(download_url) = download_url {
        if let Err(_e) = connection::download_file(&download_url, &download_dest).await {
            return false;
        }
    }
    true
}