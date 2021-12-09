use crate::cropper;
use crate::entities::Proximity;
use crate::finder::{find_files, find_tiles};
use crate::image::draw;
use crate::match_maker::make_match;
use crate::parser::parse;
use indicatif::ProgressBar;
use std::path::Path;

pub fn crop(source_directory: String, destination_directory: String) {
    let files = find_files(source_directory);
    let bar = ProgressBar::new(files.len() as u64);

    for file in bar.wrap_iter(files.into_iter()) {
        cropper::crop(&file.path(), &destination_directory, true);
    }
}

pub fn make(picture: String, tiles_per_side: u16, tiles_directory: String) {
    println!("Cropping picture...");
    let picture = Path::new(&picture);
    cropper::crop(
        picture,
        &picture.parent().unwrap().to_string_lossy().to_string(),
        false,
    );
    let parts_chunks = parse(picture, tiles_per_side);
    let tiles = find_tiles(tiles_directory);
    let mut final_map: Vec<Proximity> = vec![];

    for (_pos, parts) in parts_chunks.into_iter().enumerate() {
        let mut map = make_match(parts, &tiles);

        final_map.append(&mut map);
    }

    draw(final_map, tiles_per_side);
}
