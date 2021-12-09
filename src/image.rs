use crate::entities::Proximity;
use image::RgbImage;

pub fn draw(map: Vec<Proximity>, tiles_per_side: u16) {
    println!("Creating the final image");
    let mut image = RgbImage::new(tiles_per_side as u32 * 100, tiles_per_side as u32 * 100);

    for i in 0..map.len() {
        let tile = image::open(&map[i].tile.path).unwrap().into_rgb8();

        image::imageops::overlay(
            &mut image,
            &tile,
            map[i].part.x as u32 * 100,
            map[i].part.y as u32 * 100,
        );
    }

    image.save("out.jpg").unwrap();
}
