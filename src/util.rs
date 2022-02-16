use std::path::PathBuf;
use ::image::ImageBuffer;
use imcon::{DataSource, Format, Image};
use anyhow::Result;
use image::{DynamicImage, Pixel, Rgb, Rgba, RgbaImage, RgbImage};

/// Takes a string representing a hex_color.
pub fn resolve_hex_color(mut hex_color: &str) -> anyhow::Result<Vec<u8>> {
    if hex_color.starts_with("#") {
        hex_color = &hex_color[1..];
    }
    let mut bytes = Vec::new();
    let size = hex_color.len();
    if size == 3 || size == 4 {
        for i in 0..size {
            let byte = u8::from_str_radix(&hex_color[i..i+1], 16)? * 0x11;
            bytes.push(byte);
        }
    } else if size == 6 || size == 8 {
        for i in 0..(size/2) {
            let byte = u8::from_str_radix(&hex_color[(i*2)..(i*2+2)], 16)?;
            bytes.push(byte);
        }
    } else {
        return Err(anyhow::anyhow!("Invalid hex color code."));
    }
    Ok(bytes)
}

pub fn resolve_image(input: &str, input_format: Format) -> Result<Image> {
    if input.starts_with('#') {
        let input = resolve_hex_color(input)?;
        let image = if input.len() == 3 {
            DynamicImage::ImageRgb8(RgbImage::from_pixel(512, 512, *Rgb::from_slice(&input)))
        } else if input.len() == 4 {
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(512, 512, *Rgba::from_slice(&input)))
        } else {
            return Err(anyhow::anyhow!("Invalid hex color code."));
        };
        let im = image.to_rgba8().to_vec();
        // TODO: This isn't right... Need to figure out a better way to represent these.
        return Ok(Image::new(DataSource::Memory(im, input_format)));
    }
    Ok(Image::new(DataSource::File(PathBuf::from(input), input_format)))
}