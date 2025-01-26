use anyhow::{Context, Error, Result};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    fs::{self},
    io::{stdout, Write},
    path::{Path, PathBuf},
};

use crate::{
    convertor::to_webp,
    utils::{read_file, write_file},
};

pub fn convert(
    src: &PathBuf,
    dest: &PathBuf,
    converted: &mut usize,
    failed: &mut HashMap<OsString, Error>,
    skipped: &mut usize,
    total_items_count: usize,
) -> Result<()> {
    let dir = fs::read_dir(&src).map_err(|e| e)?;

    dir.into_iter().try_for_each::<_, Result<()>>(|entry| {
        let entry = entry?;

        if entry.metadata().is_ok_and(|meta| meta.is_dir()) {
            convert(
                &(entry.path()),
                &dest.join(entry.file_name()),
                converted,
                failed,
                skipped,
                total_items_count,
            )?;
        } else {
            let path = entry.path();
            let ext = path.extension().and_then(OsStr::to_str);

            let relative_path = path.strip_prefix(
                path.parent()
                    .with_context(|| format!("Parent directory not found"))?,
            )?;

            let output_path = dest.join(relative_path);

            // println!("{output_path:?}");

            if ext.is_some_and(|ext| ext.eq("webp")) {
                *skipped += 1;
                let file = read_file(path.as_path().as_ref())?;
                write_file(&output_path, file.into_bytes())?;
            } else if let Err(err) = convert_to_webp(&path, &output_path, 80) {
                failed.insert(entry.file_name(), err);
            } else {
                *converted += 1;
            };

            let total_processed = *converted + *skipped + failed.len();
            print!(
                "\rConverted {}, Failed {}, Skipped {}, ({}/{})",
                converted,
                failed.len(),
                skipped,
                total_processed,
                total_items_count
            );
            stdout().flush()?;
        }

        return Ok(());
    })?;

    return Ok(());
}

fn convert_to_webp(src: &Path, dest: &Path, quality: u8) -> Result<()> {
    let img = read_file(src)?;
    let webp_data = to_webp(img, quality)?;
    write_file(&dest.with_extension("webp"), webp_data.as_ref())?;
    Ok(())
}
