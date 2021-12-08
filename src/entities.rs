use crate::color::{get_dominant_color, remove_alpha};
use image::io::Reader;
use image::GenericImageView;
use palette::Lab;
use std::path::PathBuf;

pub struct Tile {
    path: String,
    dominant_color: Lab,
}

pub struct ImagePart {
    x: u16,
    y: u16,
    dominant_color: Lab,
}

pub struct Proximity {
    part: ImagePart,
    tile: Tile,
    proximity: f32,
}

impl From<PathBuf> for Tile {
    fn from(path: PathBuf) -> Self {
        let img = Reader::open(&path).unwrap().decode().unwrap().to_bytes();
        let dominant_color = get_dominant_color(img);

        Tile {
            dominant_color,
            path: path.to_string_lossy().to_string(),
        }
    }
}
