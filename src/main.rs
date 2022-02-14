#![allow(unused)]

pub mod convert;

use std::path::Path;
use std::sync::Once;
use clap::Arg;
use image::ImageFormat;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::pdfium::Pdfium;
use convert::image::Format;
pub mod error;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = clap::App::new(NAME)
        .version(VERSION)
        .arg(Arg::new("input")
            .help("Sets the input file to use")
            .required(true)
            .multiple_values(true)
        )
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .help("Sets the output format")
            .takes_value(true)
            .conflicts_with("output")
            .possible_values(&["png", "jpg", "jpeg"])
        )
        .arg(Arg::new("width")
            .long("width")
            .short('w')
            .takes_value(true)
        )
        .arg(Arg::new("input-format")
            .long("input-format")
            .short('i')
            .takes_value(true)
        )
        .arg(Arg::new("scale")
            .long("scale")
            .takes_value(true)
        )
        .arg(Arg::new("height")
            .long("height")
            .short('h')
            .takes_value(true)
        )
        .arg(Arg::new("max-width")
            .long("max-width")
            .short('W')
            .takes_value(true)
        )
        .arg(Arg::new("max-height")
            .long("max-height")
            .short('H')
            .takes_value(true)
        )
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("Sets the output file path to use. Use the following placeholders as needed:
              '{}':   input file name without file extension
              '{i}':  number of the output file (starting from 1).
              '{/}':  input file path without file extension
              '{.}':  input file name with file extension
            ")
            .takes_value(true)
            .conflicts_with("format")
        )
        .get_matches();

    let input = args.values_of("input").unwrap();
    for input in input {
        let data = match args.value_of("input-format").or_else(|| {
            let input = Path::new(input);
            input.extension()
                .map(|ext| ext.to_str().unwrap_or(""))
        }) {
            Some(ext) => {
                match ext.to_lowercase().as_ref() {
                    "pdf" => Format::Pdf,
                    "heic" => Format::Heif,
                    _ => { return Err(format!("Unrecognized file format: {}", ext).into()); }
                }
            }
            None => { return Err("Failed to determine input format".into()); }
        };

        let mut im = convert::Image::new(input, data);
        if let Some(width) = args.value_of("width") {
            im.target_width(width.parse()?);
        }
        if let Some(height) = args.value_of("height") {
            im.target_height(height.parse()?);
        }
        if let Some(scale) = args.value_of("scale") {
            im.scale(scale.parse()?);
        }
        if let Some(max_width) = args.value_of("max-width") {
            im.max_width(max_width.parse()?);
        }
        if let Some(max_height) = args.value_of("max-height") {
            im.max_height(max_height.parse()?);
        }

        let output = args.value_of("output")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
            let path = Path::new(input);
            let extension = args.value_of("format")
                .unwrap_or_else(|| match path.extension()
                    .map(|ext| ext.to_str().unwrap_or(""))
                    .unwrap_or("")
                    .to_lowercase()
                    .as_ref() {
                    "pdf" => "png",
                    "heic" => "png",
                    _ => "png",
                }
            );
            format!("{}{{i}}.{}", path.file_stem().unwrap().to_string_lossy(), extension)
        });
        im.save_pages_to_path(&output)?;
    }
    Ok(())
}
