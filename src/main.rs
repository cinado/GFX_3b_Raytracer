//mod file_loader;

pub mod color_utility;
pub mod png_creator;
mod ray;
mod vec3;
mod camera;

use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use vec3::{Color, Vec3};
use camera::Camera;


fn ray_color(ray: &Ray) -> Color {
    let normalized_direction_vector = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (normalized_direction_vector.y() + 1.0);
    return &(&(1.0 - t) * &Color::from_values(1.0, 1.0, 1.0))
        + &(&t * &Color::from_values(0.5, 0.7, 1.0));
}

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;

    // Just temporarily, needs to be changed!
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 /ASPECT_RATIO) as u32;

    let camera = Camera::basic_camera(ASPECT_RATIO);

    let mut image_data = vec![];
    let progress_bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {percent}% ({eta})")
            .expect("Failed to create progress style for progress bar"),
    );

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let ray = camera.construct_ray(i, j);
            let color: Color = ray_color(&ray);
            /*let color: Color = Color::from_values(
                (f64::from(i) / f64::from(IMAGE_WIDTH - 1)) as f32,
                (f64::from(j) / f64::from(IMAGE_HEIGHT - 1)) as f32,
                0.25,
            );*/
            color_utility::to_png_color(&color, &mut image_data);
            progress_bar.inc(1);
        }
    }

    png_creator::create_png_at_path(&image_data, &String::from("test2_gradient.png")).err();
}
