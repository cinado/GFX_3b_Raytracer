use std::rc::Rc;

use crate::{
    ray::Ray,
    vec3::{Point, Vec3}, material::Material,
};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        }
    }
    pub fn new() -> Self {
        Self {
            point: Point::from_values(0.0,0.0,0.0),
            normal: Vec3::from_values(0.0,0.0,0.0),
            t: 0.0,
            front_face: true,
            material: Rc::new(crate::material::MaterialSolid::new())
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}
