use std::path::PathBuf;
use image::{DynamicImage, ImageBuffer};
use libheif_rs::{Channel, ColorSpace, HeifContext, RgbChroma};
use anyhow::Result;
use crate::transform::Resize;


pub fn create_image(ctx: HeifContext) -> Result<DynamicImage> {
    let handle = ctx.primary_image_handle()?;
    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), false)?;
    let width = image.width(Channel::Interleaved).map_err(|e| anyhow::anyhow!("{}", e))?;
    let height = image.height(Channel::Interleaved).map_err(|e| anyhow::anyhow!("{}", e))?;
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.unwrap();
    ImageBuffer::from_raw(width, height, interleaved_plane.data.to_owned())
        .map(DynamicImage::ImageRgb8)
        .ok_or(anyhow::anyhow!("Failed to create image buffer"))
}

pub fn open_image(path: &PathBuf, _resize: Option<Resize>) -> Result<DynamicImage> {
    let im = HeifContext::read_from_file(path.to_string_lossy().as_ref())?;
    create_image(im)
}


pub fn read_image(data: &[u8], _resize: Option<Resize>) -> Result<DynamicImage> {
    let ctx = HeifContext::read_from_bytes(data)?;
    create_image(ctx)
}