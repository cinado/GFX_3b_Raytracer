pub mod file_loader;
mod camera;
pub mod color_utility;
pub mod png_creator;
mod ray;
mod vec3;
mod sphere;
pub mod hittable;
mod hittable_list;
pub mod deserialization_helpers;
pub mod light;
mod scene;
pub mod material;

use std::{f32::INFINITY, rc::Rc};

use camera::Camera;
use hittable::{Hittable, HitRecord};
use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use scene::Scene;
use vec3::{Color, Point, Vec3};

use crate::{hittable_list::HittableList, sphere::Sphere};

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    /*let t = hit_sphere(&Point::from_values(0.0, 0.0, -1.0), &0.5, &ray);
    if t > 0.0 {
        let normal = Vec3::unit_vector(&(&ray.at(t) - &Vec3::from_values(0.0, 0.0, -1.0)));
        return &0.5 * &Color::from_values(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }*/
    let mut hit_record = HitRecord::new();
    //let white = Color::from_values(1.0, 1.0, 1.0);
    //let blue = Color::from_values(0.5, 0.7, 1.0);

    if scene.surfaces.hit(ray, 0.0, INFINITY, &mut hit_record) {
        //return &0.5 * &(&hit_record.normal + &white)
        return hit_record.material.get_color()
    }
    scene.background_color
    /*let normalized_direction_vector = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (normalized_direction_vector.y() + 1.0);
    
    &(&(1.0 - t) * &white) + &(&t * &blue)*/
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
    let scene = file_loader::load_and_deserialize_scene().expect("Failed to create scene!");

    let mut image_data = vec![];

    let progress_bar = ProgressBar::new((scene.camera.resolution_horizontal * scene.camera.resolution_vertical).try_into().unwrap());
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {percent}% ({eta})")
            .expect("Failed to create progress style for progress bar"),
    );

    for j in (0..scene.camera.resolution_vertical).rev() {
        for i in 0..scene.camera.resolution_horizontal {
            let ray = scene.camera.construct_ray(i as f64, j as f64);
            let color: Color = ray_color(&ray, &scene);
            //print!("{}\n", color);
            color_utility::to_png_color(&color, &mut image_data);
            progress_bar.inc(1);
        }
    }

    png_creator::create_png_at_path(&image_data, &scene).err();
}
