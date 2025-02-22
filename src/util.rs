use crate::vec3::Vec3;
use image::Rgb;

// Assumes [0,1] input
pub fn write_color(buf: &mut String, color: Vec3) {
    let r: i64 = (255.0 * liner_to_gamma(color.x())).trunc() as i64;
    let g: i64 = (255.0 * liner_to_gamma(color.y())).trunc() as i64;
    let b: i64 = (255.0 * liner_to_gamma(color.z())).trunc() as i64;
    buf.push_str(format!("{} {} {} ", r, g, b).as_str());
}

pub fn process_rgb(color: Vec3) -> Rgb<u8> {
    let r = (255.0 * liner_to_gamma(color.x())).trunc() as u8;
    let g = (255.0 * liner_to_gamma(color.y())).trunc() as u8;
    let b = (255.0 * liner_to_gamma(color.z())).trunc() as u8;

    return Rgb([r, g, b]);
}

pub fn write_new_line(buf: &mut String) {
    buf.push_str("\n");
}

pub fn liner_to_gamma(x: f64) -> f64 {
    return if x > 0.0 { x.sqrt() } else { 0.0 };
}
