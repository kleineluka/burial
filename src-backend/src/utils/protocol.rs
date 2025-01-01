#[cfg(target_os = "windows")]
// example input: "burial"
pub fn register_protocol(subkey: String) -> std::io::Result<()> {
    // windows-only imports
    use winreg::enums::*;
    use winreg::RegKey;
    // create the "burial" key under HKEY_CLASSES_ROOT
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    // if the key already exists, skip
    if hkcr.open_subkey(&subkey).is_ok() {
        return Ok(());
    }
    let (key, _) = hkcr.create_subkey(&subkey)?;
    // description, command
    let protocol_description = format!("URL:{} Protocol", subkey);
    key.set_value("", &protocol_description)?;
    key.set_value("URL Protocol", &"")?;
    let (command_key, _) = key.create_subkey("shell\\open\\command")?;
    // path to the application executable with "%1" as an argument
    let exe_path = std::env::current_exe()?.to_str().unwrap().to_string();
    command_key.set_value("", &format!("\"{}\" \"%1\"", exe_path))?;
    Ok(())
}

// example input: "burial-app", "burial"
#[cfg(target_os = "macos")]
pub fn register_protocol(app_name: &str, scheme_name: &str) -> std::io::Result<()> {
    // macos-only imports
    use std::fs;
    use std::path::Path;
    // path to the Info.plist file 
    let plist_path = format!("/Applications/{}/Contents/Info.plist", app_name);
    // check if Info.plist exists
    if !Path::new(&plist_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Info.plist not found",
        ));
    }
    // read the Info.plist content
    let plist_content = fs::read_to_string(&plist_path)?;
    // check if the custom URL scheme already exists in the Info.plist
    if plist_content.contains(&format!("<string>{}</string>", scheme_name)) {
        println!("Protocol '{}' is already registered.", scheme_name);
        return Ok(()); // Skip registration
    }
    // add the CFBundleURLTypes entry with the custom scheme if not already present
    let updated_plist_content = plist_content.replace(
        "</dict>",
        &format!(
            r#"
            <key>CFBundleURLTypes</key>
            <array>
                <dict>
                    <key>CFBundleURLSchemes</key>
                    <array>
                        <string>{}</string>
                    </array>
                </dict>
            </array>
            </dict>"#,
            scheme_name
        ),
    );
    // write the updated Info.plist back
    fs::write(&plist_path, updated_plist_content)?;
    Ok(())
}

// example input: "burial-app", "Burial App", "burial"
#[cfg(target_os = "linux")]
pub fn register_protocol(app_name: String, entry_name: String, scheme_name: String) -> std::io::Result<()> {
    // linux-only imports
    use std::fs;
    use std::process::Command;
    // path to the .desktop file
    let desktop_file_path = format!(
        "/usr/share/applications/{}.desktop",
        app_name.to_lowercase()
    );
    // check if the .desktop file already exists
    if Path::new(&desktop_file_path).exists() {
        let desktop_content = fs::read_to_string(&desktop_file_path)?;
        // check if the MIME type already exists for the given URL scheme
        if desktop_content.contains(&format!("x-scheme-handler/{}", scheme_name)) {
            println!("Protocol '{}' is already registered.", scheme_name);
            return Ok(()); 
        }
    }
    // create the .desktop file
    let desktop_content = format!(
        "[Desktop Entry]
Name={}
Exec={} %u
Type=Application
MimeType=x-scheme-handler/{};",
        entry_name, 
        std::env::current_exe()?.to_str().unwrap(),
        scheme_name 
    );
    fs::write(&desktop_file_path, desktop_content)?;
    // register the protocol handler
    let _ = Command::new("xdg-mime")
        .args([
            "default",
            &format!("{}.desktop", app_name.to_lowercase()),
            &format!("x-scheme-handler/{}", scheme_name),
        ])
        .status()?;
    Ok(())
}