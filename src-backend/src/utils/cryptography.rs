// imports
use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use cfb8::Decryptor;
use aes::Aes256;
use hex::decode;
use std::fs;
use std::path::Path;

// extract the current decryption key from teh game 
// PLEASE NOTE: this plugin is not used by the game yet. (Decryption.js)
fn extract_key(base_path: &str) -> Option<String> {
    // create a path and navigate to the target
    let base_path = Path::new(base_path);
    let target_path = base_path.join("www/js/plugins/Decryption.js");
    // make sure the file exists
    if !target_path.exists() {
        eprintln!("File does not exist: {:?}", target_path);
        let error = format!("File does not exist: {:?}", target_path);
        return Some(error);
    }
    // read the file contents
    let content = match fs::read_to_string(&target_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read the file: {:?}", e);
            let error = format!("Failed to read the file: {:?}", e);
            return Some(error);
        }
    };
    // filter the contents to find the key
    if let Some(start_index) = content.find("let decryptionAESKey = \"") {
        let key_start = start_index + "let decryptionAESKey = \"".len();
        if let Some(end_index) = content[key_start..].find("\"") {
            let key = content[key_start..key_start + end_index].to_string();
            // ensure the key is valid (64 characters)
            if key.len() != 64 {
                eprintln!("Invalid key length: {:?}", key.len());
                let error = format!("Invalid key length: {:?}", key.len());
                return Some(error);
            }
            return Some(key);
        }
    }
    let error = "Failed to extract the key.".to_string();
    Some(error)
}

// convert a key (hex string) to a byte array
// PLEASE NOTE: this plugin is not used by the game yet. (Decryption.js)
fn convert_key(key: &str) -> Result<[u8; 32], String> {
    // convert the key to bytes
    let key_bytes = match decode(key) {
        Ok(key_bytes) => key_bytes,
        Err(_) => return Err("INVALID_KEY".into()),
    };
    // check the length of the key
    if key_bytes.len() != 32 {
        return Err("INVALID_LENGTH".into());
    }
    // create a key array + copy + return
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes);
    Ok(key)
}

// get a key and convert it to a byte array (just perform the two steps above..)
// PLEASE NOTE: this plugin is not used by the game yet. (Decryption.js)
pub fn get_key(base_path: &str) -> Result<[u8; 32], String> {
    let key = match extract_key(base_path) {
        Some(key) => key,
        None => return Err("Failed to extract the key.".into()),
    };
    convert_key(&key)
}

// decypt a stream of data with a decryiption key
// PLEASE NOTE: this plugin is not used by the game yet. (Decryption.js)
pub fn decryption(encrypted_data: Vec<u8>, decryption_key: [u8; 32]) -> Result<Vec<u8>, String> {
    // set up the decryption cipher
    let iv = [0u8; 16]; // default initialization vector
    let mut cipher = Decryptor::<Aes256>::new(&decryption_key.into(), &iv.into());
    // create a mutable buffer to hold the decrypted data + decrypt the data + return
    let mut decrypted_data = encrypted_data.clone();
    cipher.decrypt(&mut decrypted_data);
    Ok(decrypted_data)
}
