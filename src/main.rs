use std::fs;

use config::Config;
mod config;
mod convert_img;

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
    println!("v0.1.3");

    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let src = fs::canonicalize(config.src).expect("Unable to read source directory");
    let dest = fs::canonicalize(config.dest).expect("Unable to read destination directory");

    // Create the destination directory
    if let Err(err) = fs::create_dir_all(&dest) {
        eprintln!("Unable to create destination folder: {err}");
    }

    // Read Source directory
    convert_img::convert(&src, &dest).unwrap_or_else(|err| eprintln!("Unable to Convert: {err}"));
}
