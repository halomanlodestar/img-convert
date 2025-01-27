use anyhow::anyhow;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum ImageFormats {
    WebP,
    Avif,
}

impl FromStr for ImageFormats {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "webp" => Ok(ImageFormats::WebP),
            "avif" => Ok(ImageFormats::Avif),
            _ => Err(anyhow!("Invalid Format")),
        }
    }
}

impl ToString for ImageFormats {
    fn to_string(&self) -> String {
        return match self {
            ImageFormats::WebP => String::from("webp"),
            ImageFormats::Avif => String::from("avif"),
        };
    }
}
