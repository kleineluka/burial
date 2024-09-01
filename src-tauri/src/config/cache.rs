// imports
use std::path::PathBuf;
use dirs::config_dir;
use crate::files;

// make sure cache folder exists
pub fn verify_cache() -> std::io::Result<()> {
    // %appdata% on windows, $XDG_CONFIG_HOME or ~/.config on linux, ~/Library/Application Support on mac (i think)
    let mut burial_dir: PathBuf = config_dir().unwrap_or_else(|| PathBuf::from("."));
    burial_dir.push("burial");
    files::verify_folder(&burial_dir)?;
    Ok(())
}
