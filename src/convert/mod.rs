use std::{fs, io};
use std::iter::{repeat, repeat_with};
use std::path::{Iter, PathBuf};
use std::pin::Pin;
use std::sync::Mutex;
use image::{DynamicImage, RgbaImage, RgbImage};
use lazy_static::lazy_static;
use once_cell::sync::{Lazy, OnceCell};
use pdfium_render::bitmap::PdfBitmap;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::document::PdfDocument;
use pdfium_render::page::PdfPage;
use pdfium_render::pdfium::Pdfium;
use std::sync::Once;
use libheif_rs::{Channel, ColorSpace, HeifContext, RgbChroma};
use libheif_rs::HeifErrorSubCode::NoFtypBox;

pub enum ImageData {
    Pdf,
    Heif,
}

pub struct Image {
    inner: ImageData,
    input_path: PathBuf,

    target_width: Option<usize>,
    target_height: Option<usize>,
    max_width: Option<usize>,
    max_height: Option<usize>,
    scale: Option<f32>,
}

impl Into<PdfBitmapConfig> for &Image {
    fn into(self) -> PdfBitmapConfig {
        let mut config = PdfBitmapConfig::new();
        if let Some(w) = self.target_width {
            config = config.set_target_width(w as u16);
        }
        if let Some(h) = self.target_height {
            config = config.set_target_height(h as u16);
        }
        if let Some(w) = self.max_width {
            config = config.set_maximum_width(w as u16);
        }
        if let Some(h) = self.max_height {
            config = config.set_maximum_height(h as u16);
        }
        if let Some(s) = self.scale {
            if s > 100.0 {
                config = config.scale_page_by_factor(s / 100.0);
            } else {
                config = config.scale_page_by_factor(s);
            }
        }
        config
    }
}

fn create_path(path_template: &str, input_path: &PathBuf, page: usize, n_pages: usize) -> String {
    path_template
        .replace("{}", input_path.file_stem().unwrap().to_string_lossy().as_ref())
        .replace("{i}", format!("{:0places$}", page + 1, places = n_pages).as_ref())
}

impl Image {
    pub fn new<S: Into<PathBuf>>(path: S, data_type: ImageData) -> Self {
        let path = path.into();
        Image {
            inner: data_type,
            input_path: path,
            target_width: None,
            scale: None,
            target_height: None,
            max_width: None,
            max_height: None,
        }
    }

    pub fn save(&self, path: &str) {
        unimplemented!()
    }

    pub fn save_pages_to_path(&self, path_template: &str) -> Result<(), io::Error> {
        match self.inner {
            ImageData::Pdf => {
                let config: PdfBitmapConfig = self.into();
                let bind = Pdfium::bind_to_system_library()
                    .expect("Failed to bind to Pdfium system library");
                let pdfium = Pdfium::new(bind);
                if !self.input_path.exists() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "File not found",
                    ));
                }
                let doc = pdfium.load_pdf_from_file(self.input_path.to_string_lossy().as_ref(), None)
                    .expect("Pdfium failed to load pdf");
                let places = doc.pages().len().to_string().len();
                doc.pages().iter().enumerate().map(|(i, page)| {
                    let path = create_path(path_template, &self.input_path, i, places);
                    page.get_bitmap_with_config(&config).unwrap()
                        .as_image()
                        .to_rgba8()
                        .save(&path)
                        .map(|_| println!("{path}: Wrote file."))
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }).collect()
            }
            ImageData::Heif => {
                let image = HeifContext::read_from_file(self.input_path.to_string_lossy().as_ref())
                    .and_then(|ctx| {
                        let handle = ctx.primary_image_handle()?;
                        let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), false)?;
                        let width = image.width(Channel::R).unwrap();
                        let height = image.height(Channel::R).unwrap();
                        let bytes = image.bits_per_pixel(Channel::R).unwrap() / 8;
                        println!("{}x{} {}", width, height, bytes);
                        let mut imgbuf = image::ImageBuffer::new(width, height);
                        // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                        //     let r = image.(x, y, Channel::R).unwrap();
                        //     let g = image.get_pixel(x, y, Channel::G).unwrap();
                        //     let b = image.get_pixel(x, y, Channel::B).unwrap();
                        //     *pixel = image::Rgb([r, g, b]);
                        // }
                        Ok(DynamicImage::ImageRgb8(imgbuf))
                    })
                    .or_else(|e| {
                        match e.sub_code {
                            NoFtypBox => {
                                println!("Trying backup parse...");
                                // iOS apparently saves jpgs as a heic file sometimes. Try to load it as a jpg.
                                // There's probably a better way to do this.
                                let stream = fs::File::open(self.input_path.to_string_lossy().as_ref())?;
                                let stream = io::BufReader::new(stream);
                                return image::load(stream, image::ImageFormat::Jpeg)
                                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                            }
                            _ => {}
                        }
                        Err(io::Error::new(io::ErrorKind::Other, e))
                    })?;
                let path = create_path(path_template, &self.input_path, 0, 1);
                image.save(&path)
                    .map(|_| println!("{path}: Wrote file."))
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            }
        }
    }

    pub fn target_width(&mut self, width: usize) -> &mut Self {
        self.target_width = Some(width);
        self
    }
    pub fn target_height(&mut self, h: usize) -> &mut Self {
        self.target_height = Some(h);
        self
    }
    pub fn max_width(&mut self, w: usize) -> &mut Self {
        self.max_width = Some(w);
        self
    }

    pub fn max_height(&mut self, h: usize) -> &mut Self {
        self.max_height = Some(h);
        self
    }

    pub fn scale(&mut self, s: f32) -> &mut Self {
        self.scale = Some(s);
        self
    }
}
