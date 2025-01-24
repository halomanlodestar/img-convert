use image::GenericImageView;
use image::ImageReader;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use webp::Encoder;

pub fn count_items(src: &PathBuf) -> Result<usize, Box<dyn Error>> {
    let dir = fs::read_dir(&src).map_err(|e| e)?;

    let mut count: usize = 0;

    dir.into_iter().map(|entry| entry.unwrap()).
    for_each(|entry| {
        if entry.metadata().unwrap().is_dir() {
            count += count_items(&entry.path()).unwrap();
        } else {
            count += 1;
        }
    });

    return Ok(count);
}

pub fn convert(src: &PathBuf, dest: &PathBuf, converted: &mut usize, total_items_count: usize) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(&src).map_err(|e| e)?;

    dir.into_iter()
        .map(|entry| entry.unwrap())
        .for_each(|entry| {
            if entry.metadata().unwrap().is_dir() {
                println!("Crawling in dir: {:?}", entry.file_name());
                convert(&entry.path(), &dest.join(entry.file_name()), converted, total_items_count).ok();
            } else {
                // println!("\tConverting file: {:?} {:?}", entry.path(), &dest)
                // Convert If Image files else recurse deeper
                if let Err(err) = convert_to_webp(&(entry.path()), dest, 80) {
                    println!("Error Occured While converting: {:?} {err}", entry.file_name());
                } else {
                    *converted += 1;
                    println!("Converted {:?}/{:?} items", converted, total_items_count)
                };
            }
        });

    return Ok(());
}

fn convert_to_webp(src: &Path, dest: &Path, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(src)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    // Encode the image as WebP
    let (width, height) = img.dimensions();
    let binding = img.to_rgba8();
    let encoder = Encoder::from_rgba(binding.as_raw(), width, height);
    let webp_data = encoder.encode(quality as f32);

    // Build the output file path
    let relative_path = src.strip_prefix(src.parent().unwrap()).unwrap();
    let output_path = dest.join(relative_path).with_extension("webp");

    // Create output subdirectories if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    // Write the encoded data to the output file
    fs::write(output_path, webp_data.as_ref()).map_err(|e| e.to_string())?;
    // println!("Converted: {}", src.display());
    Ok(())
}
