// imports
use std::fs;
use std::io::{self};

// replace string in a file (assuming it can be read as text)
pub fn replace_text(file_path: &str, old_text: &str, new_text: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let updated_content = content.replace(old_text, new_text);
    fs::write(file_path, updated_content)?;
    Ok(())
}