pub mod file_loader;
mod camera;
pub mod color_utility;
pub mod png_creator;
mod ray;
mod vec3;
mod surfaces;
pub mod hittable;
mod hittable_list;
pub mod deserialization_helpers;
pub mod light;
mod scene;
pub mod material;

use std::{f32::INFINITY};

use hittable::{Hittable, HitRecord};
use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use scene::Scene;
use vec3::{Color};

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    let mut hit_record = HitRecord::new();

    if scene.surfaces.hit(ray, 0.0, INFINITY, &mut hit_record) {
        return scene.lights.calculate_final_color(&ray, &hit_record, &scene.surfaces)
    }
    scene.background_color
}

fn main() {
    let scene = file_loader::load_and_deserialize_scene();

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
            color_utility::to_png_color(&color, &mut image_data);
            progress_bar.inc(1);
        }
    }

    png_creator::create_png_at_path(&image_data, &scene);
}
