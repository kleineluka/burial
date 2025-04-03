use std::fs;
use std::path::Path;
use tree_magic;

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

// generates a value based on the file stem (name without extension) in uppercase.
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

pub fn encrypt(data: &[u8], file_path: &str, advanced_positions: bool) -> Vec<u8> {
    let data_len = data.len();
    let (num_bytes_to_encrypt, indicator) = if advanced_positions {
        let extension = Path::new(file_path)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("");
        if data_len >= data_len || data_len > 255 {
             (data_len, 0u8) // Encrypt all, indicator 0
        } else {
            (data_len, data_len as u8) 
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

pub fn decrypt_file(file_path: &str) -> (Vec<u8>, String) {
    let data = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[Error] Failed to read file for decryption '{}': {}", file_path, e);
            return (Vec::new(), String::new()); 
        }
    };
    let decrypted_data = decrypt(&data, file_path);
    let file_stem = Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    if file_stem.is_empty() {
        eprintln!("[Warning] Could not extract a valid file stem from '{}' for decryption output.", file_path);
        (decrypted_data, "decrypted_noname".to_string())
    } else {
        (decrypted_data, file_stem.to_string())
    }
}

pub fn encrypt_file(file_path: &str, advanced_positions: bool) -> (Vec<u8>, String) {
    let data = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[Error] Failed to read file for encryption '{}': {}", file_path, e);
            return (Vec::new(), String::new()); 
        }
    };
    let encrypted_data = encrypt(&data, file_path, advanced_positions);
    let file_stem = Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let output_filename = if file_stem.is_empty() {
         eprintln!("[Warning] Could not extract a valid file stem from '{}' for encryption output.", file_path);
         "encrypted_noname".to_string() 
    } else {
        file_stem.to_string() 
    };
    (encrypted_data, output_filename)
}