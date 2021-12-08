use crate::cropper;
use crate::finder::{find_files, find_tiles};
use indicatif::ProgressBar;

pub fn crop(source_directory: String, destination_directory: String) {
    let files = find_files(source_directory);
    let bar = ProgressBar::new(files.len() as u64);

    for file in bar.wrap_iter(files.into_iter()) {
        cropper::crop(&file.path(), &destination_directory);
    }
}

pub fn make(picture: String, tiles_per_side: u16, tiles_directory: String) {
    let tiles = find_tiles(tiles_directory);
}
