use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Camera {
    pub position: Point,
    pub lookat: Vec3,
    pub up: Vec3,
    pub horizontal_fov: f32,
    pub resolution_horizontal: u32,
    pub resolution_vertical: u32,
    pub max_bounces: u32,

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
        resolution_horizontal: u32,
        resolution_vertical: u32,
        max_bounces: u32,
    ) -> Self {
        Self {
            position,
            lookat,
            up,
            horizontal_fov,
            resolution_horizontal,
            resolution_vertical,
            max_bounces,
            //Todo replace with real values
            horizontal: Vec3::from_values(0.0, 0.0, 0.0),
            vertical: Vec3::from_values(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::from_values(0.0, 0.0, 0.0),
        }
    }

    /*pub fn basic_camera(aspect_ratio: f32) -> Self {
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
            lower_left_corner: &position
                - &(&(&horizontal / &2.0)
                    - &(&(&vertical / &2.0) - &Vec3::from_values(0.0, 0.0, focal_length))),
        }
    }

    pub fn construct_ray(&self, i: u32, j: u32) -> Ray {
        // Testing purposes
        //Todo replace with real camera values
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const IMAGE_WIDTH: u32 = 400;

        // Just temporarily, needs to be changed!
        const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

        let u = (f64::from(i) / f64::from(IMAGE_WIDTH - 1)) as f32;
        let v = (f64::from(j) / f64::from(IMAGE_HEIGHT - 1)) as f32;

        Ray::from_values(
            &self.position,
            &(&self.lower_left_corner
                + &(&(&u * &self.horizontal) + &(&(&v * &self.vertical) - &self.position))),
        )
    }*/
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
            lower_left_corner: &(&(&(&position - &(&horizontal / &2.0))
                - &(&horizontal / &2.0))
                - &(&vertical / &2.0))
                - &Vec3::from_values(0.0, 0.0, focal_length)
        }
    }

    pub fn construct_ray(&self, i: u32, j: u32) -> Ray {
        // Testing purposes
        //Todo replace with real camera values
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const IMAGE_WIDTH: u32 = 400;//256;

        // Just temporarily, needs to be changed!
        const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

        let u = (f64::from(i) / f64::from(IMAGE_WIDTH - 1)) as f32;
        let v = (f64::from(j) / f64::from(IMAGE_HEIGHT - 1)) as f32;

        Ray::from_values(
            &self.position,
            &(&self.lower_left_corner
                + &(&(&u * &self.horizontal) + &(&(&v * &self.vertical) - &self.position))),
        )
    }
}
