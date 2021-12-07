use palette::Lab;

pub struct Thumb {
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
    thumb: Thumb,
    proximity: f32,
}
