mod scene;
mod tracer;
mod utils;

use std::f32::INFINITY;

use indicatif::{ProgressBar, ProgressStyle};
use scene::scene::Scene;
use tracer::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use utils::{color_utility, file_loader, png_creator, vec3::{Color, Vec3}};

fn ray_color(ray: &Ray, scene: &Scene, current_bounce_number: usize) -> Color {
    let mut hit_record = HitRecord::new();
    let mut color = Color::new();
    let mut reflected_color = Color::new();

    if scene.surfaces.hit(ray, 0.0, INFINITY, &mut hit_record) {
        color = scene
            .lights
            .calculate_final_color(&ray, &hit_record, &scene.surfaces);

        if current_bounce_number > scene.camera.max_bounces {
            return color;
        }

        if hit_record.material.get_reflectance().r > 0.00001 {
            let bias = 0.001; // Small bias value to mitigate surface acne
            let reflected_direction = reflect(&ray.direction, &hit_record.normal);
            let reflected_origin = &hit_record.point + &(&bias * &reflected_direction); // Apply bias to the origin
            let reflected_ray = Ray {
                origin: reflected_origin,
                direction: reflected_direction,
            };

            reflected_color = &hit_record.material.get_reflectance().r
                * &ray_color(&reflected_ray, scene, current_bounce_number + 1);
        }

        if(hit_record.material.get_transmittance().t > 0.00001){
            
        }

        return &(&color * &(1.0 - hit_record.material.get_reflectance().r)) + &reflected_color;
    }

    scene.background_color
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - &(&(2.0 * incident.dot(normal)) * normal)
}

fn main() {
    let scene = file_loader::load_and_deserialize_scene();

    let mut image_data = vec![];

    let progress_bar = ProgressBar::new(
        (scene.camera.resolution_horizontal * scene.camera.resolution_vertical)
            .try_into()
            .unwrap(),
    );
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {percent}% ({eta})")
            .expect("Failed to create progress style for progress bar"),
    );

    for j in (0..scene.camera.resolution_vertical).rev() {
        for i in 0..scene.camera.resolution_horizontal {
            let ray = scene.camera.construct_ray(i as f64, j as f64);
            let color: Color = ray_color(&ray, &scene, 0);
            color_utility::to_png_color(&color, &mut image_data);
            progress_bar.inc(1);
        }
    }

    png_creator::create_png_at_path(&image_data, &scene);
}
