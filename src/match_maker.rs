use crate::color::compute_proximity;
use crate::entities::{ImagePart, Proximity, Tile};
use std::cmp::Ordering;

pub fn make_match(parts: Vec<ImagePart>, tiles: Vec<Tile>) -> Vec<Proximity> {
    let mut map = build_proximities_map(parts, tiles);
    let mut result = vec![];

    println!("Building the best mosaic");
    loop {
        if map.len() == 0 {
            break;
        }
        let proximity = map.first().unwrap().clone();
        remove_references_to_tile_and_part(&mut map, &proximity);
        result.push(proximity);
    }

    result.sort_by(|a, b| match a.part.y.cmp(&b.part.y) {
        Ordering::Equal => a.part.x.cmp(&b.part.x),
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    });

    result
}

fn build_proximities_map(parts: Vec<ImagePart>, tiles: Vec<Tile>) -> Vec<Proximity> {
    let mut out = vec![];

    for i in 0..parts.len() {
        for j in 0..tiles.len() {
            out.push(Proximity {
                part: ImagePart {
                    x: parts[i].x,
                    y: parts[i].y,
                    dominant_color: parts[i].dominant_color,
                },
                tile: Tile {
                    path: tiles[j].path.clone(),
                    dominant_color: tiles[j].dominant_color,
                },
                proximity: compute_proximity(parts[i].dominant_color, tiles[j].dominant_color),
            });
        }
    }

    out.sort_by(|a, b| match a.proximity.partial_cmp(&b.proximity) {
        Some(ordering) => ordering,
        None => Ordering::Equal,
    });

    out
}

fn remove_references_to_tile_and_part(map: &mut Vec<Proximity>, proximity: &Proximity) {
    map.retain(|item| {
        item.tile.path != proximity.tile.path
            && !(item.part.x == proximity.part.x && item.part.y == proximity.part.y)
    });
}
