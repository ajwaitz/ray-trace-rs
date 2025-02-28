mod camera;
mod interval;
mod material;
mod util;
mod vec3;
mod world;

use camera::Camera;
use material::{Lambertian, Material, Metal};
use world::{HittableList, Sphere};
use vec3::Vec3;

use std::sync::Arc;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render() -> Vec<u8> {
    let camera = Camera::new();

    let mut world = HittableList::new();

    // Materials
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    // let material_center: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.1, 0.2, 0.5), 0.1));
    let material_left: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    // Scene objects
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    let mut rng = thread_rng();
    let random_x: f64 = rng.gen_range(-0.2..0.2);
    let random_y: f64 = rng.gen_range(-0.2..0.2);
    // let random_radius: f64 = rng.gen_range(0.1..1.0);

    world.add(Arc::new(Sphere::new(
        Vec3(random_x, random_y, -0.5),
        0.1,
        &material_left,
    )));

    let world_ptr = Arc::new(world);

    let img = camera.render_single_thread(world_ptr.clone());

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .expect("Failed to write image");

    png_bytes
}
