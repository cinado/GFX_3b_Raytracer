use quick_xml::de::from_reader;
use std::fs::File;
use std::{env, io::BufReader};

use crate::camera::Camera;
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

pub fn load_and_deserialize_test() -> Result<Camera, Box<dyn std::error::Error>> {
    let path_to_xml_file: String = env::args().nth(1).ok_or("No path specified!")?;
    //let path_to_xml_file = "./scenes/example1.xml";
    //let current_dir = env::current_dir();
    //println!("{:?}", current_dir.unwrap().as_os_str().to_str());

    /*let test = r#"<camera>
    <position x="0.0" y="0.0" z="1.0"/>
    <lookat x="0.0" y="0.0" z="-2.5"/>
    <up x="0.0" y="1.0" z="0.0"/>
    <horizontal_fov angle="45"/>
    <resolution horizontal="512" vertical="512"/>
    <max_bounces n="8"/>
</camera>"#;*/

    let file = File::open(path_to_xml_file)?;
    let reader = BufReader::new(file);

    //Read file and deserialize content --> return deserialized scene
    /*match from_reader::<_, Scene>(reader) {
        Ok(scene) => Ok(scene),
        Err(err) => Err(err.into()),
    }*/

    //quick_xml::de::from_str(test).map_err(|error| error.into())
    quick_xml::de::from_reader(reader).map_err(|error| error.into())
}
