use ray_tracer_core::*;
use std::fs::File;
use std::io::{BufReader, Write};
use std::sync::Arc;
use std::time;
use clap::Parser;

mod threaded_camera;
use threaded_camera::ThreadedCamera;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the output file
    #[arg(short, long)]
    filename: String,

    /// Verbosity
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let filename = args.filename;
    let mut verbose = args.verbose;

    let start = time::Instant::now();
    let mut file = File::create(filename).unwrap();

    let camera = ThreadedCamera::new();

    // Start with default scene (ground + spheres)
    let mut world = default_scene();

    // Load .obj mesh with center material
    let mats = SceneMaterials::default();
    let input = BufReader::new(File::open("./shape.obj").unwrap());
    world.add(Arc::new(Polygon::new(input, &mats.center)));
    println!("Done parsing .obj!");

    // Add an extra small sphere
    world.add(Arc::new(Sphere::new(Vec3(0.5, 0.0, -0.7), 0.05, &mats.left)));

    let world_ptr = Arc::new(world);
    let str_buf = camera.render(world_ptr.clone(), 16, verbose);

    file.write_all(str_buf.as_ref()).unwrap();

    println!("Done! {} s", start.elapsed().as_secs());
}