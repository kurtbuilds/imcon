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


impl Into<PdfBitmapConfig> for &Resize {
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


pub fn load_document(pdfium: &Pdfium, datasource: DataSource) -> Result<PdfDocument> {
    match datasource {
        DataSource::File(path) => {
            if !path.exists() {
                return Err(anyhow::anyhow!("File not found: {}", path.display()));
            }
            pdfium.load_pdf_from_file(path.to_string_lossy().as_ref(), None)
        }
        DataSource::Memory(bytes) => {
            pdfium.load_pdf_from_bytes(bytes.as_slice(), None)
        }
    }
        .map_err(|e| anyhow::anyhow!("Failed to load PDF document: {:?}", e))
}


pub fn load_image(datasource: DataSource, i: usize, resize: Option<Resize>) -> Result<DynamicImage> {
    let config = resize.map(|ref r| r.into()).unwrap_or_default();
    let pdfium = make_library_binding();
    let doc = load_document(&pdfium, datasource)?;
    let pages = doc.pages();
    let page = pages.get(i as PdfPageIndex)
        .map_err(|_e| anyhow::anyhow!("Page out of bounds"))?;
    let mut bitmap = page.get_bitmap_with_config(&config)
        .map_err(|e| anyhow::anyhow!("Failed to get bitmap: {:?}", e))?;
    Ok(bitmap.as_image())
}


pub fn load_all_images(datasource: DataSource, resize: Option<Resize>) -> Result<Vec<DynamicImage>> {
    let config = resize.map(|ref r| r.into()).unwrap_or_default();
    let pdfium = make_library_binding();
    let doc = load_document(&pdfium, datasource)
        .map_err(|e| anyhow::anyhow!("Failed to load PDF document: {:?}", e))?;
    let pages = doc.pages();
    pages.iter().map(|page| page
        .get_bitmap_with_config(&config)
        .map(|mut bitmap| bitmap.as_image())
        .map_err(|e| anyhow::anyhow!("Failed to load page: {:?}", e))
    ).collect()
}