use std::rc::Rc;

use crate::{
    scene::material::{Material, MaterialSolid},
    utils::vec3::{Point, Vec3},
};

use super::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
    pub texture_coordinate: Option<Vec3>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = outward_normal.clone(); /*if self.front_face {
                                                  outward_normal.clone()
                                              } else {
                                                  -outward_normal
                                              }*/
    }

    pub fn set_texture_coordinate(&mut self, texture_coordinate: &Vec3) {
        self.texture_coordinate = Some(texture_coordinate.clone());
    }

    pub fn new() -> Self {
        Self {
            point: Point::from_values(0.0, 0.0, 0.0),
            normal: Vec3::from_values(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
            material: Rc::new(MaterialSolid::new()),
            texture_coordinate: None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}
