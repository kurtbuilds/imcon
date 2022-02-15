use clap::Arg;
use crate::{NAME, VERSION};

pub fn clap_app() -> clap::App<'static> {
    clap::App::new(NAME)
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
        .arg(Arg::new("input-format")
            .long("input-format")
            .takes_value(true)
        )
        .arg(Arg::new("scale")
            .long("scale")
            .takes_value(true)
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
        .arg(Arg::new("metadata")
            .long("metadata")
            .short('m')
            .multiple_values(true)
            .multiple_occurrences(true)
            .conflicts_with_all(&["width", "height", "max-width", "max-height", "scale", "output-format", "output"])
            .possible_values(&["all"])
        )
        .arg(Arg::new("dominant")
            .long("dominant")
            .takes_value(true)
            .conflicts_with_all(&["width", "height", "max-width", "max-height", "scale", "output-format", "output"])
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
        .arg(Arg::new("force")
            .long("force")
            .short('f')
            .help("By default, imcon refuses to write files in place. Use this flag to override this behavior.")
        )
}
