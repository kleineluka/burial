use tauri::Manager;

// imports
use crate::utils::files;
use crate::utils::deno;
use crate::config::cache;
use crate::config::storage;

// create hausmaerchen path if it doesn't exist
pub fn verify_hausmaerchen() {
    // get the cache + deno + deno.exe
    let cache = cache::cache_folder();
    let haus_path = cache.join("hausmaerchen");
    // create the deno folder if it doesn't exist
    if !haus_path.exists() {
        std::fs::create_dir_all(&haus_path).unwrap();
    }
}

// get the hausmaerchen path
pub fn hausmaerchen_path() -> std::path::PathBuf {
    let _ = verify_hausmaerchen();
    let cache = cache::cache_folder();
    cache.join("hausmaerchen")
}

// get the hausmaerchen entry point
pub fn hausmaerchen_entry() -> std::path::PathBuf {
    let cache = cache::cache_folder();
    let haus_path = cache.join("hausmaerchen");
    haus_path.join("main.ts")
}

// see if hausmaerchen is installed
pub fn hausmaerchen_installed() -> bool {
    let haus_path = hausmaerchen_path();
    haus_path.join("main.ts").exists()
}

// one-stop setup for hausmaerchen, every time it's called
pub fn hausmaerchen_setup(app: &tauri::AppHandle) {
    if hausmaerchen_installed() {
        return;
    }
    // first, verify + get the hausmaerchen path
    let haus_path = hausmaerchen_path();
    // next, get the resource path and join it with hausmaerchen
    let resource_path = storage::read_from_store(app, "state-bundled-resources").expect("Failed to read from store");
    let mut resource_haus_path = std::path::PathBuf::from(resource_path.as_str().unwrap());
    resource_haus_path.push("hausmaerchen");
    // copy the resource hausmaerchen to the cache hausmaerchen
    files::copy_directory(&resource_haus_path.to_string_lossy(), &haus_path.to_string_lossy()).unwrap();
}

pub fn remove_hausmaerchen() {
    let haus_path = hausmaerchen_path();
    std::fs::remove_dir_all(&haus_path).unwrap();
}

pub fn run_hausmaerchen(window: &tauri::Window, code_path: String, out_path: String, add_comments: bool, rename_variables: bool) -> String {
    // set up if needed
    hausmaerchen_setup(&window.app_handle());
    // get necessary paths to run deno
    let deno_exe = deno::deno_executable();
    let haus_entry = hausmaerchen_entry();
    // the command will look like:
    //  deno run --allow-read --allow-write=output_file --allow-env "ENTRYPOINT" --in-path="INPATH" --out-path="OUTPATH" --add-comments=ADDCOMMENTS --rename-variables=RENAMEVARIABLES
    let mut command = std::process::Command::new(deno_exe);
    command.arg("run")
        .arg("--allow-env")
        .arg("--allow-read")
        .arg("--allow-write") // sorry, a bit janky when only allowing reading/writing to a specific file
        .arg(haus_entry)
        .arg(format!("--code-path={}", code_path))
        .arg(format!("--out-path={}", out_path))
        .arg(format!("--add-comments={}", add_comments))
        .arg(format!("--rename-variables={}", rename_variables));
    // run the command
    let output = command.output().expect("Failed to execute Deno command");
    // Check the command's output
    if output.status.success() {
        let _ = String::from_utf8_lossy(&output.stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
    // return dummy string for now
    "hausmaerchen".to_string()
}