use std::rc::Rc;

use serde::{Deserialize, Deserializer};

use crate::{vec3::Color, deserialization_helpers::deserialize_color};

pub trait Material {}

#[derive(Deserialize)]
struct MaterialSolid {
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
    phong: Phong,
    reflectance: Reflectance,
    transmittance: Transmittance,
    refraction: Refraction,
}

impl Material for MaterialSolid {}

#[derive(Deserialize)]
struct MaterialTextured {
    texture: Texture,
    phong: Phong,
    reflectance: Reflectance,
    transmittance: Transmittance,
    refraction: Refraction,
}

impl Material for MaterialTextured {}

#[derive(Deserialize)]
struct Phong {
    #[serde(rename = "@ka")]
    ka: f32,
    #[serde(rename = "@kd")]
    kd: f32,
    #[serde(rename = "@ks")]
    ks: f32,
    #[serde(rename = "@exponent")]
    exponent: f32,
}

#[derive(Deserialize)]
struct Reflectance {
    #[serde(rename = "@r")]
    r: f32,
}

#[derive(Deserialize)]
struct Transmittance {
    #[serde(rename = "@t")]
    t: f32,
}

#[derive(Deserialize)]
struct Refraction {
    #[serde(rename = "@iof")]
    iof: f32,
}

#[derive(Deserialize)]
struct Texture {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Deserialize)]
enum MaterialEnum {
    #[serde(rename = "material_solid")]
    Solid(MaterialSolid),
    #[serde(rename = "material_textured")]
    Textured(MaterialTextured),
}

pub fn deserialize_material<'de, D>(deserializer: D) -> Result<Rc<dyn Material>, D::Error>
where
    D: Deserializer<'de>,
{
    let material: MaterialEnum = Deserialize::deserialize(deserializer)?;
    match material {
        MaterialEnum::Solid(material_solid) => Ok(Rc::new(material_solid) as Rc<dyn Material>),
        MaterialEnum::Textured(material_textured) => {
            Ok(Rc::new(material_textured) as Rc<dyn Material>)
        }
    }
}
