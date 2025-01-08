use std::f32::consts::E;
use std::fs::{self, File};
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

// detects what kind of an archive this is
pub fn get_archive_type(file_path: &Path) -> Result<&'static str, std::io::Error> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = [0; 4]; // Read the first 4 bytes
    file.read_exact(&mut buffer)?;
    if buffer == [0x50, 0x4B, 0x03, 0x04] || buffer == [0x50, 0x4B, 0x05, 0x06] {
        Ok("zip")
    } else if &buffer == b"Rar!" { // different kind of rar..
        Ok("rar")
    } else if buffer == [0x52, 0x61, 0x72, 0x21] {
        Ok("rar")
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unsupported file type",
        ))
    }
}


// zip a directory to an output file
pub fn compress_directory<T>(src_dir: &Path, output_file: T) -> zip::result::ZipResult<()> where T: Write + Seek, {
    // create the zip options
    let mut zip = ZipWriter::new(output_file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    // iterate over the files in the directory
    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();
    for entry in it.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(src_dir)).unwrap();
        // if the path is a file, add it to the zip
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        } else if path.is_dir() {
            // directories need to be added with a trailing slash
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

// decompress a zip file to a directory (don't delete zip for now..)
pub fn decompress_zip(zip_file_path: &Path, output_folder: &Path) -> io::Result<()> {
    let file = File::open(zip_file_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut outpath = output_folder.to_path_buf();
        if let Some(name) = file.enclosed_name() {
            outpath.push(name);
        } else {
            continue;
        }
        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
            // configure unix permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }
    }
    // delete the zip file after extraction is complete
    // fs::remove_file(zip_file_path)?;
    Ok(())
}

// don't make sub folder inside of it..
pub fn decompress_zip_nosub(zip_file_path: &Path, output_folder: &Path) -> io::Result<()> {
    // decompress the directory first
    decompress_zip(zip_file_path, output_folder)?;
    // find the extracted folder by looking for the first subdirectory in the output folder
    let extracted_folder_path = fs::read_dir(output_folder)?
        .filter_map(|entry| entry.ok()) // ignore errors
        .find(|entry| entry.path().is_dir()) // find the first directory
        .map(|entry| entry.path()) // get its path
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Extracted folder not found..?"))?;
    // move files from the extracted folder to the output folder
    for entry in fs::read_dir(&extracted_folder_path)? {
        let entry = entry?;
        let path = entry.path();
        let new_path = output_folder.join(path.file_name().unwrap());
        fs::rename(path, new_path)?;
    }
    // remove the original extracted folder
    fs::remove_dir(extracted_folder_path)?;
    Ok(())
}

// decompress EITHER a zip or rar
pub fn decompress_archive(file_path: &Path, output_folder: &Path, no_sub: bool) -> Result<(), Box<dyn std::error::Error>> {
    match get_archive_type(file_path)? {
        "zip" => {
            if no_sub {
                decompress_zip_nosub(file_path, output_folder)?;
            } else {
                decompress_zip(file_path, output_folder)?;
            }
        }
        "rar" => {
            return Err("Rar decompression not yet implemented".into());
        }
        _ => {
            return Err("Unsupported archive type".into());
        }
    }
    Ok(())
}