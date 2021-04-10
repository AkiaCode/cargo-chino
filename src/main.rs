use viuer::{print_from_file, Config};

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

    print_from_file("./img.png", &conf).expect("Image printing failed.");
}
