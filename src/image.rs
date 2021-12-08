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

// pub fn draw_dominants(map: Vec<ImagePart>, tiles_per_side: u16) {
//     println!("Creating the debug image");
//     let mut image = RgbImage::new(tiles_per_side as u32 * 10, tiles_per_side as u32 * 10);
//
//     for i in 0..map.len() {
//         for y in (map[i].y * 10)..(map[i].y * 10 + 9) {
//             for x in (map[i].x * 10)..(map[i].x * 10 + 9) {
//                 image.put_pixel(x as u32, y as u32, map[i].dominant_color.clone().into());
//             }
//         }
//     }
//
//     image.save("out.jpg").unwrap();
// }
