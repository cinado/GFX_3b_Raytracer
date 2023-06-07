use png::{BitDepth, ColorType, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::scene::Scene;

pub(crate) fn create_png_at_path(png_data: &Vec<u8>, scene: &Scene) -> Result<(), std::io::Error> {
    let output_directory = Path::new("output_files");
    let target_location = output_directory.join(&scene.output_file);
    let file = File::create(target_location).expect("Failed to create file");

    let ref mut writer = BufWriter::new(file);
    let mut encoder = Encoder::new(
        writer,
        scene.camera.resolution_horizontal as u32,
        scene.camera.resolution_vertical as u32,
    );
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);

    // Taken from the png rust doc - to ensure the same colors
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);

    let mut writer = encoder.write_header().expect("Failed to write PNG header");

    writer
        .write_image_data(&png_data)
        .expect("Failed to write pixel data");

    writer.finish().expect("Failed to finish writing PNG");
    Ok(())
}
