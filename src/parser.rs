use crate::cropper::pick;
use crate::entities::ImagePart;
use image::io::Reader;
use image::RgbImage;
use indicatif::ProgressBar;
use std::path::Path;

pub fn parse(picture: &Path, tiles_per_side: u16) -> Vec<Vec<ImagePart>> {
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

    if tiles_per_side <= 10 {
        return vec![out];
    }

    chunk(out, tiles_per_side)
}

fn chunk(mut list: Vec<ImagePart>, tiles_per_side: u16) -> Vec<Vec<ImagePart>> {
    let chunks_per_side = tiles_per_side / 10;

    list.sort_by(|a, b| {
        a.get_chunk(chunks_per_side)
            .cmp(&b.get_chunk(chunks_per_side))
    });

    list.chunks(100).map(|s| s.into()).collect()
}
