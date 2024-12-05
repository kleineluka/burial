// imports
use tauri::Window;
use tauri::command;
use crate::utils::game;

// check devtools status
fn devtools_presence(in_path: &str) -> bool {
    // if not a game path, return false
    if !game::verify_game(in_path).unwrap() {
        return false;
    }
    // open the main.js file
    let mainjs_path = game::get_mainjs(in_path);
    let mainjs_content = std::fs::read_to_string(mainjs_path).unwrap();
    // check for devtools (iterate line by line for "start devtools")
    for line in mainjs_content.lines() {
        if line.contains("start: devtools") {
            return true;
        }
    }
    // no devtools found
    false
}

// get presence of devtools
#[command]
pub fn dev_presences(window: Window, in_path: String) {
    window.emit("devtools", devtools_presence(&in_path)).unwrap();
}

// toggle developer tools
#[command]
pub fn toggle_devtools(window: Window, in_path: String, injected_code: String, target_line: String, code_toggle: bool, code_indent: String) {
    // make sure that the in_path is a game path
    if !game::verify_game(&in_path).unwrap() {
        window.emit("error", "Your TCOAAL path doesn't seem to be right..").unwrap();
        return;
    }
    // get the main.js path and make sure it exists
    let mainjs_path = game::get_mainjs(&in_path);
    let mainjs_path_clone = mainjs_path.clone();
    if !mainjs_path.exists() {
        window.emit("error", "Your TCOAAL installation appears to be corrupt or modified.").unwrap();
        return;
    }
    // check if devtools are present
    if !code_toggle {
        // find devtools code and remove it (starts with // start: devtools and ends with // end: devtools)
        window.emit("status", "Reading main.js file..").unwrap();
        let mainjs_content = std::fs::read_to_string(mainjs_path).unwrap();
        let mut new_content = String::new();
        let mut in_code = false;
        window.emit("status", "Looking for devtools to disable..").unwrap();
        for line in mainjs_content.lines() {
            if line.contains("// start: devtools") {
                in_code = !in_code;
            }
            if !in_code {
                new_content.push_str(line);
                new_content.push_str("\n");
            }
            if line.contains("// end: devtools") {
                in_code = !in_code;
            }
        }
        window.emit("status", "Writing back to the main.js file..").unwrap();
        // write the new content to the main.js file
        std::fs::write(mainjs_path_clone, new_content).unwrap();
    } else {
        // if it already has devtools, return
        if devtools_presence(&in_path) {
            window.emit("status", "Devtools are already enabled.").unwrap();
            return;
        }
        // read the main.js file line by line
        let mainjs_content = std::fs::read_to_string(mainjs_path).unwrap();
        // find the line with the target_line and insert a new line after it with the devtools code
        let mut new_content = String::new();
        for line in mainjs_content.lines() {
            new_content.push_str(line);
            new_content.push_str("\n");
            if line.contains(&target_line) {
                // for each line in the devtools code, add it to the main.js file
                for devtools_line in injected_code.lines() {
                    new_content.push_str(&code_indent);
                    new_content.push_str(devtools_line);
                    if devtools_line != injected_code.lines().last().unwrap() {
                        new_content.push_str("\n");
                    }
                }
                new_content.push_str("\n");
            }
        }
        // write the new content to the main.js file
        std::fs::write(mainjs_path_clone, new_content).unwrap();
    }
}