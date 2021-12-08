use crate::color::get_dominant_color;
use crate::entities::ImagePart;
use image::imageops::FilterType;
use image::io::Reader;
use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage, Rgba};
use std::fs;
use std::path::Path;

pub fn crop(picture_path: &Path, directory: &String, should_resize: bool) {
    fs::create_dir_all(directory).unwrap();

    let mut img = Reader::open(picture_path).unwrap().decode().unwrap();
    let dimensions_and_offset = guess_crop_dimensions_and_offset(&img);
    let dest = format!(
        "{}/{}",
        directory,
        picture_path.file_name().unwrap().to_string_lossy()
    );

    let mut cropped = image::imageops::crop(
        &mut img,
        dimensions_and_offset.x,
        dimensions_and_offset.y,
        dimensions_and_offset.length,
        dimensions_and_offset.length,
    )
    .to_image();

    if should_resize {
        cropped = resize(cropped);
    }

    cropped.save(dest).unwrap();
}

pub fn pick(image: &RgbImage, x: u16, y: u16, tiles_per_side: u16) -> ImagePart {
    let mut cloned = image.clone();
    let tile_length = image.width() / tiles_per_side as u32;
    let start_x: u32 = x as u32 * tile_length;
    let start_y: u32 = y as u32 * tile_length;

    let cropped = DynamicImage::ImageRgb8(
        image::imageops::crop(&mut cloned, start_x, start_y, tile_length, tile_length).to_image(),
    );

    ImagePart {
        x,
        y,
        dominant_color: get_dominant_color(&cropped),
    }
}

struct CropInfo {
    x: u32,
    y: u32,
    length: u32,
}

fn guess_crop_dimensions_and_offset(image: &DynamicImage) -> CropInfo {
    let (width, height) = (image.width(), image.height());

    if width > height {
        return CropInfo {
            x: (width - height) / 2,
            y: 0,
            length: height,
        };
    }

    CropInfo {
        x: 0,
        y: (height - width) / 2,
        length: width,
    }
}

fn resize(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::imageops::resize(&image, 100, 100, FilterType::Nearest)
}
