use std::path::{PathBuf};
use std::str::FromStr;
use crate::transform::{Resize, Transform};
use anyhow::Result;
use ::image::{DynamicImage, ImageFormat};
use ::image::imageops::FilterType;
use crate::util::create_path;

mod pdf;
mod heif;
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
            "png" => Format::Png,
            "pdf" => Format::Pdf,
            "jpeg" | "jpg" => Format::Jpeg,
            "heic" => Format::Heif,
            "bmp" => Format::Bmp,
            _ => return Err(()),
        })
    }
}

impl TryInto<ImageFormat> for Format {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<ImageFormat, Self::Error> {
        Ok(match self {
            Format::Png => ImageFormat::Png,
            Format::Jpeg => ImageFormat::Jpeg,
            Format::Bmp => ImageFormat::Bmp,
            _ => return Err(anyhow::anyhow!("Format unsupported by image-rs library.")),
        })
    }
}


#[allow(unused)]
pub struct Metadata {
    width: usize,
    height: usize,
}


pub enum DataSource {
    File(PathBuf, Format),
    Memory(Vec<u8>, Format),
    Image(DynamicImage),
}


impl DataSource {
    fn input_file(&self) -> Option<&PathBuf> {
        match self {
            DataSource::File(path, _) => Some(path),
            _ => None,
        }
    }
}

/// a pdf can be opened from a file or from Vec<u8>
/// an image can be opened from a file, buffer of pixels (Dynamic Image)


pub struct Image {
    source: DataSource,
    #[allow(unused)]
    metadata: Option<Metadata>,

    // Operations
    resize: Option<Resize>,
    transforms: Vec<Transform>,
}


impl Image {
    pub fn new(source: DataSource) -> Self {
        Self {
            source,
            metadata: None,
            resize: None,
            transforms: vec![],
        }
    }
}


fn apply_transforms(mut image: DynamicImage, resize: Option<Resize>, _transforms: Vec<Transform>) -> Result<DynamicImage> {
    if let Some(resize) = resize {
        let (width, height) = resize.calculate_dimensions(image.width(), image.height());
        image = image.resize(width, height, FilterType::Lanczos3);
    }
    Ok(image)
}

impl Image {
    pub fn open<S: Into<PathBuf>>(path: S) -> Result<Self> {
        let path = path.into();
        let ext = path.extension()
            .expect("No file extension or file extension unrecognized.")
            .to_string_lossy();
        let format = Format::from_str(&ext)
            .map_err(|_| anyhow::anyhow!("Unknown file extension: {}", ext))?;
        Ok(Self::new(DataSource::File(path, format)))
    }

    pub fn read(data: &[u8], format: Format) -> Result<Self> {
        Ok(Self::new(DataSource::Memory(data.to_vec(), format)))
    }

    pub fn save(self, path: &str) -> Result<()> {
        self.to_image()?
            .to_rgba8()
            .save(path)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub fn save_every_image(self, path_template: &str) -> Result<()> {
        match self.source {
            DataSource::File(ref src_path, format) => match format {
                Format::Pdf => {
                    let Image { resize, transforms, .. } = self;
                    return pdf::transform_all_pages_from_path(
                        &src_path, resize, |i, n_pages, image| {
                            let transforms = transforms.clone();
                            let image = apply_transforms(image, None, transforms)?;
                            let path = create_path(path_template, &src_path, i, n_pages);
                            image
                                .save(&path)
                                .map_err(|e| anyhow::anyhow!("{}", e))?;
                            return Ok(())
                        });
                }
                _ => {}
            }
            _ => {}
        };
        let path = if let DataSource::File(ref src_path, ..) = self.source {
            create_path(path_template, &src_path, 1, 1)
        } else {
            "stdin".to_string()
        };
        self.save(path.as_ref())
    }

    pub fn to_image(self) -> Result<DynamicImage> {
        let Image { source, resize, transforms, .. } = self;
        let image = match source {
            DataSource::File(path, format) => match format {
                Format::Pdf => pdf::open_page(&path, 0, None)?,
                Format::Heif => heif::open_image(&path, None)?,
                other_format => image_rs::open_image(&path, other_format)?,
            }
            DataSource::Memory(data, format) => match format {
                Format::Pdf => pdf::read_page(&data, 0, None)?,
                Format::Heif => heif::read_image(&data, None)?,
                other_format => image_rs::read_image(data, other_format)?,
            },
            DataSource::Image(im) => im
        };
        apply_transforms(image, resize, transforms)
    }

    pub fn apply(self) -> Result<Image> {
        let im = self.to_image()?;
        Ok(Self {
            source: DataSource::Image(im),
            metadata: None,
            resize: None,
            transforms: vec![],
        })
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

    // pub fn dominant_colors(self, n: usize) -> Result<Vec<(u8, u8, u8)>> {
    //     unimplemented!()
    //     // let im = self.to_image()?
    //     //     .to_rgb8();
    //     // let width = im.width();
    //     // let height = im.height();
    //     // let data = im.to_vec();
    //     // let kmean = kmeans::KMeans::new(data, (width * height) as usize, 3);
    //     // let result = kmean.kmeans_lloyd(n, 100, KMeans::init_kmeanplusplus, &KMeansConfig::default());
    //     // assert_eq!(result.centroids.len(), n * 3);
    //     // Ok(result.centroids.chunks(3).map(|c| (c[0], c[1], c[2])).collect())
    // }
}
