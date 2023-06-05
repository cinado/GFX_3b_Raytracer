use serde::Deserialize;

use crate::{deserialization_helpers::{deserialize_color, deserialize_point, deserialize_vector}, vec3::{Color, Point, Vec3}};


pub struct LightList{
    pub light_list: Vec<Box<dyn Light>>
}

pub trait Light{

}

#[derive(Deserialize)]
struct AmbientLight{
    #[serde(deserialize_with="deserialize_color")]
    color: Color,
}

impl Light for AmbientLight{

}

impl Default for LightList{
    fn default() -> Self {
        Self { light_list: Default::default() }
    }
}

#[derive(Deserialize)]
struct ParallelLight{
    #[serde(deserialize_with="deserialize_color")]
    color: Color,
    #[serde(deserialize_with="deserialize_vector")]
    direction: Vec3,
}

impl Light for ParallelLight{

}

#[derive(Deserialize)]
struct PointLight{
    #[serde(deserialize_with="deserialize_color")]
    color: Color,
    #[serde(deserialize_with="deserialize_point")]
    position: Point,
}

impl Light for PointLight{
    
}

#[derive(Deserialize)]
struct SpotLight{
    #[serde(deserialize_with="deserialize_color")]
    color: Color,
    #[serde(deserialize_with="deserialize_point")]
    position: Point,
    #[serde(deserialize_with="deserialize_vector")]
    direction: Vec3,
    fall_off: FallOff,
}

impl Light for SpotLight{
    
}

#[derive(Deserialize)]
struct FallOff{
    #[serde(rename = "@alpha1")]
    alpha1: f32,
    #[serde(rename = "@alpha2")]
    alpha2: f32,
}

#[derive(Deserialize)]
enum LightEnum {
    #[serde(rename = "ambient_light")]
    Ambient {
        #[serde(deserialize_with = "deserialize_color")]
        color: Color,
    },
    #[serde(rename = "parallel_light")]
    Parallel {
        #[serde(deserialize_with = "deserialize_color")]
        color: Color,
        #[serde(deserialize_with = "deserialize_vector")]
        direction: Vec3,
    },
    #[serde(rename = "point_light")]
    Point {
        #[serde(deserialize_with = "deserialize_color")]
        color: Color,
        #[serde(deserialize_with = "deserialize_point")]
        position: Point,
    },
    #[serde(rename = "spot_light")]
    Spot {
        #[serde(deserialize_with = "deserialize_color")]
        color: Color,
        #[serde(deserialize_with = "deserialize_point")]
        position: Point,
        #[serde(deserialize_with = "deserialize_vector")]
        direction: Vec3,
        fall_off: FallOff,
    },
}

impl<'de> Deserialize<'de> for LightList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerLights {
            #[serde(rename = "$value")]
            lights: Vec<LightEnum>,
        }

        let inner_lights: InnerLights = InnerLights::deserialize(deserializer)?;

        let light_list: Vec<Box<dyn Light>> = inner_lights
            .lights
            .into_iter()
            .map(|light_type| -> Box<dyn Light> {
                match light_type {
                    LightEnum::Ambient { color } => Box::new(AmbientLight { color }),
                    LightEnum::Parallel { color, direction } => Box::new(ParallelLight {
                        color,
                        direction,
                    }),
                    LightEnum::Point { color, position } => Box::new(PointLight {
                        color,
                        position,
                    }),
                    LightEnum::Spot {
                        color,
                        position,
                        direction,
                        fall_off,
                    } => Box::new(SpotLight {
                        color,
                        position,
                        direction,
                        fall_off,
                    }),
                }
            })
            .collect();

        Ok(LightList { light_list })
    }
}