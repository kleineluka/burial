// imports
use std::path::Path;
use reqwest::Url;
use tokio::fs::create_dir_all;
use tokio::io::AsyncWriteExt;

// download file from url
pub async fn download_file(url: &str, dest_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    // parse the url + make folder destination
    let url = Url::parse(url)?;
    create_dir_all(dest_folder).await?;
    // parse the name + build file destination
    let file_name = url
        .path_segments()
        .and_then(std::iter::Iterator::last)
        .ok_or("Cannot extract file name from URL")?;
    let file_path = Path::new(dest_folder).join(file_name);
    // start the download
    let response = reqwest::get(url.clone()).await?;
    // create the file (asynchronously) with `.await`
    let mut file = tokio::fs::File::create(&file_path).await?;
    // write the content to the file
    let mut content = response.bytes().await?;
    file.write_all_buf(&mut content).await?;
    Ok(())
}
