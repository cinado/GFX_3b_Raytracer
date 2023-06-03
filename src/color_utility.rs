use crate::vec3::Color;

pub fn to_png_color(color: &Color, pixel_colors: &mut Vec<u8>) {
    let temp_values: [u8; 3] = [
        (color.r() * 255.999) as u8,
        (color.g() * 255.999) as u8,
        (color.b() * 255.999) as u8,
    ];
    pixel_colors.extend(&temp_values);
}
