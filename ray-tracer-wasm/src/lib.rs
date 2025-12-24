use ray_tracer_core::*;
use wasm_bindgen::prelude::*;
use std::sync::Arc;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};

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

// Standalone render function for simpler usage (like main branch)
#[wasm_bindgen]
pub fn render(samples: i64) -> Vec<u8> {
    let mut camera = Camera::new();
    camera.samples_per_pixel = samples;

    let mut world = HittableList::new();

    // Materials
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_left: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    // Scene objects
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));

    let mut rng = thread_rng();
    let random_x: f64 = rng.gen_range(-1.0..1.0);

    world.add(Arc::new(Sphere::new(
        Vec3(random_x, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    let random_x: f64 = rng.gen_range(-0.3..0.3);
    let random_y: f64 = rng.gen_range(-0.3..0.3);

    world.add(Arc::new(Sphere::new(
        Vec3(random_x, random_y, -0.5),
        0.1,
        &material_left,
    )));

    let pixels = camera.render_single_threaded(&world);
    let mut img = ImageBuffer::new(camera.image_width as u32, camera.image_height as u32);

    for (i, j, pixel) in img.enumerate_pixels_mut() {
        let idx = (j as usize) * (camera.image_width as usize) + (i as usize);
        *pixel = process_rgb(pixels[idx]);
    }

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .expect("Failed to write image");

    png_bytes
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
        let camera = Camera::new();
        let mut world = HittableList::new();

        // Default materials
        let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
        let material_center: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.1, 0.2, 0.5), 0.1));
        let material_left: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
        let material_right: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

        // Default scene objects
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

        WasmRayTracer { camera, world }
    }

    #[wasm_bindgen]
    pub fn render(&self) -> Vec<u8> {
        console_log!("Starting WASM ray trace render...");
        
        let pixels = self.camera.render_single_threaded(&self.world);
        let mut img = ImageBuffer::new(self.camera.image_width as u32, self.camera.image_height as u32);
        
        // pixels are in row-major order: row 0, row 1, etc.
        for (i, j, pixel) in img.enumerate_pixels_mut() {
            let idx = (j as usize) * (self.camera.image_width as usize) + (i as usize);
            *pixel = process_rgb(pixels[idx]);
        }
        
        let mut png_bytes: Vec<u8> = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .expect("Failed to write image");
        
        console_log!("WASM render complete!");
        png_bytes
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