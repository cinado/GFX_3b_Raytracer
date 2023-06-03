//mod file_loader;

pub mod color_utility;
pub mod png_creator;
mod vec3;

use indicatif::{ProgressBar, ProgressStyle};
use vec3::Color;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut image_data = vec![];
    let progress_bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {percent}% ({eta})")
            .expect("Failed to create progress style for progress bar"),
    );

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let color: Color = Color::form_values(
                (f64::from(i) / f64::from(IMAGE_WIDTH - 1)) as f32,
                (f64::from(j) / f64::from(IMAGE_HEIGHT - 1)) as f32,
                0.25,
            );
            color_utility::to_png_color(&color, &mut image_data);
            progress_bar.inc(1);
        }
    }

    png_creator::create_png_at_path(&image_data, &String::from("test1.png")).err();
}
