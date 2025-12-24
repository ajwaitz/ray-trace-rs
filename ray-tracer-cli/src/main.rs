use ray_tracer_core::*;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time;
use std::io::BufReader;

mod threaded_camera;
use threaded_camera::ThreadedCamera;

fn main() {
    let verbose = true;

    let start = time::Instant::now();
    let mut file = File::create("test.ppm").unwrap();

    let camera = ThreadedCamera::new();

    let mut world = HittableList::new();

    // Materials
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.1, 0.2, 0.5), 0.1));
    let material_left: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let input = BufReader::new(File::open("./shape.obj").unwrap());
    world.add(Arc::new(Polygon::new(input, &material_center)));

    println!("Done parsing .obj!");

    // Scene objects
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
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

    let str_buf = camera.render(world_ptr.clone(), 16, verbose);

    file.write_all(str_buf.as_ref()).unwrap();

    println!("Done! {} s", start.elapsed().as_secs());
}