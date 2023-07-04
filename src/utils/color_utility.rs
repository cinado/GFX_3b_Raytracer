use super::vec3::Color;

pub fn to_png_color(color: &Color, pixel_colors: &mut Vec<u8>, sample_size: f32) {

    let scaling_factor = 1.0 / sample_size;

    let temp_values: [u8; 3] = [
        ((color.r() * scaling_factor).clamp(0.0, 0.999) * 255.999) as u8,
        ((color.g() * scaling_factor).clamp(0.0, 0.999) * 255.999) as u8,
        ((color.b() * scaling_factor).clamp(0.0, 0.999) * 255.999) as u8,
    ];
    pixel_colors.extend(&temp_values);
}
