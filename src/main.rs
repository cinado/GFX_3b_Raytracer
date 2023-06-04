//mod file_loader;

mod camera;
pub mod color_utility;
pub mod png_creator;
mod ray;
mod vec3;
mod sphere;
pub mod hittable;
mod hittable_list;

use std::{f32::INFINITY, rc::Rc};

use camera::Camera;
use hittable::{Hittable, HitRecord};
use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use vec3::{Color, Point, Vec3};

use crate::{hittable_list::HittableList, sphere::Sphere};

fn ray_color(ray: &Ray, hittable: &dyn Hittable) -> Color {
    /*let t = hit_sphere(&Point::from_values(0.0, 0.0, -1.0), &0.5, &ray);
    if t > 0.0 {
        let normal = Vec3::unit_vector(&(&ray.at(t) - &Vec3::from_values(0.0, 0.0, -1.0)));
        return &0.5 * &Color::from_values(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }*/
    let mut hit_record = HitRecord::new();
    let white = Color::from_values(1.0, 1.0, 1.0);
    let blue = Color::from_values(0.5, 0.7, 1.0);

    if hittable.hit(ray, 0.0, INFINITY, &mut hit_record) {
        return &0.5 * &(&hit_record.normal + &white)
    }

    let normalized_direction_vector = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (normalized_direction_vector.y() + 1.0);
    
    &(&(1.0 - t) * &white) + &(&t * &blue)
}

/*fn hit_sphere(center: &Point, radius: &f32, ray: &Ray) -> f32 {
    let oc = &ray.origin - &center;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f32::sqrt(discriminant)) / a
    }
}*/

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;

    // Just temporarily, needs to be changed!
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    let camera = Camera::basic_camera(ASPECT_RATIO);

    let mut image_data = vec![];
    let progress_bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {percent}% ({eta})")
            .expect("Failed to create progress style for progress bar"),
    );

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from_values(&Point::from_values(0.0,0.0,-1.0), &0.5)));
    world.add(Rc::new(Sphere::from_values(&Point::from_values(0.0,-100.5,-1.0), &100.0)));

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let ray = camera.construct_ray(i, j);
            let color: Color = ray_color(&ray, &world);
            //print!("{}\n", color);
            color_utility::to_png_color(&color, &mut image_data);
            progress_bar.inc(1);
        }
    }

    png_creator::create_png_at_path(&image_data, &String::from("test4.png")).err();
}
