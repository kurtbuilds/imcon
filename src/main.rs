use std::path::{Path, PathBuf};
use clap::{Arg, Args};
use imcon::{Image, Format, DataSource};
use anyhow::Result;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");


fn input_format(input: &str, input_format: Option<&str>) -> Result<Format> {
    if input.starts_with("#") && vec![4, 5, 7, 9].contains(&input.len()) {
        return Ok(Format::Png)
    }
    let ext = input_format.map(String::from).or_else(|| {
        let input = Path::new(input);
        input.extension().map(|s| s.to_string_lossy().into_owned())
    }).map(|s| s.to_lowercase())
        .ok_or_else(|| anyhow::anyhow!("Could not determine input format"))?;
    return Ok(match ext.as_ref() {
        "pdf" => Format::Pdf,
        "heic" => Format::Heif,
        "jpeg" | "jpg" => Format::Jpeg,
        "png" => Format::Png,
        _ => { return Err(anyhow::anyhow!("Unrecognized file format: {}", ext)); }
    })
}

fn main() -> Result<()> {
    let args = clap::App::new(NAME)
        .version(VERSION)
        .arg(Arg::new("input")
            .help("Sets the input file to use")
            .required(true)
            .multiple_values(true)
        )
        .arg(Arg::new("output-format")
            .long("output-format")
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
              '{dir}':  input file dir
              '{filename}':  input file name with file extension
            ")
            .takes_value(true)
            .conflicts_with("output-format")
        )
        .get_matches();

    let input = args.values_of("input").unwrap();
    for input in input {
        let format = input_format(input, args.value_of("input-format"))?;
        let mut im = Image::new(format, DataSource::File(PathBuf::from(input)));
        if let Some(width) = args.value_of("width") {
            im.set_width(width.parse()?);
        }
        if let Some(height) = args.value_of("height") {
            im.set_height(height.parse()?);
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
        im.save_all(&output)?;
    }
    Ok(())
}
