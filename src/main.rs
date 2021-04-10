use viuer::{print_from_file, Config};
use std::path::PathBuf;
use std::fs;

fn main() {
    let conf = Config {
        transparent: true,
        absolute_offset: false,
        x: 0,
        y: 0,
        width: Some(80),
        height: Some(30),
        ..Default::default()
    };

    let out_dir = PathBuf::from("img.png");

    print_from_file(fs::canonicalize(&out_dir).unwrap().to_str().unwrap(), &conf).expect("Image printing failed.");
}
