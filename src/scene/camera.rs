use serde::Deserialize;

use crate::{
    tracer::ray::Ray,
    utils::{
        deserialization_helpers::{deserialize_point, deserialize_vector},
        vec3::{Point, Vec3},
    },
};

pub struct Camera {
    pub position: Point,
    pub lookat: Vec3,
    pub up: Vec3,
    pub horizontal_fov: f32,
    pub resolution_horizontal: usize,
    pub resolution_vertical: usize,
    pub max_bounces: usize,

    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point,
}

impl Camera {
    pub fn from_values(
        position: Point,
        lookat: Vec3,
        up: Vec3,
        horizontal_fov: f32,
        resolution_horizontal: usize,
        resolution_vertical: usize,
        max_bounces: usize,
    ) -> Self {
        let aspect_ratio = resolution_horizontal as f32 / resolution_vertical as f32;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length: f32 = 1.0;

        let horizontal = Vec3::from_values(viewport_width, 0.0, 0.0);
        let vertical = Vec3::from_values(0.0, viewport_height, 0.0);
        Self {
            position: Point::from_values(position.x(), position.y(), position.z()),
            lookat,
            up,
            horizontal_fov,
            resolution_horizontal,
            resolution_vertical,
            max_bounces,
            horizontal,
            vertical,
            lower_left_corner: &(&(&position - &(&horizontal / &2.0)) - &(&vertical / &2.0))
                - &Vec3::from_values(0.0, 0.0, focal_length),
        }
    }
    /*
    pub fn basic_camera(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length: f32 = 1.0;

        let position = Point::from_values(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::from_values(viewport_width, 0.0, 0.0);
        let vertical = Vec3::from_values(0.0, viewport_height, 0.0);

        Self {
            position,
            lookat: Vec3::from_values(0.0, 0.0, 0.0),
            up: Vec3::from_values(0.0, 1.0, 0.0),
            //---------- Test values
            horizontal_fov: 45.0,
            resolution_horizontal: 256,
            resolution_vertical: 256,
            max_bounces: 0,
            //-------
            horizontal,
            vertical,
            lower_left_corner: &(&(&position - &(&horizontal / &2.0)) - &(&vertical / &2.0))
                - &Vec3::from_values(0.0, 0.0, focal_length),
        }
    }*/

    pub fn construct_ray(&self, i: f64, j: f64) -> Ray {
        let u = (i / f64::from(self.resolution_horizontal as u32 - 1)) as f32;
        let v = (j / f64::from(self.resolution_vertical as u32 - 1)) as f32;

        Ray::from_values(
            &self.position,
            &(&self.lower_left_corner
                + &(&(&u * &self.horizontal) + &(&(&v * &self.vertical) - &self.position))),
        )
    }
}

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DeserializeCamera {
            #[serde(deserialize_with = "deserialize_point")]
            position: Point,
            #[serde(deserialize_with = "deserialize_vector")]
            lookat: Vec3,
            #[serde(deserialize_with = "deserialize_vector")]
            up: Vec3,
            horizontal_fov: HorizontalFov,
            resolution: Resolution,
            max_bounces: MaxBounces,
        }

        #[derive(Deserialize)]
        struct HorizontalFov {
            #[serde(rename = "@angle")]
            angle: f32,
        }

        #[derive(Deserialize)]
        struct MaxBounces {
            #[serde(rename = "@n")]
            n: usize,
        }

        #[derive(Deserialize)]
        struct Resolution {
            #[serde(rename = "@horizontal")]
            resolution_horizontal: usize,
            #[serde(rename = "@vertical")]
            resolution_vertical: usize,
        }

        let deserialized_camera = DeserializeCamera::deserialize(deserializer)?;

        Ok(Camera::from_values(
            deserialized_camera.position,
            deserialized_camera.lookat,
            deserialized_camera.up,
            deserialized_camera.horizontal_fov.angle,
            deserialized_camera.resolution.resolution_horizontal,
            deserialized_camera.resolution.resolution_vertical,
            deserialized_camera.max_bounces.n,
        ))
    }
}
