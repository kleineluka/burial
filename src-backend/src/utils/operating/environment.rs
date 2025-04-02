// imports
use sysinfo::System;
use blake3::Hasher;

// get the operating system
pub fn get_os() -> &'static str {
    match () {
        _ if cfg!(target_os = "windows") => "windows",
        _ if cfg!(target_os = "macos") => "macos",
        _ if cfg!(target_os = "linux") => "linux",
        _ => "windows", // default to windows for burial..
    }
}

// create a hwid
pub fn get_hwid() -> String {
    // get system info for hwid
    let mut sys = System::new_all();
    sys.refresh_all();
    let os_name = System::name().unwrap_or("Unknown OS".to_string());
    let cpu_brand = sys.cpus().get(0).map_or("Unknown CPU".to_string(), |cpu| cpu.brand().to_string());
    let total_memory = sys.total_memory();
    let host_name = System::host_name().unwrap_or("Unknown Host".to_string());
    // hash the hwid
    let mut hasher = Hasher::new();
    hasher.update(os_name.as_bytes());
    hasher.update(cpu_brand.as_bytes());
    hasher.update(total_memory.to_string().as_bytes());
    hasher.update(host_name.as_bytes());
    let hwid = hasher.finalize();
    // shorten the hwid and return it
    // note: as you can see, only the hash is returned! no identifiable information is needed!
    let pretty_hwid = hwid.to_hex().chars().take(14).collect::<String>();
    pretty_hwid
}

// get a folder path from Tauri's resources
pub fn get_resources(app: &tauri::App) -> std::path::PathBuf {
    let resource_path = app.path_resolver().resolve_resource("bundled").unwrap();
    // remove the \\\\?\\ prefix
    let resource_path = resource_path.to_string_lossy().to_string();
    let resource_path = resource_path.replace("\\\\?\\", "");
    std::path::PathBuf::from(&resource_path)
}