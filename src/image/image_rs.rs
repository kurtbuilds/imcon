use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use ::image as image_rs;
use image::{DynamicImage, ImageFormat};
use crate::image::DataSource;
use anyhow::Result;
use crate::Format;


impl TryInto<image_rs::ImageFormat> for Format {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<ImageFormat, Self::Error> {
        Ok(match self {
            Format::Png => image_rs::ImageFormat::Png,
            Format::Jpeg => image_rs::ImageFormat::Jpeg,
            Format::Bmp => image_rs::ImageFormat::Bmp,
            _ => return Err(anyhow::anyhow!("Format unsupported by image-rs library.")),
        })
    }
}

pub fn open_image(path: &PathBuf, format: Format) -> Result<DynamicImage> {
    let format = format.try_into()?;
    let f = File::open(path)?;
    let f = BufReader::new(f);
    ::image::load(f, format)
        .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn read_image(data: Vec<u8>, format: Format) -> Result<DynamicImage> {
    let format = format.try_into()?;
    let f = Cursor::new(data);
    ::image::load(f, format)
        .map_err(|e| anyhow::anyhow!("{}", e))
}