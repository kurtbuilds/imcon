use std::path::PathBuf;
use image::{DynamicImage, ImageBuffer};
use libheif_rs::{Channel, ColorSpace, HeifContext, HeifError, RgbChroma};
use crate::image::DataSource;
use anyhow::Result;
use crate::transform::Resize;


pub fn create_heif_image(source: DataSource) -> Result<libheif_rs::Image, HeifError> {
    let ctx = match source {
        DataSource::File(path) => {
            HeifContext::read_from_file(path.to_string_lossy().as_ref())?
        }
        DataSource::Memory(reader) => {
            HeifContext::read_from_memory(reader)?
        }
    };
    let handle = ctx.primary_image_handle()?;
    handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), false)
}


pub fn load_image(source: DataSource, resize: Option<Resize>) -> Result<DynamicImage> {
    let image = create_heif_image(source)?;
    let width = image.width(Channel::Interleaved).map_err(|e| anyhow::anyhow!("{}", e))?;
    let height = image.height(Channel::Interleaved).map_err(|e| anyhow::anyhow!("{}", e))?;
    ImageBuffer::from_raw(width, height, interleaved_plane.data.to_owned())
        .map(DynamicImage::ImageBgra8)
        .ok_or(anyhow::anyhow!("Failed to create image buffer"))
}