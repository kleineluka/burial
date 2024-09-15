// imports
use tauri::Window;
use tauri::command;
use open;
use crate::utils::files;

// re-used error message for non-existent files
fn verify_file(window: &Window, file_path: &str) -> bool {
    if files::file_exists(file_path) == false {
        window.emit("error", "Either the file could not be found in the game folder or your injection code is not set.").unwrap();
        return false;
    }
    true
}

// read a file, use the before/after positions, and insert code (with indentation)
fn inject(game_path: String, in_path: String, before: String, after: String, code_path: String, indentation: i32, use_placeholder: bool) -> Result<String, String> {
    // read files
    let combined_path = format!("{}/{}", game_path, in_path);
    let base_code = files::read_file(&combined_path);
    let injected_code = match use_placeholder {
        true => Vec::from("/* Your Code Here! */"),
        false => files::read_file(&code_path),
    };
    // convert to strings
    let mut base_code = String::from_utf8(base_code).expect("Failed to convert base code to UTF-8");
    let mut injected_code = String::from_utf8(injected_code).expect("Failed to convert injected code to UTF-8");
    // add indentation
    let indent_spaces = "   ".repeat(indentation as usize);
    injected_code = injected_code
        .lines()
        .map(|line| format!("{}{}", indent_spaces, line)) 
        .collect::<Vec<_>>()
        .join("\n");
     // Find the positions where the injection should happen
    if let Some(before_pos) = base_code.find(&before) {
        if let Some(after_pos) = base_code[before_pos..].find(&after) {
            let insertion_point = before_pos + after_pos + after.len();
            base_code.insert_str(insertion_point, &format!("\n{}", injected_code));
            return Ok(base_code);
        }
    }
    Ok(base_code)
}

// open a new window
#[command]
pub fn injection_backup(window: Window, game_path: String, in_path: String) {
    window.emit("status", "Backing up file..").unwrap();
    let combined_path = format!("{}/{}", game_path, in_path);
    if verify_file(&window, &combined_path) == false {
        return;
    }
    let _ = files::backup_file_multiple(&combined_path);
    window.emit("status", "File backed up!").unwrap();
}

// open the file in the default editor
#[command]
pub fn injection_open_file(window: Window, game_path: String, in_path: String) {
    let combined_path = format!("{}/{}", game_path, in_path);
    if verify_file(&window, &combined_path) == false {
        return;
    }
    let _ = open::that(combined_path);
    window.emit("status", "File opened!").unwrap();
}

// open the folder
#[command]
pub fn injection_open_folder(window: Window, game_path: String, in_path: String) {
    let combined_path = format!("{}/{}", game_path, in_path);
    let folder_path = combined_path.rsplitn(2, '/').last().unwrap();
    let _ = open::that(folder_path);
    window.emit("status", "Folder opened!").unwrap();
}

// preview the injection
#[command]
pub fn injection_preview(window: Window, game_path: String, in_path: String, before: String, after: String, code_path: String, indentation: i32) {
    // read the file to inject + verify
    window.emit("status", "Generating preview of the injection..").unwrap();
    let combined_path = format!("{}/{}", game_path, in_path);
    if verify_file(&window, &combined_path) == false {
        return;
    }
    // see if code_path exists, if not, use a placeholder
    let use_placeholder = files::file_exists(&code_path) == false;
    let result = inject(game_path, in_path, before, after, code_path, indentation, use_placeholder);
    match result {
        Ok(injected_code) => {
            window.emit("preview", injected_code).unwrap();
            window.emit("status", "Injection preview generated!").unwrap();
        },
        Err(e) => {
            window.emit("error", format!("Failed to preview injection: {}", e)).unwrap();
        }
    }
}

// inject the code and save to file
#[command]
pub fn injection_save(window: Window, game_path: String, in_path: String, before: String, after: String, code_path: String, indentation: i32) {
    // read the file to inject + verify
    window.emit("status", "Generating preview of the injection..").unwrap();
    let combined_path = format!("{}/{}", game_path, in_path);
    if verify_file(&window, &combined_path) == false {
        return;
    }
    // read the code to inject + verify
    if verify_file(&window, &code_path) == false {
        return;
    }
    // inject the code
    let result = inject(game_path, in_path, before, after, code_path, indentation, false);
    match result {
        Ok(injected_code) => {
            // write the file
            let _ = files::write_file(&combined_path, injected_code.as_bytes());
            window.emit("status", "Injection saved!").unwrap();
        },
        Err(e) => {
            window.emit("error", format!("Failed to save injection: {}", e)).unwrap();
        }
    }
}