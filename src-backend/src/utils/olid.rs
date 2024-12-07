// imports
use std::error::Error;
use flate2::{write::DeflateEncoder, Compression};
use image::{DynamicImage, GenericImageView, Pixel};
use rand::{thread_rng, Rng};
use std::io::Write;

// https://github.com/rphsoftware/OneLoaderImageDelta
// https://codeberg.org/basil/OLID.ts/src/branch/main/src/lib/lib.ts
// A OneLoader Image Delta (Rust/JS), ported to TypeScript for Tomb, then back to Rust (but in a way that's easier for me to use)
const TILE_SIZE: u32 = 32;
const PIXELS: usize = (TILE_SIZE * TILE_SIZE) as usize;
const MASK_SIZE: usize = PIXELS / 8;

// compute the difference between two images
pub fn compute_diff(source_img: &DynamicImage, target_img: &DynamicImage) -> Result<Vec<u8>, Box<dyn Error>> {
    // i already know people will run into this issue..
    if source_img.dimensions() != target_img.dimensions() {
        return Err("Source and target images must have the same dimensions".into());
    }
    let (width, height) = target_img.dimensions();
    let mut segments: Vec<Vec<u8>> = Vec::new();
    // iterate over the tiles in the images
    for x in 0..(width + TILE_SIZE - 1) / TILE_SIZE {
        for y in 0..(height + TILE_SIZE - 1) / TILE_SIZE {
            let source_tile = get_tile(source_img, x, y);
            let target_tile = get_tile(target_img, x, y);
            // compute the difference between the tiles
            let diff = compute_diff_tile(&source_tile, &target_tile)?;
            if diff.len() > (TILE_SIZE * TILE_SIZE / 8) as usize {
                let mut target_data = Vec::new();
                target_data.extend_from_slice(&(x as u16).to_le_bytes());
                target_data.extend_from_slice(&(y as u16).to_le_bytes());
                target_data.extend_from_slice(&(diff.len() as u32).to_le_bytes());
                target_data.extend_from_slice(&diff);
                segments.push(target_data);
            }
        }
    }
    // compress the segments
    let total_len: usize = segments.iter().map(|seg| seg.len()).sum();
    let mut uncompressed = Vec::with_capacity(total_len);
    for block in segments {
        uncompressed.extend(block);
    }
    // compress the data
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&uncompressed)?;
    let compressed = encoder.finish()?;
    // create the output buffer
    let mut output = Vec::new();
    output.extend_from_slice(&0xFEFFD808u32.to_le_bytes());
    output.extend_from_slice(&0xDD21u16.to_le_bytes());
    output.extend_from_slice(&width.to_le_bytes());
    output.extend_from_slice(&height.to_le_bytes());
    // add a random 8-byte value
    let mut rng = thread_rng();
    let random_bytes: [u8; 8] = rng.gen();
    output.extend_from_slice(&random_bytes);
    // add the compressed data
    output.extend_from_slice(&(compressed.len() as u32).to_le_bytes());
    output.extend_from_slice(&compressed);
    Ok(output)
}

// compute the difference between two tiles
pub fn compute_diff_tile(source: &[u8], target: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // okay, this one we shouldn't run into..
    if source.len() != PIXELS * 4 || target.len() != PIXELS * 4 {
        return Err("Source and target must be exactly the size of a tile".into());
    }
    let mut bitmap = vec![0u8; MASK_SIZE];
    let mut values = Vec::new();
    // iterate over the pixels in the tiles
    for i in 0..PIXELS {
        if &source[i * 4..i * 4 + 4] != &target[i * 4..i * 4 + 4] {
            values.extend_from_slice(&target[i * 4..i * 4 + 4]);
            bitmap[i / 8] |= 1 << (i % 8);
        }
    }
    // create the output buffer
    let mut output = Vec::new();
    output.extend_from_slice(&bitmap);
    output.extend(values);
    Ok(output)
}

// get a tile from an image
fn get_tile(image: &DynamicImage, x: u32, y: u32) -> Vec<u8> {
    // create a buffer for the tile
    let mut tile = vec![0u8; (TILE_SIZE * TILE_SIZE * 4) as usize];
    let (width, height) = image.dimensions();
    let px = x * TILE_SIZE;
    let py = y * TILE_SIZE;
    // copy the pixels from the image to the tile
    for ty in 0..TILE_SIZE {
        for tx in 0..TILE_SIZE {
            let ix = px + tx;
            let iy = py + ty;
            // if the pixel is within the bounds of the image
            if ix < width && iy < height {
                let pixel = image.get_pixel(ix, iy).to_rgba();
                let offset = ((ty * TILE_SIZE + tx) * 4) as usize;
                tile[offset..offset + 4].copy_from_slice(&pixel.0);
            }
        }
    }
    tile
}
