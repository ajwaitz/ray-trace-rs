mod camera;
mod interval;
mod material;
mod util;
mod vec3;
mod world;

use camera::Camera;
use material::{Lambertian, Material, Metal};
use world::{HittableList, Sphere, Triangle, Polygon};
use vec3::Vec3;

use std::fs::File;
use std::io::Write;

use std::sync::Arc;

use std::time;

use std::io::BufReader;

// fn main() {
//     let start = time::Instant::now();
//     let mut file = File::create("test.ppm").unwrap();

//     let camera = Camera::new();

//     let mut world = HittableList::new();

//     let input = BufReader::new(File::open("./shape.obj").unwrap());
//     world.add(Arc::new(Polygon::new(input)));

//     println!("Done parsing .obj!");

//     // Materials
//     let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
//     // let material_center: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.1, 0.2, 0.5), 0.1));
//     let material_left: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
//     let material_right: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

//     // Scene objects
//     world.add(Arc::new(Sphere::new(
//         Vec3(0.0, -100.5, -1.0),
//         100.0,
//         &material_ground,
//     )));
//     // world.add(Arc::new(Sphere::new(
//     //     Vec3(0.0, 0.0, -1.2),
//     //     0.5,
//     //     &material_center,
//     // )));
//     // world.add(Arc::new(Sphere::new(
//     //     Vec3(-1.0, 0.0, -1.0),
//     //     0.5,
//     //     &material_left,
//     // )));
//     world.add(Arc::new(Sphere::new(
//         Vec3(1.0, 0.0, -1.0),
//         0.5,
//         &material_right,
//     )));
//     world.add(Arc::new(Sphere::new(
//         Vec3(0.5, 0.0, -0.7),
//         0.05,
//         &material_left,
//     )));
//     // world.add(Arc::new(Sphere::new(
//     //     Vec3(-0.5, 0.0, -1.2),
//     //     0.05,
//     //     &material_right,
//     // )));
//     // world.add(Arc::new(Triangle::new(
//     //     Vec3(0.7, -0.2, -1.2),
//     //     Vec3(-0.7, -0.2, -0.8),
//     //     Vec3(0.0, 0.7, -1.5),
//     //     &material_right,
//     // )));

//     let world_ptr = Arc::new(world);

//     let str_buf = camera.render(world_ptr.clone(), 16);

//     file.write_all(str_buf.as_ref()).unwrap();

//     println!("Done! {} s", start.elapsed().as_secs());
// }

pub fn main() {
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
    world.add(Arc::new(Sphere::new(
        Vec3(0.5, 0.0, -0.7),
        0.05,
        &material_left,
    )));

    let world_ptr = Arc::new(world);

    let img = camera.render(world_ptr.clone(), 16);

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .expect("Failed to write image");

    println!("Done! Output vector: {:?}", png_bytes);
}
