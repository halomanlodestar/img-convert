use image::{GenericImageView, ImageReader};
use std::{
    fs,
    ffi::OsString,
    io::{stdout, Write},
    collections::HashMap,
    path::{Path, PathBuf}
};
use webp::Encoder;
use anyhow::{Context, Error, Result};

pub fn count_items(src: &PathBuf) -> Result<usize> {
    let dir = fs::read_dir(&src).map_err(|e| e)?;

    let mut count: usize = 0;

    dir.into_iter()
        .map(|entry| entry.unwrap())
        .for_each(|entry| {
            if entry.metadata().unwrap().is_dir() {
                count += count_items(&entry.path()).unwrap();
            } else {
                count += 1;
            }
        });

    return Ok(count);
}

pub fn convert(
    src: &PathBuf,
    dest: &PathBuf,
    converted: &mut usize,
    failed: &mut HashMap<OsString, Error>,
    total_items_count: usize,
) -> Result<()> {
    let dir = fs::read_dir(&src).map_err(|e| e)?;

    dir.into_iter()
        .map(|entry| entry.unwrap())
        .for_each(|entry| {
            if entry.metadata().is_ok_and(|meta| meta.is_dir()) {
                // println!("Crawling in dir: {:?}", entry.file_name());
                convert(
                    &entry.path(),
                    &dest.join(entry.file_name()),
                    converted,
                    failed,
                    total_items_count,
                )
                .ok();
            } else {
                // println!("\tConverting file: {:?} {:?}", entry.path(), &dest)
                // Convert If Image files else recurse deeper
                if let Err(err) = convert_to_webp(&(entry.path()), dest, 80) {
                    // println!(
                    //     "Error Occured While converting: {:?} {err}",
                    //     entry.file_name()
                    // );
                    failed.insert(entry.file_name(), err);
                } else {
                    *converted += 1;
                    print!("\rConverted {}/{} items", converted, total_items_count);
                    stdout().flush().ok();
                };
            }
        });

    return Ok(());
}

fn convert_to_webp(src: &Path, dest: &Path, quality: u8) -> Result<()> {
    let img = ImageReader::open(src)
        .with_context(|| format!("Failed to open image: {:?}", src))?
        .decode()
        .with_context(|| format!("Failed to decode image: {:?}", src))?;

    // Encode the image as WebP
    let (width, height) = img.dimensions();
    let binding = img.to_rgba8();
    let encoder = Encoder::from_rgba(binding.as_raw(), width, height);
    let webp_data = encoder.encode(quality as f32);

    // Build the output file path
    let relative_path = src.strip_prefix(src.parent().with_context(|| format!("Parent directory not found"))?)?;
    let output_path = dest.join(relative_path).with_extension("webp");

    // Create output subdirectories if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }

    // Write the encoded data to the output file
    fs::write(output_path.clone(), webp_data.as_ref()).with_context(|| format!("Failed to write WebP data to: {:?}", output_path))?;
    // println!("Converted: {}", src.display());
    Ok(())
}
