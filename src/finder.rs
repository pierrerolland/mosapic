use crate::entities::Tile;
use indicatif::ProgressBar;
use std::fs;
use std::fs::DirEntry;

pub fn find_files(directory: String) -> Vec<DirEntry> {
    let mut out = vec![];

    for entry in fs::read_dir(&directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            out.push(entry);
        }
    }

    out
}

pub fn find_tiles(tiles_directory: String) -> Vec<Tile> {
    let files = find_files(tiles_directory);
    let bar = ProgressBar::new(files.len() as u64);

    println!("Retrieving tiles...");
    bar.wrap_iter(files.into_iter())
        .map(|entry| Tile::from(entry.path()))
        .collect()
}
