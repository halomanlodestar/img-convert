use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use image::{DynamicImage, ImageReader};

pub fn read_file(src: &Path) -> Result<DynamicImage> {
    
    let img = ImageReader::open(src)
    .with_context(|| format!("Failed to open image: {:?}", src))?
    .decode()
    // .map_err(|e| {eprintln!("{e}:?"); e})
    .with_context(|| format!("Failed to decode image: {:?}", src))?;

    return Ok(img);
}

pub fn write_file<T: AsRef<[u8]>>(dest: &Path, data: T) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }

    fs::write(dest, data.as_ref())
        .with_context(|| format!("Failed to write WebP data to: {:?}", dest))?;

    return Ok(());
}

pub fn count_items(src: &PathBuf) -> Result<usize> {
    let dir = fs::read_dir(&src).map_err(|e| e)?;

    let mut count: usize = 0;

    dir.into_iter().try_for_each::<_, Result<()>>(|entry| {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            count += count_items(&entry.path())?;
        } else {
            count += 1;
        }
        return Ok(());
    })?;

    return Ok(count);
}
