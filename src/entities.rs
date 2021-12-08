use crate::color::get_dominant_color;
use image::io::Reader;
use image::{DynamicImage, Rgb, RgbImage};
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone)]
pub struct Tile {
    pub path: String,
    pub dominant_color: Color,
}

#[derive(Clone)]
pub struct ImagePart {
    pub x: u16,
    pub y: u16,
    pub dominant_color: Color,
}

#[derive(Clone)]
pub struct Proximity {
    pub part: ImagePart,
    pub tile: Tile,
    pub proximity: f32,
}

impl From<PathBuf> for Tile {
    fn from(path: PathBuf) -> Self {
        let img: RgbImage = Reader::open(&path).unwrap().decode().unwrap().into_rgb8();
        let img = DynamicImage::ImageRgb8(img);

        let dominant_color = get_dominant_color(&img);

        Tile {
            dominant_color,
            path: path.to_string_lossy().to_string(),
        }
    }
}

impl Into<Rgb<u8>> for Color {
    fn into(self) -> Rgb<u8> {
        Rgb([self.red, self.green, self.blue])
    }
}
