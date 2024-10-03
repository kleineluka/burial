pub fn get_os() -> &'static str {
    match () {
        _ if cfg!(target_os = "windows") => "windows",
        _ if cfg!(target_os = "macos") => "macos",
        _ if cfg!(target_os = "linux") => "linux",
        _ => "windows", // default to windows for burial..
    }
}