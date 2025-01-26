mod config;
mod convert_img;
mod utils;
mod convertor;

use anyhow::Error;
use config::Config;
use utils::count_items;
use std::{collections::HashMap, ffi::OsString, fs};

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
        let mut skipped: usize = 0;

        convert_img::convert(
            &src,
            &dest,
            &mut converted,
            &mut failed,
            &mut skipped,
            count,
        )
        .unwrap_or_else(|err| eprintln!("Unable to Convert: {err}"));

        println!("\nSuccessfully Converted {}/{} images ✅", converted, count);
        println!("Failed {}", failed.keys().len());

        failed.into_iter().for_each(|item| {
            println!("{:#}", item.1);
        });
    }
}
