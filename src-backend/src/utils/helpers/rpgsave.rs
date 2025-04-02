use lz_str::compress_to_base64;
use lz_str::decompress_from_base64;
use jsonxf::pretty_print;
use jsonxf::minimize;
use std::error::Error;

/// Encodes a string into Base64 LZString compressed data.
pub fn encode(data: &str) -> Result<String, Box<dyn Error>> {
    let decompressed_minified = minimize(data)?;
    let compressed = compress_to_base64(&decompressed_minified);
    Ok(compressed)
}

/// decodes base64 lzstring compressed data
pub fn decode(data: &str) -> Result<String, Box<dyn Error>> {
    let decompressed = decompress_from_base64(&data).unwrap();
    let decompressed_string = String::from_utf16(&decompressed).unwrap();
    let decompressed_pretty = pretty_print(&decompressed_string).unwrap();
    Ok(decompressed_pretty)
}
