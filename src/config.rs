use crate::img_formats::ImageFormats;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    src: String,

    #[arg(short, long)]
    dest: String,

    #[arg(default_value_t=ImageFormats::WebP)]
    format: ImageFormats,
}

pub struct Config {
    pub src: String,
    pub dest: String,
    pub format: ImageFormats,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();

        println!("{args:?}");
        let format = match args
            .format
            .to_string()
            .as_str()
            .trim()
            .to_ascii_lowercase()
            .as_str()
        {
            "avif" => ImageFormats::Avif,
            "webp" => ImageFormats::WebP,
            _ => panic!("Unsupported format"),
        };

        return Self {
            src: args.src,
            dest: args.dest,
            format,
        };
    }
}
