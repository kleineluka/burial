// imports
use std::{collections::HashMap, fs, io::{self, ErrorKind}, path::Path};
use serde::{Deserialize, Serialize};

// nemlang format (can be loaded from csv, txt, or loc)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NemLang {
    pub lang_name: String,
    pub lang_info: [String; 3],
    pub font_face: String,
    pub font_size: i32,
    pub sys_label: HashMap<String, String>,
    pub sys_menus: HashMap<String, String>,
     #[serde(rename = "labelLUT")]
    pub label_lut: HashMap<String, String>,
    #[serde(rename = "linesLUT")]
    pub lines_lut: HashMap<String, Vec<String>>,
}

impl NemLang {
    fn new() -> Self {
        Self {
            lang_name: String::new(),
            lang_info: ["".to_string(), "".to_string(), "".to_string()],
            font_face: String::new(),
            font_size: 0,
            sys_label: HashMap::new(),
            sys_menus: HashMap::new(),
            label_lut: HashMap::new(),
            lines_lut: HashMap::new(),
        }
    }
}

// constant signature
pub const LOC_SIGNATURE: &str = "00000NEMLEI00000";
pub const LOC_OFFSET: usize = 4;

// see if a signature is present in a file
fn has_signature(file_path: &str) -> Result<bool, io::Error> {
    // read the file content
    let data = fs::read(file_path)?;
    let signature_len = LOC_SIGNATURE.len();
    // check if the signature is present
    Ok(data.windows(signature_len).any(|window| window == LOC_SIGNATURE.as_bytes()))
}

// load languages from .txt files
pub fn load_txt(file_path: &str) -> Result<NemLang, io::Error> {
    // read the file content
    let mut data = NemLang::new();
    let file_content = fs::read_to_string(file_path)?;
    let mut current_section = String::new();
    let mut current_lines: Vec<String> = Vec::new();
    let mut is_label_mode = false;
    // go through line by line
    for line in file_content.lines() {
        let line = line.trim();
        // skip empty lines
        if line.is_empty() {
            continue;
        }
        // check for section
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_uppercase();
            is_label_mode = current_section == "LABELS";
            current_lines.clear();
            continue;
        }
        // match the section of the language file with the nemlang struct
        match current_section.as_str() {
            "LANGUAGE" => data.lang_name = line.to_string(),
            "VERSION" => {
                if let Some(version) = line.split(':').nth(1) {
                    data.font_size = version.trim().parse().unwrap_or(0);
                }
            }
            "FONTFACE" => data.font_face = line.to_string(),
            "FONTSIZE" => {
                data.font_size = line.parse().unwrap_or(0);
            }
            "LANGINFO" => {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    data.lang_info = [
                        parts[0].trim().to_string(),
                        parts[1].trim().to_string(),
                        parts[2].trim().to_string(),
                    ];
                }
            }
            "LABELS" if is_label_mode => {
                if let Some((key, value)) = line.split_once(':') {
                    data.label_lut.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            _ if line.contains(':') => {
                if let Some((key, value)) = line.split_once(':') {
                    let section_map = match current_section.as_str() {
                        "SYSLABEL" => &mut data.sys_label,
                        "MENUS" => &mut data.sys_menus,
                        _ => continue,
                    };
                    section_map.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
            _ => {
                if current_section == "LINES" {
                    if let Some((key, value)) = line.split_once(':') {
                        let entry = data.lines_lut.entry(key.trim().to_string()).or_insert(vec![]);
                        entry.push(value.trim().to_string());
                    }
                }
            }
        }
    }
    // return the nemlang structure (that hopefully parsed..)
    Ok(data)
}

// load languages from .csv files
pub fn load_csv(file_path: &str) -> Result<NemLang, io::Error> {
    // read the file content
    let mut data = NemLang::new();
    let file_content = fs::read_to_string(file_path)?;
    let mut current_section = String::new();
    // go through line by line
    for line in file_content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // split the line by commas
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_quotes = false;
        let chars: Vec<char> = line.chars().collect();
        // go through each character
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            match c {
                '"' if !in_quotes => in_quotes = true,
                '"' if in_quotes && i + 1 < chars.len() && chars[i + 1] == '"' => {
                    current_token.push('"');
                    i += 1;
                }
                '"' if in_quotes => in_quotes = false,
                ',' if !in_quotes => {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                _ => current_token.push(c),
            }
            i += 1;
        }
        tokens.push(current_token.trim().to_string());
        // skip empty lines
        if tokens.len() < 1 || tokens[0].is_empty() {
            continue;
        }
        // update the current section based on the first token
        if !tokens[0].is_empty() {
            current_section = tokens[0].to_uppercase();
        }
        // match the section of the language file with the nemlang struct
        match current_section.as_str() {
            "LANGUAGE" => {
                if tokens.len() >= 4 {
                    data.lang_name = tokens[1].clone();
                    data.font_face = tokens[2].clone();
                    data.font_size = tokens[3].parse().unwrap_or(0);
                }
            }
            "LANGINFO" => {
                for (j, item) in tokens.iter().skip(1).take(3).enumerate() {
                    data.lang_info[j] = item.clone();
                }
            }
            "SYSLABEL" => {
                if tokens.len() >= 3 {
                    data.sys_label.insert(tokens[1].clone(), tokens[2].clone());
                }
            }
            "MENUS" => {
                if tokens.len() >= 3 {
                    data.sys_menus.insert(tokens[1].clone(), tokens[2].clone());
                }
            }
            "LABELS" => {
                if tokens.len() >= 3 {
                    data.label_lut.insert(tokens[1].clone(), tokens[2].clone());
                }
            }
            "LINES" => {
                if tokens.len() >= 3 {
                    data.lines_lut
                        .entry(tokens[1].clone())
                        .or_insert_with(Vec::new)
                        .push(tokens[2].clone());
                }
            }
            _ => {
                println!("Unknown section: {}", current_section);
            }
        }
    }
    // return the nemlang structure (that hopefully parsed..)
    Ok(data)
}

// load languages from .loc files
pub fn load_loc(file_path: &str) -> Result<NemLang, io::Error> {
    // read the file content
    let json_str = if !has_signature(file_path)? {
        // it's a modified one from the project, so just read it as a json file
        fs::read_to_string(file_path)?
    } else {
        // locate the signature 
        let data = fs::read(file_path)?;
        let signature_len = LOC_SIGNATURE.len();
        if let Some(start_index) = data.windows(signature_len).position(|window| window == LOC_SIGNATURE.as_bytes()) {
            let sliced_data = &data[start_index + signature_len + LOC_OFFSET..];
            String::from_utf8(sliced_data.to_vec()).map_err(|_| io::Error::new(ErrorKind::InvalidData, "Invalid UTF-8 data"))?
        } else {
            return Err(io::Error::new(ErrorKind::NotFound, "Signature not found"));
        }
    };
    // parse the json string
    serde_json::from_str::<NemLang>(&json_str)
        .map(|mut result| {
            // Initialize sys_label and sys_menus if they are empty
            if result.sys_label.is_empty() {
                result.sys_label.insert("default".to_string(), String::new());
            }
            if result.sys_menus.is_empty() {
                result.sys_menus.insert("default".to_string(), String::new());
            }
            result
        })
        .map_err(|e| {
            eprintln!("Failed to parse JSON: {}", e);
            io::Error::new(ErrorKind::InvalidData, "Failed to parse JSON")
        })
}

// parse a file
pub fn load_nemlang(file_path: &str) -> Result<NemLang, io::Error> {
    // check the file extension
    let ext = match Path::new(file_path).extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => return Err(io::Error::new(ErrorKind::InvalidData, "Invalid file extension")),
    };
    // load the file based on the extension
    match ext.to_lowercase().as_str() {
        "txt" => load_txt(file_path),
        "csv" => load_csv(file_path),
        "loc" => load_loc(file_path),
        _ => Err(io::Error::new(ErrorKind::InvalidData, "Invalid file extension")),
    }
}