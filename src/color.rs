use crate::entities::Color;
use image::DynamicImage;

pub fn get_dominant_color(img: &DynamicImage) -> Color {
    let pixels = img.to_bytes();

    let mut total: [u32; 3] = [0, 0, 0];
    for i in 0..pixels.len() {
        total[i % 3] += pixels[i] as u32;
    }

    Color {
        red: (total[0] / (pixels.len() / 3) as u32) as u8,
        green: (total[1] / (pixels.len() / 3) as u32) as u8,
        blue: (total[2] / (pixels.len() / 3) as u32) as u8,
    }
}

pub fn compute_proximity(rgb1: Color, rgb2: Color) -> f32 {
    let labs1 = lab::rgbs_to_labs(&[[rgb1.red, rgb1.green, rgb1.blue]]);
    let labs2 = lab::rgbs_to_labs(&[[rgb2.red, rgb2.green, rgb2.blue]]);
    let color1 = labs1.first().unwrap();
    let color2 = labs2.first().unwrap();

    ((color2.l - color1.l) * (color2.l - color1.l)
        + ((color2.a - color1.a) * (color2.a - color1.a))
        + ((color2.b - color1.b) * (color2.b - color1.b)))
        .sqrt()
}
