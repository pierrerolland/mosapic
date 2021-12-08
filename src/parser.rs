use crate::cropper::pick;
use crate::entities::ImagePart;
use image::io::Reader;
use image::RgbImage;
use indicatif::ProgressBar;
use std::path::Path;

pub fn parse(picture: &Path, tiles_per_side: u16) -> Vec<ImagePart> {
    println!("Retrieving pieces of the base image");
    let picture: RgbImage = Reader::open(picture).unwrap().decode().unwrap().into_rgb8();

    let mut out = vec![];
    let bar = ProgressBar::new(tiles_per_side as u64 * tiles_per_side as u64);
    for x in 0..tiles_per_side {
        for y in 0..tiles_per_side {
            out.push(pick(&picture, x, y, tiles_per_side));
            bar.inc(1);
        }
    }

    out
}
