// imports
use image::{DynamicImage, RgbaImage};
use flate2::{write::ZlibEncoder, Compression};
use rand::Rng;
use std::io::Write;

// constant properties per spec
const TILE_SIZE: u32 = 16;
const PIXELS: usize = (TILE_SIZE * TILE_SIZE) as usize;
const MASK_SIZE: usize = 32;

// create a oneloaderimagedelta patch between two images
pub fn compute_diff(source_img: &DynamicImage, target_img: &DynamicImage) -> Vec<u8> {
    // convert images to rgba8
    let source_rgba = source_img.to_rgba8();
    let target_rgba = target_img.to_rgba8();
    // get the number of tiles in the x and y direction
    let tiles_x = (target_img.width() + TILE_SIZE - 1) / TILE_SIZE;
    let tiles_y = (target_img.height() + TILE_SIZE - 1) / TILE_SIZE;
    // go through each tile
    let mut segments = Vec::new();
    for x in 0..tiles_x {
        for y in 0..tiles_y {
            // extract the tile from the source and target images
            let source_tile = extract_tile(&source_rgba, x, y);
            let target_tile = extract_tile(&target_rgba, x, y);
            // compare... essentially, in u8 32 = no changes, so skip (PIXELS / 8 = 32)
            let diff = compute_diff_tile(source_tile, target_tile);
            if diff.len() > PIXELS / 8 {
                // tile stream is: x, y, diff length, diff
                let mut tile_stream = Vec::new();
                tile_stream.extend_from_slice(&(x as u16).to_be_bytes());
                tile_stream.extend_from_slice(&(y as u16).to_be_bytes());
                tile_stream.extend_from_slice(&(diff.len() as u32).to_be_bytes());
                tile_stream.extend(diff);
                segments.push(tile_stream);
            }
        }
    }
    // get uncompressed data ready
    let total_len: usize = segments.iter().map(|seg| seg.len()).sum();
    let mut uncompressed = Vec::with_capacity(total_len);
    for segment in segments {
        uncompressed.extend(segment);
    }
    // compress the data
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&uncompressed).unwrap();
    let compressed = encoder.finish().unwrap();
    // create the output following olid spec..
    let mut output = Vec::new();
    output.extend_from_slice(&[0xFE, 0xFF, 0xD8, 0x08, 0xDD, 0x21]); // header
    output.extend_from_slice(&(target_img.width() as u32).to_be_bytes()); // width
    output.extend_from_slice(&(target_img.height() as u32).to_be_bytes()); // height
    let mut rng = rand::thread_rng(); 
    let unique_id: [u8; 8] = rng.gen();
    output.extend_from_slice(&unique_id); // unique id
    output.extend_from_slice(&(compressed.len() as u32).to_be_bytes()); // compressed length
    output.extend(compressed); // compressed data
    // return the output
    output
}

// get a tile from an image given pos
fn extract_tile(image: &RgbaImage, tile_x: u32, tile_y: u32) -> Vec<u32> {
    // go through each pixel in the tile
    let mut tile = vec![0u32; PIXELS];
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let img_x = tile_x * TILE_SIZE + x;
            let img_y = tile_y * TILE_SIZE + y;
            // get the pixel, or 0 if out of bounds
            let pixel = if img_x < image.width() && img_y < image.height() {
                image.get_pixel(img_x, img_y)
            } else {
                &image::Rgba([0, 0, 0, 0])
            };
            // set the pixel in the tile
            let index = (y * TILE_SIZE + x) as usize;
            tile[index] = ((pixel[0] as u32) << 24)
                | ((pixel[1] as u32) << 16) // 
                | ((pixel[2] as u32) << 8)
                | (pixel[3] as u32);
        }
    }
    // return the tile
    tile
}

// compute the diff between two tiles
pub fn compute_diff_tile(source: Vec<u32>, target: Vec<u32>) -> Vec<u8> {
    // should not get here.. please..
    if source.len() != PIXELS || target.len() != PIXELS {
        panic!("Source and target must be 1024 bytes long");
    }
    // create the bitmap and values
    let mut bitmap = vec![0u8; MASK_SIZE];
    let mut values: Vec<u32> = Vec::new();
    // go through each pixel
    for i in 0..PIXELS {
        if source[i] != target[i] {
            values.push(target[i]);
            bitmap[i / 8] |= 1 << (i % 8); 
        }
    }
    // note: omori olid = big endian, but tcoaal olid = little endian
    let mut output = Vec::from(bitmap); 
    for value in values {
        output.extend_from_slice(&value.to_le_bytes()); 
    }
    // return the difference (empty = all 0 with size 32)
    output
}