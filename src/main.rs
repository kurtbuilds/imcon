#![allow(unused)]
mod cli;
mod util;

use std::path::{Path, PathBuf};
use std::str::FromStr;
use clap::Arg;
use imcon::{DataSource, Format, Image};
use anyhow::Result;
use imcon::Format::Pdf;
use crate::util::resolve_hex_color;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");


/// Based on the command line inputs, infer the format of the input data.
fn resolve_input_format(input: &str, input_format: Option<&str>) -> Result<Format> {
    if let Some(format) = input_format {
        return Format::from_str(format).map_err(|_| anyhow::anyhow!("Unknown input format"))
    }
    if input.starts_with('#') && vec![4, 5, 7, 9].contains(&input.len()) {
        return Ok(Format::Bmp)
    }
    if let Some(ext) = Path::new(input).extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        return Format::from_str(&ext).map_err(|_| anyhow::anyhow!("Unknown input format"))
    }
    Err(anyhow::anyhow!("Could not determine input format."))
}


fn resolve_output_format(output_path: &Option<&str>, output_format: Option<&str>, input_format: Format) -> Result<Format> {
    if let Some(output) = output_format {
        return Format::from_str(output).map_err(|_| anyhow::anyhow!("Unknown output format"))
    }
    if let Some(output) = output_path {
        if let Some(ext) = Path::new(output).extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            return Format::from_str(&ext).map_err(|_| anyhow::anyhow!("Unknown output format"));
        }
    }
    Ok(match input_format {
        Format::Png => Format::Png,
        Format::Jpeg => Format::Jpeg,
        Format::Heif => Format::Jpeg,
        Format::Pdf => Format::Png,
        Format::Bmp => Format::Png,
    })
}



fn main() -> Result<()> {
    let args = cli::clap_app().get_matches();

    let allow_overwrite = args.is_present("force");
    let input = args.values_of("input").unwrap();
    for filepath in input {
        let input_format = resolve_input_format(filepath, args.value_of("input-format"))?;

        let mut im = Image::new(input_format, DataSource::File(PathBuf::from(filepath)));

        if let Some(width) = args.value_of("width") {
            im = im.set_width(width.parse()?);
        }
        if let Some(height) = args.value_of("height") {
            im = im.set_height(height.parse()?);
        }
        if let Some(scale) = args.value_of("scale") {
            im = im.scale(scale.parse()?);
        }
        if let Some(max_width) = args.value_of("max-width") {
            im = im.max_width(max_width.parse()?);
        }
        if let Some(max_height) = args.value_of("max-height") {
            im = im.max_height(max_height.parse()?);
        }

        let output_path = args.value_of("output");
        let output_format = resolve_output_format(
            &output_path,
            args.value_of("output-format"),
            input_format
        )?;
        let path_template = output_path.map(String::from).unwrap_or_else(
            || match input_format {
                Format::Pdf => format!("{{}}_{{i}}.{}", output_format.as_str()),
                _ => format!("{{}}.{}", output_format.as_str()),
            }
        );
        im.save_all(&path_template, allow_overwrite)?;
    }
    Ok(())
}
