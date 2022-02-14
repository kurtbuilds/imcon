use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::pdfium::Pdfium;
use crate::image::{DataSource, Image};
use crate::transform::{Resize, Transform};
use anyhow::Result;
use image::{DynamicImage, RgbaImage};
use pdfium_render::pages::{PdfPageIndex, PdfPages};
use once_cell::sync::Lazy;
use pdfium_render::document::PdfDocument;


pub static PDFIUM: Lazy<Pdfium> = Lazy::new(|| {
    let bind = Pdfium::bind_to_system_library()
        .expect("Failed to bind to Pdfium system library");
    Pdfium::new(bind)
});


impl Into<PdfBitmapConfig> for &Resize {
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
            config = config.scale_page_by_factor(s);
        }
        config
    }
}


pub fn load_document<'a>(datasource: DataSource) -> Result<PdfDocument<'a>> {
    match datasource {
        DataSource::File(path) => {
            if !path.exists() {
                return Err(anyhow::anyhow!("File not found: {}", path.display()));
            }
            PDFIUM.load_pdf_from_file(path.to_string_lossy().as_ref(), None)
        }
        DataSource::Memory(bytes) => {
            PDFIUM.load_pdf_from_memory(bytes.as_slice(), None)
        }
    }
        .map_err(|e| anyhow::anyhow!("Failed to load PDF document: {:?}", e))
}

pub fn load_image(datasource: DataSource, i: usize, resize: Option<Resize>) -> Result<DynamicImage> {
    let config = resize.map(|r| r.into())
        .unwrap_or(PdfBitmapConfig::new());
    let doc = load_document(datasource)?;
    doc
        .and_then(|doc| doc.pages.get(i as PdfPageIndex))
        .and_then(|page| page.get_bitmap_with_config(&config))
        .map(|mut bitmap| bitmap.as_image())
        .map_err(|e| anyhow::anyhow!("Failed to load page: {:?}", e))
}


pub fn load_all_images(datasource: DataSource, resize: Option<Resize>) -> Result<Box<dyn Iterator<Item=DynamicImage>>> {
    let config = resize.map(|r| r.into())
        .unwrap_or(PdfBitmapConfig::new());
    let doc = load_document(datasource)?;
    Ok(Box::new(doc.pages()
        .iter()
        .map(|page| page
            .get_bitmap_with_config(&config)
        ).and_then(|bitmap| bitmap.as_image())
    ))
}