mod config;
mod convert_img;

use anyhow::Error;
use config::Config;
use convert_img::count_items;
use std::{collections::HashMap, ffi::OsString, fs};

// Steps
// 1. Parse the command-line arguments
// 2. Create the destination directory (if it doesn't exist)
// 3. Read the source directory
// 4. Convert the images to WebP
// 5. Write the converted images to the destination directory
// 6. Print the path of the converted images
// 7. Handle errors
// 8. Exit the program

fn main() {
    let config = Config::new();

    let src = fs::canonicalize(config.src).expect("Unable to read source directory");
    let dest = fs::canonicalize(config.dest).expect("Unable to read destination directory");

    // Create the destination directory
    if let Err(err) = fs::create_dir_all(&dest) {
        eprintln!("Unable to create destination folder: {err}");
    }

    // Read Source directory
    let total_items = count_items(&src);

    if let Ok(count) = total_items {
        println!("Converting {count} items");

        let mut converted: usize = 0;
        let mut failed: HashMap<OsString, Error> = HashMap::new();

        convert_img::convert(&src, &dest, &mut converted, &mut failed, count)
            .unwrap_or_else(|err| eprintln!("Unable to Convert: {err}"));

        println!("\nSuccessfully Converted {}/{} images ✅", converted, count);
        println!("Failed {}", failed.keys().len());

        failed.into_iter().for_each(|item| {
            println!("{:#}", item.1);
        });
    }
}
