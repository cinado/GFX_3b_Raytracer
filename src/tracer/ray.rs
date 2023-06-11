use crate::utils::vec3::{Point, Vec3};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn from_values(origin: &Point, direction: &Vec3) -> Self {
        Self {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn at(&self, t: f32) -> Point {
        return &self.origin + &(&t * &self.direction);
    }
}
