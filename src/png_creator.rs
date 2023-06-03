use png::{BitDepth, ColorType, Encoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub(crate) fn create_png_at_path(
    png_data: &Vec<u8>,
    filename: &String,
) -> Result<(), std::io::Error> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let output_directory = Path::new("output_files");
    let target_location = output_directory.join(filename);
    let file = File::create(target_location).expect("Failed to create file");

    let ref mut writer = BufWriter::new(file);
    let mut encoder = Encoder::new(writer, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().expect("Failed to write PNG header");

    writer
        .write_image_data(&png_data)
        .expect("Failed to write pixel data");

    writer.finish().expect("Failed to finish writing PNG");
    Ok(())
}
