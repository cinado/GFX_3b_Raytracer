use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_hit_record = HitRecord::new();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit_record) {
                hit_anything = true;
                closest_so_far = temp_hit_record.t;
                *hit_record = temp_hit_record.clone();
            }
        }

        hit_anything
    }
}
