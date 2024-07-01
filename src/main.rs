mod vec3;
mod interval;
mod buffer;
mod camera;
mod world;

use camera::Camera;
use world::{HittableList, Sphere};

use vec3::Vec3;

use std::fs::File;
use std::io::Write;

use std::sync::{Arc, Mutex};
use std::thread;

use rand::{thread_rng, Rng};
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
    world.add(Arc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    let str_buf = camera.parallel_render(16, world);

    file.write_all(str_buf.as_ref()).unwrap();

    println!("Done! {} s", start.elapsed().as_secs());
}
