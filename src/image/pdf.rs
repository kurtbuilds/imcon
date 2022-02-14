use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::pdfium::Pdfium;
use crate::image::{DataSource};
use crate::transform::{Resize};
use anyhow::Result;
use image::{DynamicImage, RgbaImage};
use pdfium_render::pages::{PdfDocumentPdfPageIterator, PdfPageIndex, PdfPages};
use once_cell::sync::Lazy;
use pdfium_render::document::PdfDocument;
use pdfium_render::page::PdfPage;


pub static PDFIUM: Lazy<Pdfium> = Lazy::new(|| {
    let bind = Pdfium::bind_to_system_library()
        .expect("Failed to bind to Pdfium system library");
    Pdfium::new(bind)
});


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


pub fn load_document<'a>(datasource: DataSource) -> Result<PdfDocument<'a>> {
    match datasource {
        DataSource::File(path) => {
            if !path.exists() {
                return Err(anyhow::anyhow!("File not found: {}", path.display()));
            }
            PDFIUM.load_pdf_from_file(path.to_string_lossy().as_ref(), None)
        }
        DataSource::Memory(bytes) => {
            PDFIUM.load_pdf_from_bytes(bytes.as_slice(), None)
        }
    }
        .map_err(|e| anyhow::anyhow!("Failed to load PDF document: {:?}", e))
}

pub fn load_image(datasource: DataSource, i: usize, resize: Option<Resize>) -> Result<DynamicImage> {
    let config = resize.map(|ref r| r.into())
        .unwrap_or(PdfBitmapConfig::new());
    let doc = load_document(datasource)?;
    Ok(doc)
        .and_then(|doc| doc.pages().get(i as PdfPageIndex))
        .and_then(|page| page.get_bitmap_with_config(&config))
        .map(|mut bitmap| bitmap.as_image())
        .map_err(|e| anyhow::anyhow!("Failed to load page: {:?}", e))
}


pub struct ExactSizePdfPageIterator<'a>{
    pub pages: PdfPages<'a>,
    pub next_index: PdfPageIndex,
    pub page_count: PdfPageIndex,
}


impl<'a> Iterator for ExactSizePdfPageIterator<'a> {
    type Item = PdfPage<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.page_count {
            return None;
        }
        let next = self.pages.get(self.next_index);
        self.next_index += 1;
        match next {
            Ok(next) => Some(next),
            Err(_) => None,
        }
    }
}

impl<'a> ExactSizeIterator for ExactSizePdfPageIterator<'a> {
    fn len(&self) -> usize {
        self.page_count as usize
    }
}

impl<'a> ExactSizePdfPageIterator<'a> {
    pub fn new(pages: PdfPages<'a>) -> Self {
        ExactSizePdfPageIterator {
            pages,
            next_index: 0,
            page_count: pages.len(),
        }
    }
}

pub fn load_all_images(datasource: DataSource, resize: Option<Resize>) -> Result<Box<dyn ExactSizeIterator<Item=Result<DynamicImage>>>> {
    let config = resize.map(|ref r| r.into())
        .unwrap_or(PdfBitmapConfig::new());
    let doc = load_document(datasource)?;

    let pages = doc.pages();
    Ok(Box::new(ExactSizePdfPageIterator {
        page_count: pages.len(),
        next_index: 0,
        pages,
    }
        .map(|page| page
            .get_bitmap_with_config(&config)
            .map(|mut bitmap| bitmap.as_image())
            .map_err(|e| anyhow::anyhow!("Failed to load page: {:?}", e))
        )
    ))
}