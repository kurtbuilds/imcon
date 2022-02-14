use std::io;
use std::io::Cursor;
use std::iter::once;
use std::path::PathBuf;
use crate::transform::{Resize, Transform};
use anyhow::Result;
use crate::image::pdf::{load_all_images, PDFIUM};
use image::GenericImageView;

mod pdf;
mod heif;
mod util;
mod image_rs;

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Pdf,
    Heif,
    Png,
    Jpeg,
}


pub struct Metadata {
    width: usize,
    height: usize,
}


pub enum DataSource {
    File(PathBuf),
    Memory(Box<dyn io::Read>),
}


pub struct Image {
    format: Format,
    source: DataSource,

    metadata: Option<Metadata>,

    // Operations
    resize: Option<Resize>,
    transforms: Vec<Transform>,
}


impl Image {
    fn new(format: Format, source: DataSource, transforms: Vec<Transform>) -> Self {
        Self {
            format,
            source,
            metadata: None,
            resize: None,
            transforms,
        }
    }
}


impl Image {
    pub fn open<S: Into<PathBuf>>(path: S) -> Self {
        let path = path.into();
        let ext = path.extension()
            .expect("No file extension or file extension unrecognized.")
            .to_string_lossy()
            .to_lowercase();
        let format = match ext.as_ref() {
            "png" => Format::Png,
            "jpeg" | "jpg" => Format::Jpeg,
            "pdf" => Format::Pdf,
            "heif" => Format::Heif,
            _ => panic!("File extension unrecognized."),
        };
        Self::new(format, DataSource::File(path), vec![])
    }

    pub fn read<S: io::Read>(reader: S, format: Format) -> Self {
        Self::new(format, DataSource::Memory(Box::new(reader)), vec![])
    }

    pub fn read_unknown_format<S: io::Read>(reader: S) -> Self {
        unimplemented!()
    }

    pub fn save(self, path: &str) -> Result<()> {
        let Image {source, resize, .. } = self;
        let mut image = match self.format {
            // TODO experiment with lib-specific resize instead of that from image-rthat from image-rs.
            Format::Pdf => pdf::load_image(source, 0, None),
            Format::Heif => heif::load_image(source, None),
            Format::Png => image_rs::load_image(source, ::image::ImageFormat::Png),
            Format::Jpeg => image_rs::load_image(source, ::image::ImageFormat::Jpeg),
        }?;
        if let Some(resize) = resize {
            let (width, height) = resize.calculate_dimensions(&image.width() as usize, &image.height() as usize);
            image = image.resize(width as u32, height as u32, image_rs::imageops::FilterType::Lanczos3);
        }
        image
            .to_rgba8()
            .save(path)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub fn save_all(self, path_template: &str) -> Result<()> {
        let Image {source, resize, .. } = self;
        let input_fpath = match &source {
            DataSource::File(path) => path.clone(),
            DataSource::Memory(_) => PathBuf::from("stdin"),
        };
        let iter = match self.format {
            Format::Pdf => pdf::load_all_images(source, None),
            Format::Heif => once(heif::load_image(source, None)),
            Format::Png => once(image_rs::load_image(source, ::image::ImageFormat::Png)),
            Format::Jpeg => once(image_rs::load_image(source, ::image::ImageFormat::Jpeg)),
        }?;
        for (i, mut img) in iter.enumerate() {
            if let Some(resize) = &resize {
                let (width, height) = resize.calculate_dimensions(&image.width() as usize, &image.height() as usize);
                img = img.resize(width as u32, height as u32, image_rs::imageops::FilterType::Lanczos3);
            }
            let path = util::create_path(path_template, &input_fpath, i + 1, places);
            img.to_rgba8()
                .save(path)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        Ok(())
    }

    pub fn into_format(self, format: Format) -> Result<Vec<u8>> {
        unimplemented!()
    }

    pub fn into_vec(self) -> Result<Vec<u8>> {
        self.into_format(self.format)
    }

    pub fn transform(self) -> Result<Image> {
        unimplemented!()
    }

    pub fn set_width(&mut self, width: usize) -> &mut Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.width = Some(width);
        self
    }

    pub fn set_height(&mut self, height: usize) -> &mut Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.height = Some(height);
        self
    }

    pub fn max_width(&mut self, max_width: usize) -> &mut Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.max_width = Some(max_width);
        self
    }

    pub fn max_height(&mut self, max_height: usize) -> &mut Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.max_height = Some(max_height);
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.scale = Some(scale);
        self
    }
}
