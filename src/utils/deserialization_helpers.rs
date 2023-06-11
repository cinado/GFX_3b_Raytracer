use serde::Deserialize;

use crate::utils::vec3::{Color, Point, Vec3};

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct ColorDeserialized {
        #[serde(rename = "@r")]
        pub r: f32,
        #[serde(rename = "@g")]
        pub g: f32,
        #[serde(rename = "@b")]
        pub b: f32,
    }

    let color = ColorDeserialized::deserialize(deserializer)?;
    Ok(Color::from_values(color.r, color.g, color.b))
}

pub fn deserialize_point<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct PointDeserialized {
        #[serde(rename = "@x")]
        pub x: f32,
        #[serde(rename = "@y")]
        pub y: f32,
        #[serde(rename = "@z")]
        pub z: f32,
    }

    let point = PointDeserialized::deserialize(deserializer)?;
    Ok(Point::from_values(point.x, point.y, point.z))
}

pub fn deserialize_vector<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct VectorDeserialized {
        #[serde(rename = "@x")]
        pub x: f32,
        #[serde(rename = "@y")]
        pub y: f32,
        #[serde(rename = "@z")]
        pub z: f32,
    }

    let vector = VectorDeserialized::deserialize(deserializer)?;
    Ok(Vec3::from_values(vector.x, vector.y, vector.z))
}
