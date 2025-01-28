mod config;
mod convert_img;
mod convertor;
mod img_formats;
mod utils;

use anyhow::Error;
use config::Config;
use std::{collections::HashMap, ffi::OsString, fs};
use utils::count_items;

fn main() {
    let config = Config::new();

    let src = fs::canonicalize(config.src).expect("Unable to read source directory");
    let dest = fs::canonicalize(config.dest).expect("Unable to read destination directory");

    if let Err(err) = fs::create_dir_all(&dest) {
        eprintln!("Unable to create destination folder: {err}");
    }

    let total_items = count_items(&src);

    if let Ok(count) = total_items {
        println!("Converting {count} items");

        let mut converted: usize = 0;
        let mut failed: HashMap<OsString, Error> = HashMap::new();
        let mut skipped: usize = 0;

        convert_img::convert(
            &src,
            &dest,
            config.quality,
            &config.format,
            &mut converted,
            &mut failed,
            &mut skipped,
            count,
        )
        .unwrap_or_else(|err| eprintln!("Unable to Convert: {err}"));

        println!("\nConverted {} ✅", converted);
        println!("Skipped {} 😴", skipped);
        println!("Failed {} ❌", failed.keys().len());

        failed.into_iter().for_each(|item| {
            println!("{:#}", item.1);
        });
    }
}
