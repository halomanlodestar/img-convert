use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use ravif::{EncodedImage, Encoder as AvifEncoder, Img};
use rgb::FromSlice;
use webp::{Encoder as WebpEncoder, WebPMemory};

pub fn to_webp(img: DynamicImage, quality: u8) -> Result<WebPMemory> {
    let (width, height) = img.dimensions();
    let binding = img.to_rgba8();
    let encoder = WebpEncoder::from_rgba(binding.as_raw(), width, height);
    let webp_data = encoder.encode(quality as f32);

    return Ok(webp_data);
}

pub fn to_avif(img: DynamicImage, quality: u8) -> Result<EncodedImage, ravif::Error> {
    let (width, height) = img.dimensions();
    let encoder = AvifEncoder::new()
        .with_quality(quality as f32)
        .with_speed(1);
    let data = img.as_bytes().as_rgba();

    return encoder.encode_rgba(Img::new(data, width as usize, height as usize));
}
