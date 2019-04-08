use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
pub fn get_filenames(dir: &str) -> Vec<String> {
    let mut v = Vec::new();
    let dirs = std::fs::read_dir(Path::new(dir)).unwrap();
    for filename in dirs {
        v.push(format!("{:?}", filename.unwrap().path()));
    }

    return v;
}
