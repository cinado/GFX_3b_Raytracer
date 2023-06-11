use deserialization_helpers::deserialize_color;
use serde::Deserialize;

use crate::{
    tracer::hittable_list::HittableList,
    utils::{deserialization_helpers, vec3::Color},
};

use super::{camera::Camera, light::LightList, surfaces::deserialize_surfaces};

#[derive(Deserialize)]
pub struct Scene {
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
