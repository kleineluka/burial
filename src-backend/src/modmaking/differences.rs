// imports
use tauri::{command, Window};
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use blake3::Hasher;
use crate::config::cache;
use crate::utils::compression;

// fast hash, good enough for changes
fn fast_hash(file_path: &Path) -> u32 {
    let file = fs::File::open(file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();
    let mut buffer = [0; 4096]; // 4kb buffer
    loop {
        let n = reader.read(&mut buffer).expect("Unable to read file");
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let hash = hasher.finalize();
    let bytes = hash.as_bytes();
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

// create a hash map of the files in a directory
fn create_hashmap(directory: &str) -> HashMap<PathBuf, u32> {
    let mut file_hashes = HashMap::new();
    for entry in fs::read_dir(directory).expect("Unable to read directory") {
        let entry = entry.expect("Unable to get entry");
        let path = entry.path();
        if path.is_file() {
            // create the fast hash
            let hash = fast_hash(&path);
            let path = path.strip_prefix(directory).unwrap().to_path_buf();
            file_hashes.insert(path, hash);
        } else if path.is_dir() {
            // recurse!
            let sub_hashes = create_hashmap(path.to_str().unwrap());
            file_hashes.extend(sub_hashes);
        }
    }
    file_hashes
}

// collect all added files into a vector (using mod1 as the original) and make sure to account for if a file was MOVED
fn added_files(mod1: &HashMap<PathBuf, u32>, mod2: &HashMap<PathBuf, u32>) -> Vec<PathBuf> {
    let mut added = Vec::new();
    for (path, hash) in mod2.iter() {
        if !mod1.contains_key(path) {
            let mut found = false;
            for (_old_path, old_hash) in mod1.iter() {
                if hash == old_hash {
                    added.push(path.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                added.push(path.clone());
            }
        }
    }
    added
}

// collect all removed files into a vector (using mod1 as the original)
fn removed_files(mod1: &HashMap<PathBuf, u32>, mod2: &HashMap<PathBuf, u32>) -> Vec<PathBuf> {
    let mut removed = Vec::new();
    for (path, hash) in mod1.iter() {
        if !mod2.contains_key(path) {
            let mut found = false;
            for (_new_path, new_hash) in mod2.iter() {
                if hash == new_hash {
                    removed.push(path.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                removed.push(path.clone());
            }
        }
    }
    removed
}

// collect all changed files into a vector (using mod1 as the original)
fn changed_files(mod1: &HashMap<PathBuf, u32>, mod2: &HashMap<PathBuf, u32>) -> Vec<PathBuf> {
    let mut changed = Vec::new();
    for (path, hash) in mod1.iter() {
        if mod2.contains_key(path) {
            let new_hash = mod2.get(path).unwrap();
            if hash != new_hash {
                changed.push(path.clone());
            }
        }
    }
    changed
}

// collect all unchanged files into a vector (using mod1 as the original)
fn unchanged_files(mod1: &HashMap<PathBuf, u32>, mod2: &HashMap<PathBuf, u32>) -> Vec<PathBuf> {
    let mut unchanged = Vec::new();
    for (path, hash) in mod1.iter() {
        if mod2.contains_key(path) {
            let new_hash = mod2.get(path).unwrap();
            if hash == new_hash {
                unchanged.push(path.clone());
            }
        }
    }
    unchanged
}

// find all files that are the same, but were moved (using mod1 as the original)
fn moved_files(mod1: &HashMap<PathBuf, u32>, mod2: &HashMap<PathBuf, u32>) -> Vec<(PathBuf, PathBuf)> {
    let mut moved = Vec::new();
    for (path1, hash1) in mod1 {
        if let Some((path2, _)) = mod2.iter().find(|(path2, hash2)| hash1 == *hash2 && &path1 != path2) {
            moved.push((path1.clone(), path2.clone()));
        }
    }
    moved
}

// for every file that is changed between the two mods, create a list of changes inside that file
fn file_changes(mod1: &HashMap<PathBuf, u32>, mod2: &HashMap<PathBuf, u32>) -> HashMap<PathBuf, Vec<String>> {
    let mod1_dir = Path::new(cache::temp_folder().to_str().unwrap()).join("mod1");
    let mod2_dir = Path::new(cache::temp_folder().to_str().unwrap()).join("mod2");
    let mut changes = HashMap::new();
    for (path, hash) in mod1.iter() {
        if mod2.contains_key(path) {
            let new_hash = mod2.get(path).unwrap();
            if hash != new_hash {
                let mut file_changes = Vec::new();
                let old_file = fs::read_to_string(mod1_dir.join(path)).expect("Unable to read file");
                let new_file = fs::read_to_string(mod2_dir.join(path)).expect("Unable to read file");
                let diff = diff::lines(&old_file, &new_file);
                for change in diff {
                    match change {
                        diff::Result::Left(line) => {
                            file_changes.push(format!("- {}", line));
                        },
                        diff::Result::Right(line) => {
                            file_changes.push(format!("+ {}", line));
                        },
                        diff::Result::Both(line, _) => {
                            file_changes.push(format!("  {}", line));
                        }
                    }
                }
                changes.insert(path.clone(), file_changes);
            }
        }
    }
    changes
}

// extract both mod zip files to the cache, naming them mod1 and mod2
fn extract_mods(path_one: &str, path_two: &str) -> (PathBuf, PathBuf) {
    let temp_dir = cache::create_temp();
    let mod_one = Path::new(path_one);
    let mod_two = Path::new(path_two);
    let dest_one = temp_dir.join("mod1");
    let dest_two = temp_dir.join("mod2");
    compression::decompress_directory(mod_one, &dest_one).unwrap();
    compression::decompress_directory(mod_two, &dest_two).unwrap();
    (dest_one, dest_two)
}

// create a diff between two mods in the format "human readable"
fn format_human_readable(mod1: &HashMap<PathBuf, u32>, mod2: &HashMap<PathBuf, u32>) -> String {
    let added = added_files(mod1, mod2);
    let removed = removed_files(mod1, mod2);
    let changed = changed_files(mod1, mod2);
    let unchanged = unchanged_files(mod1, mod2);
    let moved = moved_files(mod1, mod2);
    let changes = file_changes(mod1, mod2);
    let mut output = String::new();
    output.push_str(&format!("Added files: {}\n", added.len()));
    for file in added {
        output.push_str(&format!("  {}\n", file.to_str().unwrap()));
    }
    output.push_str(&format!("Removed files: {}\n", removed.len()));
    for file in removed {
        output.push_str(&format!("  {}\n", file.to_str().unwrap()));
    }
    output.push_str(&format!("Changed files: {}\n", changed.len()));
    for file in changed {
        output.push_str(&format!("  {}\n", file.to_str().unwrap()));
        for change in changes.get(&file).unwrap() {
            output.push_str(&format!("    {}\n", change));
        }
    }
    output.push_str(&format!("Unchanged files: {}\n", unchanged.len()));
    for file in unchanged {
        output.push_str(&format!("  {}\n", file.to_str().unwrap()));
    }
    output.push_str(&format!("Moved files: {}\n", moved.len()));
    for (old_file, new_file) in moved {
        output.push_str(&format!("  {} -> {}\n", old_file.to_str().unwrap(), new_file.to_str().unwrap()));
    }
    output
}

// front end receiver for the diff command
#[command]
pub fn find_differences(window: Window, mod_one: String, mod_two: String, format: String, output_path: String) {
    // sanity checks
    if !mod_one.ends_with(".zip") || !mod_two.ends_with(".zip") {
        window.emit("error", "Both mods need to be zip files").unwrap();
        return;
    }
    if !Path::new(&mod_one).exists() || !Path::new(&mod_two).exists() {
        window.emit("error", "Both mods need to exist").unwrap();
        return;
    }
    // extract the mods and create the hashmaps
    window.emit("status", "Extracting the mod files..").unwrap();
    let (mod_one, mod_two) = extract_mods(&mod_one, &mod_two);
    window.emit("status", "Creating hashmap for mod one..").unwrap();
    let mod_one = create_hashmap(mod_one.to_str().unwrap());
    window.emit("status", "Creating hashmap for mod two..").unwrap();
    let mod_two = create_hashmap(mod_two.to_str().unwrap());
    // find the difference and format it
    window.emit("status", "Finding differences..").unwrap();
    let output = match format.as_str() {
        "human_readable" => format_human_readable(&mod_one, &mod_two),
        _ => format_human_readable(&mod_one, &mod_two)
    };
    window.emit("status", "Cleaning up..").unwrap();
    cache::clear_temp();
    // write to output_path + differences-(time).txt
    let output_path = Path::new(&output_path).join(format!("differences-{}.txt", chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")));
    let mut file = fs::File::create(output_path).expect("Unable to create file");
    file.write_all(output.as_bytes()).expect("Unable to write to file");
    window.emit("status", "Generated differences!").unwrap();
}