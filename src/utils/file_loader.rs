use std::fs::{self, File};
use std::{env, io::BufReader};

use png::Decoder;

use crate::scene::material::Texture;
use crate::scene::scene::Scene;

use super::vec3::Color;

pub fn load_and_deserialize_scene() -> Scene {
    let path_to_xml_file = env::args().nth(1).unwrap();
    let file =
        File::open(path_to_xml_file).expect("Failed to open the file at the specified file path");
    let reader = BufReader::new(file);
    quick_xml::de::from_reader(reader).expect("Deserialization failed!")
}

pub fn load_obj_file(name: &String) -> Result<String, std::io::Error> {
    let path_to_obj_file = format!("./scenes/{}", name);

    match fs::read_to_string(path_to_obj_file) {
        Ok(contents) => Ok(contents),
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            Err(err)
        }
    }
}

pub fn load_texture_file(name: &String) -> Texture {
    let path_to_texture_file = format!("./scenes/{}", name);

    let file = File::open(path_to_texture_file).expect("Failed to open image file");
    let decoder = Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();

    let mut texture_pixels = Vec::new();

    for pixel in buf.chunks_exact_mut(3) {
        texture_pixels.push(Color::from_values(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        ));
    }
    Texture{
        name: name.to_string(),
        width: info.width as f32,
        height: info.height as f32,
        texture_pixels
    }
}
