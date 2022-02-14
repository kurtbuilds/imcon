use std::io::Cursor;
use ::image as image_rs;
use image::DynamicImage;
use crate::image::DataSource;
use anyhow::Result;


pub fn load_image(source: DataSource, format: image_rs::ImageFormat) -> Result<DynamicImage> {
    match source {
        DataSource::File(path) => image_rs::open(path),
        DataSource::Memory(reader) => {
            image_rs::io::Reader::new(
                Cursor::new(reader)
            )
                .with_format(format)
                .and_then(|image| image.decode())
        }
    }
        .map_err(|e| anyhow::anyhow!("{}", e))
}