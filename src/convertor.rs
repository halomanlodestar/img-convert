use image::{DynamicImage, GenericImageView};
use webp::{Encoder, WebPMemory};
use anyhow::Result;

pub fn to_webp(img: DynamicImage, quality: u8) -> Result<WebPMemory> {
    let (width, height) = img.dimensions();
    let binding = img.to_rgba8();
    let encoder = Encoder::from_rgba(binding.as_raw(), width, height);
    let webp_data = encoder.encode(quality as f32);

    return Ok(webp_data);
}