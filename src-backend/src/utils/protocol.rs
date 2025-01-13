// constants 
pub const APP_NAME: &str = "burial-app";
pub const SUBKEY_SCHEME_NAME: &str = "burial";
pub const ENTRY_NAME: &str = "Burial App";

// example input: "burial"
#[cfg(target_os = "windows")]
pub fn register_protocol_windows(subkey: String) -> std::io::Result<()> {
    // windows-only imports
    use winreg::enums::*;
    use winreg::RegKey;
    // create the "burial" key under HKEY_CLASSES_ROOT
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS)?;
    // if the key already exists, skip
    if classes.open_subkey(&subkey).is_ok() {
        classes.delete_subkey_all(&subkey)?;
    }
    let create_result = classes.create_subkey(&subkey);
    match create_result {
        Ok((key, _)) => {
        // description, command
        let protocol_description = format!("URL:{} Protocol", subkey);
        key.set_value("", &protocol_description)?;
        key.set_value("URL Protocol", &"")?;
        let (command_key, _) = key.create_subkey("shell\\open\\command")?;
        // path to the application executable with "%1" as an argument
        let exe_path = std::env::current_exe()?.to_str().unwrap().to_string();
        command_key.set_value("", &format!("\"{}\" \"%1\"", exe_path))?;
        return Ok(());
    },
    Err(e) => {
        println!("Failed to create key '{}': {:?}", subkey, e);
        return Err(e);
    }};
    Ok(())
}

// example input: "burial"
#[cfg(target_os = "windows")]
pub fn unregister_protocol_windows(subkey: String) -> std::io::Result<()> {
    // windows-only imports
    use winreg::enums::*;
    use winreg::RegKey;
    // open the "burial" key under HKEY_CLASSES_ROOT
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS)?;
    let burial_key = classes.open_subkey(&subkey);
    // if the key does not exist, skip
    if burial_key.is_err() {
        return Ok(());
    }
    // delete the "burial" key
    classes.delete_subkey_all(&subkey)?;
    Ok(())
}

// example input: "burial-app", "burial"
#[cfg(target_os = "macos")]
pub fn register_protocol_macos(app_name: &str, scheme_name: &str) -> std::io::Result<()> {
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
        return Ok(()); // skip registration, unlike windows, assume not dev environment
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

// example input: "burial-app", "burial"
#[cfg(target_os = "macos")]
pub fn unregister_protocol_macos(app_name: &str, scheme_name: &str) -> std::io::Result<()> {
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
    if !plist_content.contains(&format!("<string>{}</string>", scheme_name)) {
        println!("Protocol '{}' is not registered.", scheme_name);
        return Ok(()); // skip registration, unlike windows, assume not dev environment
    }
    // remove the CFBundleURLTypes entry with the custom scheme
    let updated_plist_content = plist_content.replace(
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
        "</dict>",
    );
    // write the updated Info.plist back
    fs::write(&plist_path, updated_plist_content)?;
    Ok(())
}

// example input: "burial-app", "Burial App", "burial"
#[cfg(target_os = "linux")]
pub fn register_protocol_linux(app_name: String, entry_name: String, scheme_name: String) -> std::io::Result<()> {
    // linux-only imports
    use std::fs;
    use std::path::Path;
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
            return Ok(()); // skip registration, unlike windows, assume not dev environment
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

// example input: "burial-app", "burial"
#[cfg(target_os = "linux")]
pub fn unregister_protocol_linux(app_name: String, scheme_name: String) -> std::io::Result<()> {
    // linux-only imports
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    // path to the .desktop file
    let desktop_file_path = format!(
        "/usr/share/applications/{}.desktop",
        app_name.to_lowercase()
    );
    // check if the .desktop file already exists
    if !Path::new(&desktop_file_path).exists() {
        println!("Protocol '{}' is not registered.", scheme_name);
        return Ok(()); // skip registration, unlike windows, assume not dev environment
    }
    // remove the .desktop file
    fs::remove_file(&desktop_file_path)?;
    // unregister the protocol handler
    let _ = Command::new("xdg-mime")
        .args(["default", "org.gnome.Nautilus.desktop", &format!("x-scheme-handler/{}", scheme_name)])
        .status()?;
    Ok(())
}

// universal command for registering the custom URL scheme
pub fn register_protocol(skip_registration: bool) -> std::io::Result<()> {
    if skip_registration {
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    register_protocol_windows(SUBKEY_SCHEME_NAME.to_string())?;
    #[cfg(target_os = "macos")]
    register_protocol_macos(APP_NAME, SUBKEY_SCHEME_NAME)?;
    #[cfg(target_os = "linux")]
    register_protocol_linux(APP_NAME.to_string(), ENTRY_NAME.to_string(), SUBKEY_SCHEME_NAME.to_string())?;
    Ok(())
}

// universal command for unregistering the custom URL scheme
pub fn unregister_protocol() -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    unregister_protocol_windows(SUBKEY_SCHEME_NAME.to_string())?;
    #[cfg(target_os = "macos")]
    unregister_protocol_macos(APP_NAME, SUBKEY_SCHEME_NAME)?;
    #[cfg(target_os = "linux")]
    unregister_protocol_linux(APP_NAME.to_string(), SUBKEY_SCHEME_NAME.to_string())?;
    Ok(())
}