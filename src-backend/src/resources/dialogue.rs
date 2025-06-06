// imports
use tauri::Window;
use tauri::command;
use std::collections::HashMap;
use std::path::Path;
use serde::Serialize;
use serde::Deserialize;
use crate::utils::operating::game;
use crate::utils::helpers::files;

// structure from the game's language files
#[allow(non_snake_case)] // can also handle converting camel, but might need to do LUT specifically since its all caps
#[derive(Serialize, Deserialize, Debug)]
struct Loc {
    langName: String,
    langInfo: Vec<String>,
    fontFace: String,
    fontSize: u32,
    sysLabel: HashMap<String, String>,
    sysMenus: HashMap<String, String>, 
    labelLUT: HashMap<String, String>, 
    linesLUT: HashMap<String, Vec<String>>,
}

// structures that will be passed for configuration
#[derive(Deserialize, Debug)]
pub struct LanguageDetails {
    shortcode: String,
    path: String,
}

#[derive(Deserialize, Debug)]
pub struct ContentDetails {
    content: String,
}

#[derive(Deserialize, Debug)]
pub struct FormatDetails {
    #[serde(rename = "type")]
    format_type: String,
    extension: String,
}

// the string used at the start of the loc files
// to-do: move to bytes
pub const LOC_HEADER: &str = "00000NEMLEI00000X ";

// remove the loc header from the language file
pub fn remove_loc_header(s: &str) -> String {
    let mut s = s.to_string();
    s.replace_range(0..LOC_HEADER.len(), "");
    s
}

// add loc_header to the start of the file (string)
fn add_loc_header(s: &str) -> String {
    let mut s = s.to_string();
    s.insert_str(0, LOC_HEADER);
    s
}

// format into minified json (= minify json)
fn format_minified_json(s: &str) -> String {
    let s = serde_json::to_string(s).unwrap();
    s
}

// format into loc (= minify json, then add header)
fn format_loc(s: &str) -> String {
    let s = serde_json::to_string(s).unwrap();
    add_loc_header(&s)
}

// format into text (= remove brackets, commas, and quotes)
fn format_text(s: &str) -> String {
    let mut s = s.to_string();
    s = s.replace("{", "");
    s = s.replace("}", "");
    s = s.replace(",", "");
    s = s.replace("\"", "");
    s = s.replace("\n\n", "\n");
    s
}

// format into text with values only (= to text, remove keys)
fn format_text_values(s: &str) -> String {
    let mut s = s.to_string();
    s = format_text(&s);
    let lines: Vec<&str> = s.split("\n").collect();
    let mut new_lines = Vec::new();
    for line in lines {
        let line: Vec<&str> = line.split(":").collect();
        new_lines.push(line[1]);
    }
    s = new_lines.join("\n");
    s
}

// format into text with keys only (= to text, remove values)
fn format_text_keys(s: &str) -> String {
    let mut s = s.to_string();
    s = format_text(&s);
    let lines: Vec<&str> = s.split("\n").collect();
    let mut new_lines = Vec::new();
    for line in lines {
        let line: Vec<&str> = line.split(":").collect();
        new_lines.push(line[0]);
    }
    s = new_lines.join("\n");
    s
}

// format into csv (= to text, replace newlines with commas)
fn format_csv(s: &str) -> String {
    let mut s = s.to_string();
    s = format_text(&s);
    s = s.replace("\n", ",");
    s
}

// format into csv with keys only (= to csv, remove values)
fn format_csv_keys(s: &str) -> String {
    let mut s = s.to_string();
    s = format_csv(&s);
    let lines: Vec<&str> = s.split(",").collect();
    let mut new_lines = Vec::new();
    for line in lines {
        let line: Vec<&str> = line.split(":").collect();
        new_lines.push(line[0]);
    }
    s = new_lines.join(",");
    s
}

// format into csv with values only (= to csv, remove keys)
fn format_csv_values(s: &str) -> String {
    let mut s = s.to_string();
    s = format_csv(&s);
    let lines: Vec<&str> = s.split(",").collect();
    let mut new_lines = Vec::new();
    for line in lines {
        let line: Vec<&str> = line.split(":").collect();
        new_lines.push(line[1]);
    }
    s = new_lines.join(",");
    s
}

// generate a dialogue, reused between the export and preview function
fn generate_dialogue(window: &Window, in_path: &String, language_details: &LanguageDetails, 
    content_details: &ContentDetails, format_details: &FormatDetails) -> String {
    // ensure that the path is a game folder
    let is_game = game::verify_game(&in_path).unwrap();
    if !is_game {
        window.emit("error", "Your folder is not a valid TCOAAL folder!").unwrap();
        return "Error".to_string();
    }
    // ensure that the file can be found at in_path + language_details.path
    let file_path = format!("{}/{}", in_path, language_details.path);
    if !Path::new(&file_path).exists() {
        window.emit("error", "The file could not be found!").unwrap();
        return "Error".to_string();
    }
    // read the language file + remove the loc_header
    window.emit("status", "Reading language file..").unwrap();
    let mut language_content = std::fs::read_to_string(&file_path).unwrap();
    language_content = remove_loc_header(&language_content);
    // now we are left with a very big json, so parse it into a Loc struct
    window.emit("status", "Parsing language file..").unwrap();
    let language_json: serde_json::Value = serde_json::from_str(&language_content).unwrap();
    let mut language_selection = &language_json;
    // dynamically(?) match the selection
    if content_details.content.to_lowercase() != "all" {
        match language_json.get(&content_details.content) {
            Some(value) => language_selection = value,
            None => println!("Field '{}' not found in the language JSON.", content_details.content),
        }
    } 
    // turn the selection into a json + match filter in the format_details.format_type
    window.emit("status", "Formatting to desired output..").unwrap();
    let language_selection_json = serde_json::to_string_pretty(&language_selection).unwrap();
    let mut output = String::new();
    match format_details.format_type.as_str() {
        "pretty_json" => output = language_selection_json,
        "minified_json" => output = format_minified_json(&language_selection_json),
        "loc" => output = format_loc(&language_selection_json),
        "text" => output = format_text(&language_selection_json),
        "text_values" => output = format_text_values(&language_selection_json),
        "text_keys" => output = format_text_keys(&language_selection_json),
        "csv" => output = format_csv(&language_selection_json),
        "csv_values" => output = format_csv_values(&language_selection_json),
        "csv_keys" => output = format_csv_keys(&language_selection_json),
        _ => window.emit("error", "Format not supported for exporting dialogue..").unwrap(),
    }
    // return the output
    output
}

// minimize from a json (pretty or minimized) to a loc (= minimize json, then add header)
#[allow(dead_code)]
fn minimize_json_to_loc(s: &str) -> String {
    let s = format_minified_json(s);
    add_loc_header(&s)
}

// generate a loc
#[command]
fn generate_loc(window: &Window, in_path: &String, language_details: &LanguageDetails, 
    content_details: &ContentDetails, format_details: &FormatDetails) -> String {
    // make sure in_path is a file
    let is_file = Path::new(&in_path).is_file();
    if !is_file {
        window.emit("error", "The given dialogue file does not seem to exist..").unwrap();
        return "Error".to_string();
    }
    // read the dialogue file
    window.emit("status", "Reading dialogue file..").unwrap();
    let dialogue_content = std::fs::read_to_string(&in_path).unwrap();
    // as of now, only all content details are supported
    if content_details.content.to_lowercase() != "all" {
        window.emit("error", "Only 'all' content details are supported for importing dialogue files..").unwrap();
        return "Error".to_string();
    }
    // depending on the format, parse the dialogue file
    window.emit("status", "Parsing dialogue file..").unwrap();
    let mut output = String::new();
    match format_details.format_type.as_str() {
        "json" => output = minimize_json_to_loc(&dialogue_content),
        _ => window.emit("error", "Format not supported for importing dialogue files..").unwrap(),
    }
    // return the output
    output
}

// export dialogue
#[command]
pub fn export_dialogue(window: Window, in_path: String, out_path: String, language_details: LanguageDetails, content_details: ContentDetails, format_details: FormatDetails) {
    // generate the dialogue
    let output = generate_dialogue(&window, &in_path, &language_details, &content_details, &format_details);
    if output == "Error" {
        return;
    }
    // write the output to the out_path + language_details.shortcode + "_output_timestamp" + format_details.extension
    let now = chrono::Local::now().format("%d-%m-%Y-%H-%M-%S").to_string();
    let output_path = format!("{}/{}_{}_output_{}.{}", out_path, language_details.shortcode, content_details.content, now, format_details.extension);
    window.emit("status", "Writing output file..").unwrap();
    std::fs::write(&output_path, output).unwrap();
    window.emit("status", "Exported the dialogue!").unwrap();
}

// preview dialogue export
#[command]
pub fn preview_export(window: Window, in_path: String, language_details: LanguageDetails, content_details: ContentDetails, format_details: FormatDetails) {
    // generate the dialogue
    let output = generate_dialogue(&window, &in_path, &language_details, &content_details, &format_details);
    if output == "Error" {
        return;
    }
    // emit the output
    window.emit("status", "Export preview generated!").unwrap();
    window.emit("load-export-preview", output).unwrap();
}

// import dialogue
#[command]
pub fn import_dialogue(window: Window, in_path: String, out_path: String, language_details: LanguageDetails, content_details: ContentDetails, format_details: FormatDetails) {
    // verify that the in_path is a file
    let is_file = Path::new(&in_path).is_file();
    if !is_file {
        window.emit("error", "The given dialogue file does not seem to exist..").unwrap();
        return;
    }
    // verify that the out_path is a game folder
    let is_game = game::verify_game(&out_path).unwrap();
    if !is_game {
        window.emit("error", "Your folder is not a valid TCOAAL folder!").unwrap();
        return;
    }
    // if a file already exists at out_path + language_details.path, back it up incrementally
    let file_path = format!("{}/{}", out_path, language_details.path);
    if (Path::new(&file_path).exists()) {
        window.emit("status", "Backing up the existing language file..").unwrap();
        let _ = files::backup_file_multiple(&file_path);
    }
    // generate the loc
    let output = generate_loc(&window, &in_path, &language_details, &content_details, &format_details);
    if output == "Error" {
        return;
    }
    // write the output to the out_path + language_details.path
    window.emit("status", "Writing output file..").unwrap();
    std::fs::write(&file_path, output).unwrap();
    window.emit("status", "Imported the dialogue!").unwrap();
}

// preview dialogue import
#[command]
pub fn preview_import(window: Window, in_path: String, language_details: LanguageDetails, content_details: ContentDetails, format_details: FormatDetails) {
    // generate the loc
    let output = generate_loc(&window, &in_path, &language_details, &content_details, &format_details);
    if output == "Error" {
        return;
    }
    // emit the output
    window.emit("status", "Import preview generated!").unwrap();
    window.emit("load-import-preview", output).unwrap();
}