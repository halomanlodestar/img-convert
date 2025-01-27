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
        .with_speed(10);
    let raw_image = img.to_rgba8().into_raw();
    let rgba_img = raw_image.as_slice().as_rgba();
    let data = Img::new(rgba_img, width as usize, height as usize);

    return encoder.encode_rgba(data);
}
