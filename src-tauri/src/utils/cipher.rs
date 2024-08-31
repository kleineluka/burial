// imports
use std::path::Path;
use crate::utils::files;

// the file extension is stored in the header of the file..
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

// create header based on file extension
pub fn create_header(extension: &str) -> Vec<u8> {
    let mut header = Vec::new();
    header.push(extension.len() as u8);
    header.extend_from_slice(extension.as_bytes());
    header
}

// use file name to generate a mask with how the bytes are scrambled
pub fn make_mask(input_string: &str) -> i32 {
    // create empty mask + extract filename in uppercase
    let mut mask_value = 0;
    let decoded_filename = Path::new(input_string)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_uppercase();
    // for each value in the name, shift the bit left 1 and xor with the ascii
    for c in decoded_filename.chars() {
        mask_value = (mask_value << 1) ^ c as i32;
    }
    // return the mask
    mask_value
}

// take a .k9a file and unscramble it to the original file..
pub fn decrypt(data: &[u8], file_path: &str) -> Vec<u8> {
    // get the header length and data length
    let header_length = data[0] as usize;
    let data_length = data[1 + header_length] as usize;
    let mut encrypted_data = vec![0u8; data.len() - 2 - header_length];
    encrypted_data.copy_from_slice(&data[2 + header_length..]);
    // create a mask from the file path
    let mut new_mask = make_mask(file_path);
    // if the data length is 0, set it to the length of the encrypted data
    let data_length = if data_length == 0 {
        encrypted_data.len()
    } else {
        data_length
    };
    // create a new vector to store the decrypted data
    let mut decrypted_data = encrypted_data.clone();
    // for each byte in the encrypted data, xor with the mask and store in the decrypted data
    for i in 0..data_length {
        let encrypted_byte = encrypted_data[i];
        decrypted_data[i] = ((encrypted_byte as i32 ^ new_mask) % 256) as u8;
        new_mask = (new_mask << 1) ^ encrypted_byte as i32;
    }
    // return the decrypted data
    decrypted_data
}

// take a file, decrypt it and  the data and filename+extension
pub fn decrypt_file(file_path: &str) -> (Vec<u8>, String) {
    // read the file and decrypt it..
    let data = files::read_file(file_path);
    let decrypted_data = decrypt(&data, file_path);
    // get the file name extension
    let file_name = files::file_name(file_path);
    let file_extension = parse_header(&data);
    // return the decrypted data and the file name + extension
    (decrypted_data, format!("{}.{}", file_name, file_extension))
}

// take a file and make it a .k9a file..
pub fn encrypt(data: &[u8], file_path: &str, advanced_positions: bool) -> Vec<u8> {
    // extract the file extension
    let extension = Path::new(file_path)
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("");
    // create the header
    let mut encrypted_data = create_header(extension);
    // Add the data length (use 0 for files larger than 255 bytes)
    encrypted_data.push(if data.len() > 255 { 0 } else { data.len() as u8 });
    // create a mask from the file path
    let mut mask = make_mask(file_path);
    // encrypt the data
    for &byte in data {
        let encrypted_byte = ((byte as i32 ^ mask) % 256) as u8;
        encrypted_data.push(encrypted_byte);
        mask = (mask << 1) ^ encrypted_byte as i32;
    }
    encrypted_data
}

// take a file, encrypt it and return the data and the new file name + extension
pub fn encrypt_file(file_path: &str, advanced_positions: bool) -> (Vec<u8>, String) {
    // read the file and encrypt it..
    let data = files::read_file(file_path);
    let encrypted_data = encrypt(&data, file_path, advanced_positions);
    // get the file name and extension
    let file_name = files::file_name(file_path);
    let file_extension = "k9a";
    // return the encrypted data and the new file name + extension
    (encrypted_data, format!("{}.{}", file_name, file_extension))
}