use std::ops::{AddAssign, MulAssign};

use serde::Deserialize;

use crate::{
    tracer::{hittable::HitRecord, hittable_list::HittableList, ray::Ray},
    utils::{
        deserialization_helpers::{deserialize_color, deserialize_point, deserialize_vector},
        vec3::{Color, Point, Vec3},
    },
};

use super::material::{Phong, Texture};

pub struct LightList {
    pub light_list: Vec<Box<dyn Light>>,
}

pub struct LightIntensity {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
}

impl LightIntensity {
    pub fn new() -> Self {
        Self {
            ambient: Color::new(),
            diffuse: Color::new(),
            specular: Color::new(),
        }
    }
}

impl AddAssign<&LightIntensity> for LightIntensity {
    fn add_assign(&mut self, rhs: &Self) {
        self.ambient += &rhs.ambient;
        self.diffuse += &rhs.diffuse;
        self.specular += &rhs.specular;
    }
}

impl MulAssign<&Phong> for LightIntensity {
    fn mul_assign(&mut self, rhs: &Phong) {
        self.ambient *= &rhs.ka;
        self.diffuse *= &rhs.kd;
        self.specular *= &rhs.ks;
    }
}

impl MulAssign<&f32> for LightIntensity {
    fn mul_assign(&mut self, rhs: &f32) {
        self.ambient *= &rhs;
        self.diffuse *= &rhs;
        self.specular *= &rhs;
    }
}

fn get_color_from_textures(texture_information: &Texture, texture_coordinate: &Vec3) -> Color {
    let antialiasing_enabled = true;
    if antialiasing_enabled {
        let x: f32 = texture_coordinate.x();
        let y = texture_coordinate.y();

        let x_floor = f32::floor(x * texture_information.width as f32) as isize;
        let y_floor = f32::floor(y * texture_information.height as f32) as isize;

        let x_frac = x * texture_information.width as f32 - x_floor as f32;
        let y_frac = y * texture_information.height as f32 - y_floor as f32;

        let x1 = (x_floor as usize).rem_euclid(texture_information.width as usize);
        let y1 = (y_floor as usize).rem_euclid(texture_information.height as usize);

        let x2 = ((x_floor + 1) as usize).rem_euclid(texture_information.width as usize);
        let y2 = ((y_floor + 1) as usize).rem_euclid(texture_information.height as usize);

        let color00 =
            texture_information.texture_pixels[texture_information.width as usize * y1 + x1];
        let color10 =
            texture_information.texture_pixels[texture_information.width as usize * y1 + x2];
        let color01 =
            texture_information.texture_pixels[texture_information.width as usize * y2 + x1];
        let color11 =
            texture_information.texture_pixels[texture_information.width as usize * y2 + x2];

        let color_top = &(&color00 * &(1.0 - x_frac)) + &(&color10 * &x_frac);
        let color_bottom = &(&color01 * &(1.0 - x_frac)) + &(&color11 * &x_frac);

        &(&color_top * &(1.0 - y_frac)) + &(&color_bottom * &y_frac)
    } else {
        // Former implementation without anti-aliasing and interpolation
        let x_converted = (f32::floor(texture_coordinate.x() * texture_information.width) as isize)
            .rem_euclid(texture_information.width as isize) as usize;
        let y_converted = (f32::floor(texture_coordinate.y() * texture_information.height) as isize)
            .rem_euclid(texture_information.height as isize) as usize;

        texture_information.texture_pixels
            [(texture_information.width as usize * y_converted) + x_converted]
    }
}

pub trait Light {
    fn calculate_light_intensities(&self, ray: &Ray, hit_record: &HitRecord) -> LightIntensity;
    fn check_if_in_shadow(&self, hit_record: &HitRecord, surfaces: &HittableList) -> bool;
}

#[derive(Deserialize)]
pub struct AmbientLight {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Color,
}

impl Light for AmbientLight {
    fn calculate_light_intensities(&self, _ray: &Ray, hit_record: &HitRecord) -> LightIntensity {
        let mut light_intesity = LightIntensity::new();

        let color = if hit_record.material.get_texture_information().is_some() {
            get_color_from_textures(
                &hit_record.material.get_texture_information().unwrap(),
                &hit_record.texture_coordinate.unwrap(),
            )
        } else {
            hit_record.material.get_color()
        };

        light_intesity.ambient = &color * &self.color;
        light_intesity
    }

    fn check_if_in_shadow(&self, _hit_record: &HitRecord, _sufaces: &HittableList) -> bool {
        // Ambient light is not affected by shadows
        false
    }
}

impl Default for LightList {
    fn default() -> Self {
        Self {
            light_list: Default::default(),
        }
    }
}

impl LightList {
    pub fn calculate_final_color(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        surfaces: &HittableList,
    ) -> Color {
        let mut light_intensity = LightIntensity::new();

        for light_source in &self.light_list {
            if light_source.check_if_in_shadow(&hit_record, &surfaces) {
                continue;
            }

            light_intensity += &light_source.calculate_light_intensities(&ray, &hit_record);
        }

        light_intensity *= &hit_record.material.get_phong();

        &(&light_intensity.ambient + &light_intensity.diffuse) + &light_intensity.specular
    }
}

#[derive(Deserialize)]
struct ParallelLight {
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
    #[serde(deserialize_with = "deserialize_vector")]
    direction: Vec3,
}

impl Light for ParallelLight {
    fn calculate_light_intensities(&self, ray: &Ray, hit_record: &HitRecord) -> LightIntensity {
        let mut light_intensity = LightIntensity::new();

        let color = if hit_record.material.get_texture_information().is_some() {
            get_color_from_textures(
                &hit_record.material.get_texture_information().unwrap(),
                &hit_record.texture_coordinate.unwrap(),
            )
        } else {
            hit_record.material.get_color()
        };

        // Calculate Diffuse
        let light_vector = (-&self.direction).unit_vector();
        let intensity = hit_record.normal.dot(&light_vector).max(0.0);
        let diffuse_intensity = &(&color * &self.color) * &intensity;

        // Calculate Specular
        //r = 2(n ⋅ l)n – l
        let reflection_vector = (&(&(&2.0 * &(hit_record.normal.dot(&light_vector)))
            * &hit_record.normal)
            - &light_vector)
            .unit_vector();

        let eye_vector = -&ray.direction.unit_vector();
        let specular_intensity = &(eye_vector
            .dot(&reflection_vector)
            .max(0.0)
            .powf(hit_record.material.get_phong().exponent))
            * &self.color;
        light_intensity.diffuse = diffuse_intensity;
        light_intensity.specular = specular_intensity;
        light_intensity
    }

    fn check_if_in_shadow(&self, hit_record: &HitRecord, surfaces: &HittableList) -> bool {
        //Check if light_vector intersects with any object
        let light_vector = (-&self.direction).unit_vector();
        surfaces.shadow_check(
            &Ray {
                origin: hit_record.point,
                direction: light_vector,
            },
            0.00001, // offset prevent intersection with object itself
            f32::INFINITY,
        )
    }
}

#[derive(Deserialize)]
struct PointLight {
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
    #[serde(deserialize_with = "deserialize_point")]
    position: Point,
}

impl Light for PointLight {
    fn calculate_light_intensities(&self, ray: &Ray, hit_record: &HitRecord) -> LightIntensity {
        let mut light_intensity = LightIntensity::new();

        let color = if hit_record.material.get_texture_information().is_some() {
            get_color_from_textures(
                &hit_record.material.get_texture_information().unwrap(),
                &hit_record.texture_coordinate.unwrap(),
            )
        } else {
            hit_record.material.get_color()
        };

        // Calculate Diffuse
        let light_vector = (&self.position - &hit_record.point).unit_vector();
        let intensity = hit_record.normal.dot(&light_vector).max(0.0);
        let diffuse_intensity = &(&color * &self.color) * &intensity;

        // Calculate Specular
        //r = 2(n ⋅ l)n – l
        let reflection_vector = (&(&(&2.0 * &(hit_record.normal.dot(&light_vector)))
            * &hit_record.normal)
            - &light_vector)
            .unit_vector();

        let eye_vector = -&ray.direction.unit_vector();
        let specular_intensity = &(eye_vector
            .dot(&reflection_vector)
            .max(0.0)
            .powf(hit_record.material.get_phong().exponent))
            * &self.color;
        light_intensity.diffuse = diffuse_intensity;
        light_intensity.specular = specular_intensity;
        light_intensity
    }

    fn check_if_in_shadow(&self, hit_record: &HitRecord, surfaces: &HittableList) -> bool {
        let mut light_vector = &self.position - &hit_record.point;
        let light_vector_length = light_vector.length();
        light_vector = light_vector.unit_vector();
        surfaces.shadow_check(
            &Ray {
                origin: hit_record.point,
                direction: light_vector,
            },
            0.00001, // offset prevent intersection with object itself
            light_vector_length,
        )
    }
}

#[derive(Deserialize)]
struct SpotLight {
    #[serde(deserialize_with = "deserialize_color")]
    color: Color,
    #[serde(deserialize_with = "deserialize_point")]
    position: Point,
    #[serde(deserialize_with = "deserialize_vector")]
    direction: Vec3,
    #[serde(rename = "falloff")]
    fall_off: FallOff,
}

impl Light for SpotLight {
    fn calculate_light_intensities(&self, ray: &Ray, hit_record: &HitRecord) -> LightIntensity {
        let mut light_intensity = LightIntensity::new();

        let color = if hit_record.material.get_texture_information().is_some() {
            get_color_from_textures(
                &hit_record.material.get_texture_information().unwrap(),
                &hit_record.texture_coordinate.unwrap(),
            )
        } else {
            hit_record.material.get_color()
        };

        let incident_light_direction = (-&(&self.position - &hit_record.point)).unit_vector();
        let incident_angle = incident_light_direction
            .dot(&self.direction.unit_vector())
            .acos();

        if incident_angle > self.fall_off.alpha2.to_radians() {
            return light_intensity;
        } else if incident_angle >= 0.0 && incident_angle <= self.fall_off.alpha1.to_radians() {
            // with angle between zero and angle1, the light should be just like a point light

            // Calculate Diffuse
            let light_vector = (&self.position - &hit_record.point).unit_vector();
            let intensity = hit_record.normal.dot(&light_vector).max(0.0);
            let diffuse_intensity = &(&color * &self.color) * &intensity;

            // Calculate Specular
            //r = 2(n ⋅ l)n – l
            let reflection_vector = (&(&(&2.0 * &(hit_record.normal.dot(&light_vector)))
                * &hit_record.normal)
                - &light_vector)
                .unit_vector();

            let eye_vector = -&ray.direction.unit_vector();
            let specular_intensity = &(eye_vector
                .dot(&reflection_vector)
                .max(0.0)
                .powf(hit_record.material.get_phong().exponent))
                * &self.color;
            light_intensity.diffuse = diffuse_intensity;
            light_intensity.specular = specular_intensity;
            return light_intensity;
        } else {
            let light_vector = (&self.position - &hit_record.point).unit_vector();
            let intensity = hit_record.normal.dot(&light_vector).max(0.0);
            let diffuse_intensity = &(&color * &self.color) * &intensity;

            // Calculate Specular
            //r = 2(n ⋅ l)n – l
            let reflection_vector = (&(&(&2.0 * &(hit_record.normal.dot(&light_vector)))
                * &hit_record.normal)
                - &light_vector)
                .unit_vector();

            let eye_vector = -&ray.direction.unit_vector();
            let specular_intensity = &(eye_vector
                .dot(&reflection_vector)
                .max(0.0)
                .powf(hit_record.material.get_phong().exponent))
                * &self.color;
            light_intensity.diffuse = diffuse_intensity;
            light_intensity.specular = specular_intensity;

            let interpolation_factor = 1.0
                - ((incident_angle - self.fall_off.alpha1.to_radians())
                    / (self.fall_off.alpha2.to_radians() - self.fall_off.alpha1.to_radians()))
                .clamp(0.0, 1.0);

            light_intensity *= &interpolation_factor;
            light_intensity
        }
    }

    fn check_if_in_shadow(&self, hit_record: &HitRecord, surfaces: &HittableList) -> bool {
        let light_vector = (&self.position - &hit_record.point).unit_vector();
        surfaces.shadow_check(
            &Ray {
                origin: hit_record.point,
                direction: light_vector,
            },
            0.00001, // offset prevent intersection with object itself
            f32::INFINITY,
        )
    }
}

#[derive(Deserialize)]
struct FallOff {
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
        #[serde(rename = "falloff")]
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
                    LightEnum::Parallel { color, direction } => {
                        Box::new(ParallelLight { color, direction })
                    }
                    LightEnum::Point { color, position } => {
                        Box::new(PointLight { color, position })
                    }
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
