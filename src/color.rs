use image::{ImageBuffer, Rgba};
use kmeans_colors::{get_kmeans, Kmeans, Sort};
use palette::{Lab, Pixel, Srgb};

pub fn get_dominant_color(img_vec: Vec<u8>) -> Lab {
    let lab: Vec<Lab> = Srgb::from_raw_slice(&img_vec)
        .iter()
        .map(|x| x.into_format().into())
        .collect();

    let mut result = Kmeans::new();
    let runs = 3;
    let k = 8;
    let max_iter = 20;
    let converge = 5.0;
    let seed = 0;
    let verbose = false;

    for i in 0..runs {
        let run_result = get_kmeans(k, max_iter, converge, verbose, &lab, seed + i as u64);

        if run_result.score < result.score {
            result = run_result;
        }
    }

    let res = Lab::sort_indexed_colors(&result.centroids, &result.indices);

    Lab::get_dominant_color(&res).unwrap()
}

pub fn compute_proximity(color1: Lab, color2: Lab) -> f32 {
    ((color2.l - color1.l) * (color2.l - color1.l)
        + ((color2.a - color1.a) * (color2.a - color1.a))
        + ((color2.b - color1.b) * (color2.b - color1.b)))
        .sqrt()
}
