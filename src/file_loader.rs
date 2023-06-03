/*use std::env;
use std::fs::File;

fn load_and_deserialize_scene() -> Result<Scene, Box<dyn std::error::Error>>{
    let path_to_xml_file = env::args().collect().nth(1).ok_or("No path specified!");
    let file = File::open(path_to_xml_file);

    //Read file and deserialize content --> return deserialized scene
    OK()
}*/