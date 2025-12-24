use image::{ImageBuffer, Rgb};
use rand::Rng;
use ray_tracer_core::*;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

// Import the `console.log` function from the Web API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ))
    }
}

// Helper function to convert Vec3 color to Rgb<u8>
fn process_rgb(color: Vec3) -> Rgb<u8> {
    let r = (255.0 * liner_to_gamma(color.x())).trunc() as u8;
    let g = (255.0 * liner_to_gamma(color.y())).trunc() as u8;
    let b = (255.0 * liner_to_gamma(color.z())).trunc() as u8;
    Rgb([r, g, b])
}

fn render_to_png(camera: &Camera, world: &HittableList) -> Vec<u8> {
    let pixels = camera.render_single_threaded(world);
    let mut img = ImageBuffer::new(camera.image_width as u32, camera.image_height as u32);

    for (i, j, pixel) in img.enumerate_pixels_mut() {
        let idx = (j as usize) * (camera.image_width as usize) + (i as usize);
        *pixel = process_rgb(pixels[idx]);
    }

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )
    .expect("Failed to write image");

    png_bytes
}

// Standalone render function with randomized sphere positions
#[wasm_bindgen]
pub fn render(samples: i64) -> Vec<u8> {
    let mut camera = Camera::new();
    camera.samples_per_pixel = samples;

    let mats = SceneMaterials::default();
    let mut world = HittableList::new();

    // Ground
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        &mats.ground,
    )));

    // Randomized spheres
    let mut rng = rand::rng();
    let random_x: f64 = rng.random_range(-1.0..1.0);
    world.add(Arc::new(Sphere::new(
        Vec3(random_x, 0.0, -1.0),
        0.5,
        &mats.right,
    )));

    let random_x: f64 = rng.random_range(-0.3..0.3);
    let random_y: f64 = rng.random_range(-0.3..0.3);
    world.add(Arc::new(Sphere::new(
        Vec3(random_x, random_y, -0.5),
        0.1,
        &mats.left,
    )));

    render_to_png(&camera, &world)
}

#[wasm_bindgen]
pub struct WasmRayTracer {
    camera: Camera,
    world: HittableList,
}

#[wasm_bindgen]
impl WasmRayTracer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmRayTracer {
        WasmRayTracer {
            camera: Camera::new(),
            world: default_scene(),
        }
    }

    #[wasm_bindgen]
    pub fn render(&self) -> Vec<u8> {
        console_log!("Starting WASM ray trace render...");
        let result = render_to_png(&self.camera, &self.world);
        console_log!("WASM render complete!");
        result
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> i64 {
        self.camera.image_width
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> i64 {
        self.camera.image_height
    }
}