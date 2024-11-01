use sfml::graphics::Texture;
use std::collections::HashMap;
use sfml::cpp::FBox;

pub fn load_images() -> HashMap<&'static str, FBox<Texture>> {
    let file_names = ["back", "background", "base", "border", "forward", "left", "mouse", "mouth", "right", "up", "wave"];
    let mut hm: HashMap<&str, FBox<Texture>> = HashMap::new();

    for img in file_names {
        match Texture::from_file( &format!("./sprites/{}.png", img) ) {
            Ok(image) => hm.insert(img, image),
            Err(_) => panic!("./sprites/{}.png not found", img)
        };
    }

    return hm;
}
