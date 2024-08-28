// imports
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// the file extension is stored in the header of the file..
pub fn file_extension(data: &[u8]) -> String {
    // sanity check, return k9a
    if data.is_empty() {
        return String::from("k9a");
    }
    // sanity check two, also return k9a
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

// use file name to generate a mask with how the bytes are scrambled
pub fn make_mask(input_string: &str) -> i32 {
    // ccreate empty mask + extract filename in uppercase
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
pub fn decrypt_file(data: &[u8], file_path: &str) -> Vec<u8> {
    // ensure proper file extension
    if !file_path.ends_with(".k9a") {
        println!("Input file is not a .k9a file, returning empty vector.");
        return vec![0];
    }
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