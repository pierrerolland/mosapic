use image::imageops::{resize, FilterType};
use image::io::Reader;
use image::{DynamicImage, GenericImageView};
use std::fs;
use std::path::Path;

pub fn crop(picture_path: &Path, directory: &String) {
    fs::create_dir_all(directory).unwrap();

    let img = Reader::open(picture_path).unwrap().decode().unwrap();
    let dimensions_and_offset = guess_crop_dimensions_and_offset(&img);
    let dest = format!(
        "{}/{}",
        directory,
        picture_path.file_name().unwrap().to_string_lossy()
    );

    crop_and_save(img, dimensions_and_offset, dest);
}

struct CropInfo {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn guess_crop_dimensions_and_offset(image: &DynamicImage) -> CropInfo {
    let (width, height) = (image.width(), image.height());

    if width > height {
        return CropInfo {
            x: (width - height) / 2,
            y: 0,
            width: height,
            height,
        };
    }

    CropInfo {
        x: 0,
        y: (height - width) / 2,
        width,
        height: width,
    }
}

fn crop_and_save(mut image: DynamicImage, info: CropInfo, to: String) {
    let cropped =
        image::imageops::crop(&mut image, info.x, info.y, info.width, info.height).to_image();
    let pixels = resize(&cropped, 100, 100, FilterType::Nearest);

    pixels.save(to).unwrap();
}
