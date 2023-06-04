use crate::{
    hittable::{HitRecord, Hittable},
    vec3::{Point, Vec3},
};

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn from_values(point: &Point, radius: &f32) -> Self {
        Self {
            center: *point,
            radius: *radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_discriminant = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (&(&hit_record.point - &self.center)) / &self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        return true;
    }
}
