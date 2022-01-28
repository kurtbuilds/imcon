#![allow(unused)]

pub mod convert;

use std::path::Path;
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
        .arg(Arg::new("width")
            .long("width")
            .short('w')
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
        println!("{}", input);
        let mut im = convert::Image::new(input);
        args.value_of("width").map(|w|
            im.target_width(w.parse::<usize>().unwrap()));
        args.value_of("height").map(|h|
            im.target_height(h.parse::<usize>().unwrap()));
        args.value_of("max-width").map(|w|
            im.max_width(w.parse::<usize>().unwrap()));
        args.value_of("max-height").map(|h|
            im.max_height(h.parse::<usize>().unwrap()));

        let output = args.value_of("output")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
            let path = Path::new(input);
            let extension = args.value_of("format")
                .unwrap_or_else(|| match path.extension()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_lowercase()
                    .as_ref() {
                    "pdf" => "png",
                    _ => "png",
                }
                );
            format!("{}{{i}}.{}", path.file_stem().unwrap().to_string_lossy(), extension)
        });
        im.save_pages_to_path(&output);
    }
    // im.iter_images().enumerate().for_each(|(i, image)| {
    //     image.save(format!("output{}.png", i)).unwrap();
    // });

    Ok(())
}
