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

    let req = reqwest::blocking::get("https://raw.githubusercontent.com/AkiaCode/cargo-chino/main/img.png").unwrap().bytes().unwrap();

    let mut file = std::fs::File::create("./img.png").unwrap();
    std::io::copy(&mut req.as_ref(), &mut file).unwrap();

    print_from_file("./img.png", &conf).expect("Image printing failed.");
}
