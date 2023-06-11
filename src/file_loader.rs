use std::fs::File;
use std::{env, io::BufReader};

use crate::scene::Scene;

pub fn load_and_deserialize_scene() -> Scene {
    let path_to_xml_file = env::args().nth(1).unwrap();
    let file = File::open(path_to_xml_file).expect("Failed to open the file at the specified file path");
    let reader = BufReader::new(file);
    quick_xml::de::from_reader(reader).expect("Deserialization failed!")
}