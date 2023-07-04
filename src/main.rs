mod scene;
mod tracer;
mod utils;

use std::{env, f32::INFINITY};

use indicatif::{ProgressBar, ProgressStyle};
use scene::scene::Scene;
use tracer::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use utils::{
    color_utility, file_loader, png_creator,
    vec3::{Color, Vec3},
};

use rand::distributions::Distribution;
use rand::distributions::Uniform;

fn ray_color(ray: &Ray, scene: &Scene, current_bounce_number: usize) -> Color {
    let mut hit_record = HitRecord::new();
    let mut color = Color::new();
    let mut reflected_color = Color::new();
    let mut refracted_color = Color::new();

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

        if hit_record.material.get_transmittance().t > 0.00001 {
            let bias = 0.001; // Small bias value to mitigate surface acne
            let refracted_direction = refract(&ray.direction, &hit_record.normal, &hit_record);
            let refracted_origin = &hit_record.point + &(&bias * &refracted_direction); // Apply bias to the origin
            let refracted_ray = Ray {
                origin: refracted_origin,
                direction: refracted_direction,
            };

            refracted_color = &hit_record.material.get_transmittance().t
                * &ray_color(&refracted_ray, scene, current_bounce_number + 1);
        }

        return &(&color
            * &(1.0
                - hit_record.material.get_reflectance().r
                - hit_record.material.get_transmittance().t))
            + &(&reflected_color + &refracted_color);
    }

    scene.background_color
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - &(&(2.0 * incident.dot(normal)) * normal)
}

fn refract(incident: &Vec3, normal: &Vec3, hit_record: &HitRecord) -> Vec3 {
    let incident_normalized = incident.unit_vector();
    let mut normal_normalized = normal.unit_vector();

    let mut cosine = incident_normalized.dot(&normal_normalized);
    cosine = cosine.clamp(-1.0, 1.0);

    let eta = if cosine < 0.0 {
        cosine = -cosine;
        1.0 / hit_record.material.get_refraction().iof
    } else {
        normal_normalized = -&normal_normalized;
        hit_record.material.get_refraction().iof
    };

    let pre_sqrt_check = 1.0 - eta.powi(2) * (1.0 - cosine.powi(2));

    if pre_sqrt_check < 0.0 {
        return reflect(&incident, &normal);
    }

    /*(&(&eta * &(&incident_normalized + &(&normal_normalized * &cosine)))
    - &(&normal_normalized * &f32::sqrt(pre_sqrt_check))).unit_vector()*/

    &(&eta * &incident_normalized)
        + &(&(eta * cosine - f32::sqrt(pre_sqrt_check)) * &normal_normalized)
}

fn main() {
    let mut sample_size: Option<usize> = None;

    for arg in env::args() {
        if let Some(value) = arg.strip_prefix("-s=") {
            if let Ok(size) = value.parse::<usize>() {
                sample_size = Some(size);
            }
        }
    }

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

    if sample_size.is_none() {
        for j in (0..scene.camera.resolution_vertical).rev() {
            for i in 0..scene.camera.resolution_horizontal {
                let ray = scene.camera.construct_ray(i as f64, j as f64);
                let color: Color = ray_color(&ray, &scene, 0);
                color_utility::to_png_color(&color, &mut image_data, 1.0);
                progress_bar.inc(1);
            }
        }
    } else {
        let mut rng = rand::thread_rng();
        let uniform_sampler = Uniform::from(-0.5..=0.5);

        for j in (0..scene.camera.resolution_vertical).rev() {
            for i in 0..scene.camera.resolution_horizontal {
                let mut color = Color::new();
                for _ in 0..sample_size.unwrap() {
                    let ray = scene.camera.construct_ray(
                        i as f64 + uniform_sampler.sample(&mut rng),
                        j as f64 + uniform_sampler.sample(&mut rng),
                    );
                    color += &ray_color(&ray, &scene, 0);
                }
                color_utility::to_png_color(
                    &color,
                    &mut image_data,
                    sample_size.unwrap() as f32,
                );
                progress_bar.inc(1);
            }
        }
    }
    png_creator::create_png_at_path(&image_data, &scene);
}
