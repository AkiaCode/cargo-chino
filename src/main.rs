use viuer::{print_from_file, Config};
use std::path::PathBuf;

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

    let mut img_path = PathBuf::new();
    img_path.push(std::env::current_dir().unwrap().to_str().unwrap());
    img_path.push("img.png");


    print_from_file(img_path.to_str().unwrap(), &conf).expect("Image printing failed.");
}
