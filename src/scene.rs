use crate::{deserialization_helpers, vec3::Color, camera::Camera, light::light::LightList, hittable_list::HittableList, sphere::deserialize_surfaces};

use serde::Deserialize;
use deserialization_helpers::deserialize_color;

#[derive(Deserialize)]
pub struct Scene{
    #[serde(rename = "@output_file")]
    pub output_file: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub background_color: Color,
    pub camera: Camera,
    #[serde(default)]
    pub lights: LightList,
    #[serde(deserialize_with = "deserialize_surfaces")]
    pub surfaces: HittableList,
}
