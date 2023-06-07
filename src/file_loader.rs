use std::fs::File;
use std::{env, io::BufReader};

use crate::scene::Scene;

pub fn load_and_deserialize_scene() -> Result<Scene, Box<dyn std::error::Error>> {
    let path_to_xml_file = env::args().nth(1).ok_or("No path specified!")?;
    //let path_to_xml_file = "./scenes/example1.xml";
    //let current_dir = env::current_dir();
    //println!("{:?}", current_dir.unwrap().as_os_str().to_str());
    let file = File::open(path_to_xml_file)?;
    let reader = BufReader::new(file);

    //Read file and deserialize content --> return deserialized scene
    /*match from_reader::<_, Scene>(reader) {
        Ok(scene) => Ok(scene),
        Err(err) => Err(err.into()),
    }*/
    quick_xml::de::from_reader(reader).map_err(|error| error.into())
}