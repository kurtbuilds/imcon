use std::{fs, io};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::transform::{Resize, Transform};
use anyhow::Result;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use ::image::imageops::FilterType;
// use kmeans::{KMeans, KMeansConfig};

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
    Bmp,
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Pdf => "pdf",
            Format::Heif => "heic",
            Format::Png => "png",
            Format::Jpeg => "jpg",
            Format::Bmp => "bmp",
        }
    }
}


impl FromStr for Format {
    type Err = ();
    fn from_str(input: &str) -> Result<Format, Self::Err> {
        Ok(match input.to_lowercase().as_str() {
            "png"  => Format::Png,
            "pdf"  => Format::Pdf,
            "jpeg" | "jpg" => Format::Jpeg,
            "heic" => Format::Heif,
            "bmp" => Format::Bmp,
            _      => return Err(()),
        })
    }
}


pub struct Metadata {
    width: usize,
    height: usize,
}


pub enum DataSource {
    File(PathBuf),
    Memory(Vec<u8>),
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
    pub fn new(format: Format, source: DataSource) -> Self {
        Self {
            format,
            source,
            metadata: None,
            resize: None,
            transforms: vec![],
        }
    }
}


impl Image {
    pub fn open<S: Into<PathBuf>>(path: S) -> Result<Self> {
        let path = path.into();
        let ext = path.extension()
            .expect("No file extension or file extension unrecognized.")
            .to_string_lossy();
        let format = Format::from_str(&ext)
            .map_err(|e| anyhow::anyhow!("Unknown file extension: {}", ext))?;
        Ok(Self::new(format, DataSource::File(path)))
    }

    pub fn read(data: &[u8], format: Format) -> Result<Self> {
        Ok(Self::new(format, DataSource::Memory(data.to_vec())))
    }

    // pub fn read<S: io::Read + 'static>(reader: S, format: Format) -> Self {
    //     Self::new(format, DataSource::Memory(Box::new(reader)), vec![])
    // }

    pub fn read_unknown_format<S: io::Read>(_reader: S) -> Self {
        unimplemented!()
    }

    pub fn save(self, path: &str) -> Result<()> {
        self.to_image()?
            .to_rgba8()
            .save(path)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub fn save_all(self, path_template: &str, allow_overwrite: bool) -> Result<()> {
        let Image {source, resize, .. } = self;
        let input_fpath = match &source {
            DataSource::File(path) => path.clone(),
            DataSource::Memory(_) => PathBuf::from("stdin"),
        };
        let images = match self.format {
            Format::Pdf => pdf::load_all_images(source, None)?,
            Format::Heif => vec![heif::load_image(source, None)?],
            Format::Png => vec![image_rs::load_image(source, ::image::ImageFormat::Png)?],
            Format::Jpeg => vec![image_rs::load_image(source, ::image::ImageFormat::Jpeg)?],
            Format::Bmp => vec![image_rs::load_image(source, ::image::ImageFormat::Bmp)?],
        };
        let places = images.len();
        for (i, mut image) in images.into_iter().enumerate() {
            if let Some(resize) = &resize {
                let (width, height) = resize.calculate_dimensions(image.width(), image.height());
                image = image.resize(width, height, FilterType::Lanczos3);
            }
            let path = util::create_path(path_template, &input_fpath, i + 1, places);
            if fs::canonicalize(Path::new(&path))? == fs::canonicalize(&input_fpath)? && !allow_overwrite {
                return Err(anyhow::anyhow!("Output file would overwrite input file. Use --force to override."));
            }
            image.to_rgba8()
                .save(path)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        Ok(())
    }

    pub fn to_image(self) -> Result<DynamicImage> {
        let Image {source, resize, .. } = self;
        let mut image = match self.format {
            // TODO experiment with lib-specific resize instead of that from image-rthat from image-rs.
            Format::Pdf => pdf::load_image(source, 0, None),
            Format::Heif => heif::load_image(source, None),
            Format::Png => image_rs::load_image(source, ::image::ImageFormat::Png),
            Format::Jpeg => image_rs::load_image(source, ::image::ImageFormat::Jpeg),
            Format::Bmp => image_rs::load_image(source, ::image::ImageFormat::Bmp),
        }?;
        if let Some(resize) = resize {
            let (width, height) = resize.calculate_dimensions(image.width(), image.height());
            image = image.resize(width, height, FilterType::Lanczos3);
        }
        Ok(image)
    }

    pub fn into_format(self, _format: Format) -> Result<Vec<u8>> {
        unimplemented!()
    }

    pub fn into_vec(self) -> Result<Vec<u8>> {
        let format = self.format;
        self.into_format(format)
    }

    pub fn transform(self) -> Result<Image> {
        let format = self.format;
        let im = self.to_image()?;
        let im = im.to_rgba8();
        let vec = im.to_vec();
        Ok(Self::new(format, DataSource::Memory(vec)))
    }

    pub fn set_width(mut self, width: usize) -> Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.width = Some(width);
        self
    }

    pub fn set_height(mut self, height: usize) -> Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.height = Some(height);
        self
    }

    pub fn max_width(mut self, max_width: usize) -> Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.max_width = Some(max_width);
        self
    }

    pub fn max_height(mut self, max_height: usize) -> Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.max_height = Some(max_height);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        let resize = self.resize.get_or_insert(Resize::default());
        resize.scale = Some(scale);
        self
    }

    pub fn dominant_colors(self, n: usize) -> Result<Vec<(u8, u8, u8)>> {
        unimplemented!()
        // let im = self.to_image()?
        //     .to_rgb8();
        // let width = im.width();
        // let height = im.height();
        // let data = im.to_vec();
        // let kmean = kmeans::KMeans::new(data, (width * height) as usize, 3);
        // let result = kmean.kmeans_lloyd(n, 100, KMeans::init_kmeanplusplus, &KMeansConfig::default());
        // assert_eq!(result.centroids.len(), n * 3);
        // Ok(result.centroids.chunks(3).map(|c| (c[0], c[1], c[2])).collect())
    }
}
