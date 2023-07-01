use std::ops::{Add, Mul, Sub};

use super::vec3::{Point, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub matrix: [f32; 16],
}

impl Mat4 {
    pub fn new() -> Mat4 {
        Mat4 { matrix: [0.0; 16] }
    }

    pub fn create() -> Mat4 {
        Mat4::identity()
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            matrix: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn construct_camera_transformation_matrix(
        up: &Vec3,
        look_at: &Vec3,
        position: &Point,
    ) -> Mat4 {
        let mut result = Mat4::identity();

        let z_component = &(position - look_at) / &((position - look_at).length());
        let x_component = &(up.cross(&z_component)) / &(up.cross(&z_component).length());
        let y_component =
            &(z_component.cross(&x_component)) / &(z_component.cross(&x_component).length());

        for i in 0..3 {
            result.matrix[i * 4] = x_component[i];
            result.matrix[i * 4 + 1] = y_component[i];
            result.matrix[i * 4 + 2] = z_component[i];
            result.matrix[i * 4 + 3] = position[i];
        }
        result
    }

    pub fn transform_point3(&self, origin: &Point) -> Point {
        let x = origin.x() * self.matrix[0]
            + origin.y() * self.matrix[1]
            + origin.z() * self.matrix[2]
            + self.matrix[3];
        let y = origin.x() * self.matrix[4]
            + origin.y() * self.matrix[5]
            + origin.z() * self.matrix[6]
            + self.matrix[7];
        let z = origin.x() * self.matrix[8]
            + origin.y() * self.matrix[9]
            + origin.z() * self.matrix[10]
            + self.matrix[11];
        Point::from_values(x, y, z)
    }

    pub fn transform_vec3(&self, direction: &Vec3) -> Vec3 {
        let x = direction.x() * self.matrix[0]
            + direction.y() * self.matrix[1]
            + direction.z() * self.matrix[2];
        let y = direction.x() * self.matrix[4]
            + direction.y() * self.matrix[5]
            + direction.z() * self.matrix[6];
        let z = direction.x() * self.matrix[8]
            + direction.y() * self.matrix[9]
            + direction.z() * self.matrix[10];
        Vec3::from_values(x, y, z)
    }
}

impl Add<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn add(self, rhs: &Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..16 {
            result.matrix[i] = self.matrix[i] + rhs.matrix[i];
        }
        result
    }
}

impl Sub<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn sub(self, rhs: &Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..16 {
            result.matrix[i] = self.matrix[i] - rhs.matrix[i];
        }
        result
    }
}

impl Mul<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.matrix[i * 4 + j] += self.matrix[i * 4 + k] * rhs.matrix[k * 4 + j];
                }
            }
        }
        result
    }
}

impl Mul<&f32> for &Mat4 {
    type Output = Mat4;

    fn mul(self, scalar: &f32) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..16 {
            result.matrix[i] = self.matrix[i] * scalar;
        }
        result
    }
}
