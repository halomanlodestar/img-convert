use crate::img_formats::ImageFormats;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    src: String,

    #[arg(short, long)]
    dest: String,

    #[arg(default_value_t=75)]
    quality: u8,

    #[arg(default_value_t=ImageFormats::Webp)]
    format: ImageFormats,

}

pub struct Config {
    pub src: String,
    pub dest: String,
    pub quality: u8,
    pub format: ImageFormats,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();

        let format = match args
            .format
            .to_string()
            .as_str()
            .trim()
            .to_ascii_lowercase()
            .as_str()
        {
            "avif" => ImageFormats::Avif,
            "webp" => ImageFormats::Webp,
            _ => panic!("Unsupported format"),
        };

        let quality = if args.quality > 100 {panic!("Quality must be between 0-100")} else {args.quality};

        return Self {
            src: args.src,
            dest: args.dest,
            quality,
            format,
        };
    }
}
