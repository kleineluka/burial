use std::fs;
use std::io; // Keep io for error type if needed, though not returned
use std::path::Path;
use tree_magic;

// LEGACY: the file extension is stored in the header of the file..
pub fn parse_header(data: &[u8]) -> String {
    // sanity check: data is empty
    if data.is_empty() {
        return String::from("k9a");
    }
    // sanity check: header length is greater than data length
    let header_length = data[0] as usize;
    if header_length + 1 > data.len() {
        return String::from("k9a");
    }
    // get the extension from the header
    let extension_bytes = &data[1..1 + header_length];
    match String::from_utf8(extension_bytes.to_vec()) {
        Ok(extension) => extension,
        Err(_) => String::new(),
    }
}

// LEGACY: create header based on file extension
pub fn create_header(extension: &str) -> Vec<u8> {
    let mut header = Vec::new();
    header.push(extension.len() as u8);
    header.extend_from_slice(extension.as_bytes());
    header
}

// lol lazy
pub fn get_extension_from_mime(data: &[u8]) -> &'static str {
    let mime = tree_magic::from_u8(data);
    match mime.as_str() {
        "image/png" => "png",
        "image/gif" => "gif",
        "audio/ogg" => "ogg",
        "text/plain" => "txt",
        "application/json" => "json",
        _ => "dat", // fallback extension
    }
}

const ASSET_SIG: &[u8] = b"TCOAAL";
const SIG_LEN: usize = ASSET_SIG.len();

// Generates a value based on the file stem (name without extension) in uppercase.
fn make_mask(input_path_str: &str) -> i32 {
    let path = Path::new(input_path_str);
    let filename_stem = path
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_uppercase();

    if filename_stem.is_empty() {
        return 0;
    }

    let mut mask_value: i32 = 0;
    for c in filename_stem.chars() {
        mask_value = mask_value.wrapping_shl(1) ^ (c as i32);
    }
    mask_value
}

// decrypt
pub fn decrypt(data: &[u8], file_path: &str) -> Vec<u8> {
    if data.len() < SIG_LEN + 1 || !data.starts_with(ASSET_SIG) {
        return data.to_vec(); 
    }
    let bytes_to_decrypt_indicator = data[SIG_LEN];
    let payload = &data[SIG_LEN + 1..];
    let payload_len = payload.len();
    if payload_len == 0 {
        return Vec::new();
    }
    let num_bytes_to_decrypt = if bytes_to_decrypt_indicator == 0 {
        payload_len
    } else {
        (bytes_to_decrypt_indicator as usize).min(payload_len)
    };
    let mask_val = make_mask(file_path);
    let mut xor_key: u8 = mask_val.wrapping_add(1).rem_euclid(256) as u8;
    let mut decrypted_payload = payload.to_vec();
    for i in 0..num_bytes_to_decrypt {
        let encrypted_byte = payload[i];
        decrypted_payload[i] = encrypted_byte ^ xor_key;
        xor_key = xor_key.wrapping_shl(1) ^ encrypted_byte;
    }
    decrypted_payload
}


// encrypt
pub fn encrypt(data: &[u8], file_path: &str, advanced_positions: bool) -> Vec<u8> {
    let data_len = data.len();
    let (num_bytes_to_encrypt, indicator) = if advanced_positions {
        let extension = Path::new(file_path)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("");
        let limit = match extension {
            _ => data_len, // Default to full length for now, k9a not used anymore
        };
        let potential_encrypt_count = limit.min(data_len);
        if potential_encrypt_count >= data_len || potential_encrypt_count > 255 {
             (data_len, 0u8) // Encrypt all, indicator 0
        } else {
            (potential_encrypt_count, potential_encrypt_count as u8) 
        }
    } else {
        (data_len, 0u8) 
    };
    let mut encrypted_data = Vec::with_capacity(SIG_LEN + 1 + data_len);
    encrypted_data.extend_from_slice(ASSET_SIG);
    encrypted_data.push(indicator);
    let mask_val = make_mask(file_path);
    let mut xor_key: u8 = mask_val.wrapping_add(1).rem_euclid(256) as u8;
    for i in 0..data_len {
        let original_byte = data[i];
        if i < num_bytes_to_encrypt {
            let encrypted_byte = original_byte ^ xor_key;
            encrypted_data.push(encrypted_byte);
            xor_key = xor_key.wrapping_shl(1) ^ encrypted_byte;
        } else {
            encrypted_data.push(original_byte); 
        }
    }
    encrypted_data
}


// --- File Handling Functions with Original Signatures ---

/// Reads an encrypted file (assumed to have no extension), decrypts its content,
/// and returns the data along with the filename stem.
/// **If file reading fails, prints error to stderr and returns `(Vec::new(), String::new())`.**
/// **The caller is responsible for determining and appending the correct original extension to the returned stem.**
pub fn decrypt_file(file_path: &str) -> (Vec<u8>, String) {
    // Read the encrypted file, handle error internally
    let data = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[Error] Failed to read file for decryption '{}': {}", file_path, e);
            return (Vec::new(), String::new()); // Return empty tuple on error
        }
    };

    // Decrypt the data using the file path for key derivation
    let decrypted_data = decrypt(&data, file_path);

    // Extract the file stem from the input path
    let file_stem = Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    if file_stem.is_empty() {
        // Handle cases like trying to decrypt a file named just ".something"
        eprintln!("[Warning] Could not extract a valid file stem from '{}' for decryption output.", file_path);
        // Return decrypted data but maybe a placeholder name
        (decrypted_data, "decrypted_noname".to_string())
    } else {
        // Return the data and the stem. Caller adds the extension.
        (decrypted_data, file_stem.to_string())
    }
}


/// Reads a file, encrypts its content using the new logic, and returns the
/// encrypted data along with the new filename (original stem only, no extension).
/// **If file reading fails, prints error to stderr and returns `(Vec::new(), String::new())`.**
pub fn encrypt_file(file_path: &str, advanced_positions: bool) -> (Vec<u8>, String) {
    // Read the original file, handle error internally
    let data = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[Error] Failed to read file for encryption '{}': {}", file_path, e);
            return (Vec::new(), String::new()); // Return empty tuple on error
        }
    };

    // Encrypt the data using the file path for key derivation
    let encrypted_data = encrypt(&data, file_path, advanced_positions);

    // Generate the output filename (stem only)
    let file_stem = Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let output_filename = if file_stem.is_empty() {
         eprintln!("[Warning] Could not extract a valid file stem from '{}' for encryption output.", file_path);
         "encrypted_noname".to_string() // Return placeholder name
    } else {
        file_stem.to_string() // Just the stem
    };

    (encrypted_data, output_filename)
}