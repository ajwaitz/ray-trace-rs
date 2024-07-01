mod vec3;
mod interval;
mod buffer;
mod camera;
mod world;
mod material;

use camera::Camera;
use world::{HittableList, Sphere};
use material::{Material, Lambertian, Metal};

use vec3::Vec3;

use std::fs::File;
use std::io::Write;

use std::sync::{Arc};

use rand::{Rng};
use std::time;

// Assumes [0,1] input
fn write_color(buf: &mut String, color: Vec3) {
    let r: i64 = (255.0 * liner_to_gamma(color.x())).trunc() as i64;
    let g: i64 = (255.0 * liner_to_gamma(color.y())).trunc() as i64;
    let b: i64 = (255.0 * liner_to_gamma(color.z())).trunc() as i64;
    buf.push_str(format!("{} {} {} ", r, g, b).as_str());
}

fn write_new_line(buf: &mut String) {
    buf.push_str("\n");
}

fn liner_to_gamma(x: f64) -> f64 {
    return if x > 0.0 { x.sqrt() } else { 0.0 };
}

fn main() {
    let start = time::Instant::now();
    let mut file = File::create("test.ppm").unwrap();

    let camera = Camera::new();

    let mut world = HittableList::new();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    // Scene objects
    world.add(Arc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Vec3(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, material_right)));

    let str_buf = camera.parallel_render(16, world);

    file.write_all(str_buf.as_ref()).unwrap();

    println!("Done! {} s", start.elapsed().as_secs());
}
