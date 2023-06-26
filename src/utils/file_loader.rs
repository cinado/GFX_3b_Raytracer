use std::fs::{self, File};
use std::{env, io::BufReader};

use crate::scene::scene::Scene;

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
