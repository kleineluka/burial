use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom, Write};
use std::str::FromStr;

// patch files to make a new file 
// why not just supply the new file? 1) can be used to make dynamic content, 2) doesn't redistribute game files
pub fn patch_file(file_path: &str, diff_list: &str) -> io::Result<()> {
    // open file, make sure read/write true (hence why not use my files class..)
    let mut file_to_edit = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)?;
    // iterate over each diff entry in the byte list
    for diff in diff_list.split(',') {
        // split each entry into position and byte value
        let parts: Vec<&str> = diff.split(':').collect();
        if parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid diff format"));
        }
        // parse position and byte value
        let pos = u64::from_str(parts[0]).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let byte_value = u8::from_str_radix(parts[1], 16)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?; 
        // seek to the position and write the new byte
        file_to_edit.seek(SeekFrom::Start(pos))?;
        file_to_edit.write_all(&[byte_value])?;
    }
    Ok(())
}

// patch bytes to data (not from a file like above)
pub fn patch_data(mut data: Vec<u8>, diff_list: &str) -> Vec<u8> {
    // iterate over each diff entry in the diff list
    for diff in diff_list.split(',') {
        // split each entry into position and byte value
        let parts: Vec<&str> = diff.split(':').collect();
        if parts.len() != 2 {
            continue; 
        }
        // parse position and byte value
        let pos = match u64::from_str(parts[0]) {
            Ok(p) => p as usize,
            Err(_) => continue, 
        };
        let byte_value = match u8::from_str_radix(parts[1], 16) {
            Ok(bv) => bv,
            Err(_) => continue, 
        };
        // ensure the vector has enough capacity to accommodate the new byte position
        if pos >= data.len() {
            // resize the vector, filling new space with 0s
            data.resize(pos + 1, 0); 
        }
        // apply the patch by setting the byte at the specified position
        data[pos] = byte_value;
    }
    // return the modified data
    data
}
