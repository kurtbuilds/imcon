use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use ::image::{DynamicImage};
use anyhow::Result;
use crate::image::Format;



pub fn open_image(path: &PathBuf, format: Format) -> Result<DynamicImage> {
    let format = format.try_into()?;
    let f = File::open(path)?;
    let f = BufReader::new(f);
    ::image::load(f, format)
        .map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn read_image(data: Vec<u8>, format: Format) -> Result<DynamicImage> {
    let format = format.try_into()?;
    let f = Cursor::new(data);
    ::image::load(f, format)
        .map_err(|e| anyhow::anyhow!("{}", e))
}