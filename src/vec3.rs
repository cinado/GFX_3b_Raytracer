use std::ops::{Add, Div, Mul, Sub, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    vector: [f32; 3],
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new() -> Self {
        Self.form_values(0.0, 0.0, 0.0)
    }

    pub fn form_values(x: f32, y: f32, z: f32) -> Self {
        Self { vector: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.vector[0]
    }

    pub fn y(&self) -> f32 {
        self.vector[1]
    }

    pub fn z(&self) -> f32 {
        self.vector[2]
    }

    pub fn r(&self) -> f32 {
        self.x()
    }

    pub fn g(&self) -> f32 {
        self.y()
    }

    pub fn b(&self) -> f32 {
        self.z()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.vector[0] * other.vector[0] + self.vector[1] * other.vector[1] + self.vector[2] * other.vector[2]
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            vector: [
                self.vector[1] * other.vector[2] - self.vector[2] * other.vector[1],
                self.vector[2] * other.vector[0] - self.vector[0] * other.vector[2],
                self.vector[0] * other.vector[1] - self.vector[1] * other.vector[0],
            ],
        }
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        self.div(self.length())
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(&self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            vector: [
                self.vector[0] + rhs.vector[0],
                self.vector[1] + rhs.vector[1],
                self.vector[2] + rhs.vector[2],
            ],
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(&self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            vector: [
                self.vector[0] - rhs.vector[0],
                self.vector[1] - rhs.vector[1],
                self.vector[2] - rhs.vector[2],
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(&self, rhs: &f32) -> Self::Output {
        Self::Output {
            vector: [
                self.vector[0] * rhs,
                self.vector[1] * rhs,
                self.vector[2] * rhs,
            ],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(&self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            vector: [
                self * rhs.vector[0],
                self * rhs.vector[1],
                self * rhs.vector[2],
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(&self) -> Self::Output {
        Self::Output {
            vector: [
                -self.vector[0],
                -self.vector[1],
                -self.vector[2],
            ],
        }
    }
}
