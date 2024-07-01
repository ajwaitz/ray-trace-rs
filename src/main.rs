mod camera;
mod interval;
mod material;
mod vec3;
mod world;
mod util;

use camera::Camera;
use material::{Lambertian, Material, Metal};
use world::{HittableList, Sphere, Triangle};

use vec3::Vec3;

use std::fs::File;
use std::io::Write;

use std::sync::Arc;

use rand::Rng;
use std::time;

fn main() {
    let start = time::Instant::now();
    let mut file = File::create("test.ppm").unwrap();

    let camera = Camera::new();

    let mut world = HittableList::new();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    // Scene objects
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    // world.add(Arc::new(Sphere::new(
    //     Vec3(0.0, 0.0, -1.2),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Vec3(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Vec3(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));
    world.add(Arc::new(Sphere::new(
        Vec3(0.5, 0.0, -1.2),
        0.05,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3(-0.5, 0.0, -1.2),
        0.05,
        material_right,
    )));
    world.add(Arc::new(Triangle::new(
        Vec3(0.7, -0.2, -1.2),
        Vec3(-0.7, -0.2, -0.8),
        Vec3(0.0, 0.7, -1.5),
        material_center
    )));

    let str_buf = camera.parallel_render(16, world);

    file.write_all(str_buf.as_ref()).unwrap();

    println!("Done! {} s", start.elapsed().as_secs());
}
