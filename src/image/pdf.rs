use std::path::PathBuf;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::pdfium::Pdfium;
use crate::image::{DataSource};
use crate::transform::{Resize};
use anyhow::Result;
use image::{DynamicImage};
use pdfium_render::pages::{PdfPageIndex};
use pdfium_render::document::PdfDocument;


fn make_library_binding() -> Pdfium {
    let bind = Pdfium::bind_to_system_library()
        .expect("Failed to bind to Pdfium system library");
    Pdfium::new(bind)
}


impl Into<PdfBitmapConfig> for Resize {
    fn into(self) -> PdfBitmapConfig {
        let mut config = PdfBitmapConfig::new();
        if let Some(w) = self.width {
            config = config.set_target_width(w as u16);
        }
        if let Some(h) = self.height {
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


fn get_page_as_image(doc: &PdfDocument, i: PdfPageIndex, config: PdfBitmapConfig) -> Result<DynamicImage> {
    let pages = doc.pages();
    let page = pages.get(i as PdfPageIndex)
        .map_err(|_e| anyhow::anyhow!("Page out of bounds"))?;
    let mut bitmap = page.get_bitmap_with_config(&config)
        .map_err(|e| anyhow::anyhow!("Failed to get bitmap: {:?}", e))?;
    Ok(bitmap.as_image())
}

pub fn open_page(path: &PathBuf, i: usize, resize: Option<Resize>) -> Result<DynamicImage> {
    let config = resize.map(|r| r.into()).unwrap_or_default();
    let pdfium = make_library_binding();
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }
    let doc = pdfium.load_pdf_from_file(path.to_string_lossy().as_ref(), None)
        .map_err(|e| anyhow::anyhow!("Failed to load PDF document: {:?}", e))?;
    get_page_as_image(&doc, i as PdfPageIndex, config)
}


pub fn read_page(data: &[u8], i: usize, resize: Option<Resize>) -> Result<DynamicImage> {
    let config = resize.map(|r| r.into()).unwrap_or_default();
    let pdfium = make_library_binding();
    let doc = pdfium.load_pdf_from_bytes(data, None)
        .map_err(|e| anyhow::anyhow!("Failed to load PDF document: {:?}", e))?;
    get_page_as_image(&doc, i as PdfPageIndex, config)
}


pub fn transform_all_pages_from_path<S>(path: &PathBuf, resize: Option<Resize>, transform: S) -> Result<()>
    where
        S: Fn(usize, usize, DynamicImage) -> Result<()>
{
    let config: PdfBitmapConfig = resize.map(|r| r.into()).unwrap_or_default();
    let pdfium = make_library_binding();
    let doc = pdfium.load_pdf_from_file(path.to_string_lossy().as_ref(), None)
        .map_err(|e| anyhow::anyhow!("Failed to load PDF document: {:?}", e))?;
    let pages = doc.pages();
    let num_pages = pages.len();
    for (i, page) in pages.iter().enumerate() {
        let mut bmp = page.get_bitmap_with_config(&config)
            .map_err(|e| anyhow::anyhow!("Failed to get bitmap: {:?}", e))?;
        let image = bmp.as_image();
        transform(i, num_pages as usize, image)?;
    }
    Ok(())
}