use std::iter::{repeat, repeat_with};
use std::path::{Iter, PathBuf};
use std::pin::Pin;
use std::sync::Mutex;
use image::{RgbaImage, RgbImage};
use lazy_static::lazy_static;
use once_cell::sync::{Lazy, OnceCell};
use pdfium_render::bitmap::PdfBitmap;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::document::PdfDocument;
use pdfium_render::page::PdfPage;
use pdfium_render::pdfium::Pdfium;
use std::sync::Once;

enum ImageData {
    Pdf,
}

pub struct Image {
    inner: ImageData,
    input_path: PathBuf,

    target_width: Option<usize>,
    target_height: Option<usize>,
    max_width: Option<usize>,
    max_height: Option<usize>,
}


impl Image {
    pub fn new<S: Into<PathBuf>>(path: S) -> Self {
        let path = path.into();
        match path.extension().expect("No file extension.").to_string_lossy().as_ref() {
            "pdf" => {
                Image {
                    inner: ImageData::Pdf,
                    input_path: path,
                    target_width: None,
                    target_height: None,
                    max_width: None,
                    max_height: None,
                }
            }
            _ => panic!("Unsupported file extension."),
        }
    }

    pub fn save(&self, path: &str) {
        unimplemented!()
    }

    pub fn save_pages_to_path(&self, path_template: &str) {
        match self.inner {
            ImageData::Pdf => {
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
                let bind = Pdfium::bind_to_system_library().unwrap();
                let pdfium = Pdfium::new(bind);
                let doc = pdfium.load_pdf_from_file(self.input_path.to_string_lossy().as_ref(), None).unwrap();
                let places = doc.pages().len().to_string().len();
                doc.pages().iter().enumerate().for_each(|(i, page)| {
                    let path = path_template
                        .replace("{}", self.input_path.file_stem().unwrap().to_string_lossy().as_ref())
                        .replace("{i}", format!("{:0places$}", i+1, places=places).as_ref());
                    page.get_bitmap_with_config(&config).unwrap()
                        .as_image()
                        .to_rgba8()
                        .save(path);
                })
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
}
