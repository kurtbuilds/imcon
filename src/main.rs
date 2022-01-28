#![allow(unused)]
pub mod convert;

use std::sync::Once;
use clap::Arg;
use image::ImageFormat;
use pdfium_render::bitmap_config::PdfBitmapConfig;
use pdfium_render::pdfium::Pdfium;

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
        .arg(Arg::new("target_width")
            .long("width")
            .short('w')
        )
        .arg(Arg::new("target_height")
            .long("height")
            .short('h')
        )
        .arg(Arg::new("max-width")
            .long("max-width")
            .short('W')
        )
        .arg(Arg::new("max-height")
            .long("max-height")
            .short('H')
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
            .conflicts_with("format")
        )
        .get_matches();

    let input = "/Users/kurt/Downloads/Fundraising Deck - Pre-Fiverr.pptx.pdf";

    let im = convert::Image::new(input);
    im.save_pages_to_path("output{i}.png");
    // im.iter_images().enumerate().for_each(|(i, image)| {
    //     image.save(format!("output{}.png", i)).unwrap();
    // });

    Ok(())
}
