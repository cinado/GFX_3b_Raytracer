use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Scene{
    #[serde(rename = @output_file)]
    pub output_file: String,
    pub background_color: Color,
    pub camera: Camera,
    pub lights: Lights,
    pub surfaces: Surfaces,
}