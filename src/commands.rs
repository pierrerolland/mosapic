use crate::cropper;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::fs::DirEntry;

pub fn crop(source_directory: String, destination_directory: String) {
    let entries = get_entries(source_directory);
    let bar = ProgressBar::new(entries.len() as u64);

    for entry in bar.wrap_iter(entries.into_iter()) {
        cropper::crop(&entry.path(), &destination_directory);
    }
}

fn get_entries(directory: String) -> Vec<DirEntry> {
    let mut out = vec![];
    let dir = fs::read_dir(&directory).unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            out.push(entry);
        }
    }

    out
}
