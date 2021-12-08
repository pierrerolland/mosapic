extern crate clap;

mod color;
mod commands;
mod cropper;
mod entities;
mod finder;
mod image;
mod match_maker;
mod parser;

use crate::commands::{crop, make};
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("mosapic")
        .version("1.0.0")
        .author("Pierre Rolland <pierre@rolland.dev>")
        .about("Turn a pic into a mosaic made out of other pics")
        .subcommand(
            SubCommand::with_name("crop")
                .help("Crop the set of pictures to use as tiles")
                .arg(
                    Arg::with_name("SOURCE_DIRECTORY")
                        .help("The directory containing the pictures to crop")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("DESTINATION_DIRECTORY")
                        .help("The directory to copy the cropped images")
                        .required(true)
                        .default_value("/tmp/mosapic")
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("make")
                .help("Make the mosaic")
                .arg(
                    Arg::with_name("PICTURE")
                        .help("The picture to transform into a mosaic")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("TILES_PER_SIDE")
                        .help("The amount of tiles per side")
                        .required(true)
                        .default_value("10")
                        .index(2),
                )
                .arg(
                    Arg::with_name("TILES_DIRECTORY")
                        .help("The directory containing the pictures to use as tiles")
                        .required(true)
                        .default_value("/tmp/mosapic")
                        .index(3),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("crop") {
        crop(
            matches.value_of("SOURCE_DIRECTORY").unwrap().to_string(),
            matches
                .value_of("DESTINATION_DIRECTORY")
                .unwrap()
                .to_string(),
        );
    } else if let Some(matches) = matches.subcommand_matches("make") {
        make(
            matches.value_of("PICTURE").unwrap().to_string(),
            matches
                .value_of("TILES_PER_SIDE")
                .unwrap()
                .to_string()
                .parse::<u16>()
                .unwrap(),
            matches.value_of("TILES_DIRECTORY").unwrap().to_string(),
        );
    }
}
