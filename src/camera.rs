use crate::interval::Interval;
use crate::material::ScatterResult;
use crate::util::{write_color, write_new_line, process_rgb};
use crate::vec3::Vec3;
use crate::world::{HitResult, HittableList, Ray};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;
use image::{ImageBuffer, Rgb};

#[derive(Copy, Clone)]
pub struct Camera {
    pub image_height: i64,
    pub image_width: i64,
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: i64,
    pub max_depth: i64,
}

impl Camera {
    // Define and return a generic camera
    pub fn new() -> Self {
        let mut cam = Camera {
            image_height: 512,
            image_width: 512,
            center: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            max_depth: 10,
        };

        let focal_length = 1.0;
        let vh = 2.0;
        let vw = vh * (cam.image_width as f64) / (cam.image_height as f64);
        let viewport_u = Vec3::new(vw, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -vh, 0.0);

        cam.pixel_delta_u = viewport_u / (cam.image_width as f64);
        cam.pixel_delta_v = viewport_v / (cam.image_height as f64);

        let viewport_upper_left =
            cam.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        cam.pixel00_loc = viewport_upper_left + (cam.pixel_delta_u + cam.pixel_delta_v) * 0.5;

        return cam;
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList, depth: i64) -> Vec3 {
        if depth < 0 {
            return Vec3::EMPTY;
        }
        if let HitResult::Hit(hit_record) = world.hit(ray, Interval::ALMOST_FORWARD) {
            if let ScatterResult::Scatter(scattered, attenuation) =
                hit_record.material.scatter(&ray, &hit_record)
            {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }

            return Vec3::EMPTY;
        }
        let unit_dir = ray.dir
            / ray
                .dir
                .x()
                .abs()
                .max(ray.dir.y().abs())
                .max(ray.dir.z().abs());
        let t = 0.5 * (unit_dir.y() + 1.0);
        return Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
    }

    pub fn render_pixel(&self, world: &HittableList, rng: &mut ThreadRng, i: i64, j: i64) -> Vec3 {
        let pixel_center = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64))
            + (self.pixel_delta_v * (j as f64));

        let mut color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..self.samples_per_pixel {
            let x_noise = rng.gen_range(-0.5..0.5);
            let y_noise = rng.gen_range(-0.5..0.5);
            let new_pixel_center =
                pixel_center + self.pixel_delta_u * x_noise + self.pixel_delta_v * y_noise;
            let ray_dir = new_pixel_center - self.center;
            let ray = Ray {
                origin: self.center,
                dir: ray_dir,
            };
            color = color + self.ray_color(&ray, &world, self.max_depth);
        }

        return color / (self.samples_per_pixel as f64);
    }

    pub fn render_single_thread(&self, world: Arc<HittableList>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(self.image_width as u32, self.image_height as u32);

        let mut rng = thread_rng();
        for (i, j, pixel) in img.enumerate_pixels_mut() {
            let c = self.render_pixel(&world, &mut rng, i as i64, j as i64);
            *pixel = process_rgb(c);
        }

        return img;
    }

    pub fn render(&self, world: Arc<HittableList>, y_blocks: i64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let buf_size = self.image_height * self.image_width * 3;
        let block_height = self.image_height / y_blocks;
        let block_size = self.image_width * 3;

        let buf = Arc::new(Mutex::new(vec![0.0; buf_size as usize]));

        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        // iterate over blocks
        for j in 0..y_blocks {
            let camera: Camera = *self;
            let buf: Arc<Mutex<Vec<f64>>> = Arc::clone(&buf);
            let world: Arc<HittableList> = world.clone();
            let block: i64 = j;
            let width: i64 = self.image_width;

            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                let mut rng = thread_rng();

                let q = block_height * block_size;
                let mut local_buf = vec![0.0; q as usize];

                // iterate internally on block
                for y in 0..block_height {
                    for x in 0..width {
                        let c = camera.render_pixel(&world, &mut rng, x, block * block_height + y);
                        local_buf[(y * block_size + x * 3) as usize] = c.x();
                        local_buf[(y * block_size + x * 3 + 1) as usize] = c.y();
                        local_buf[(y * block_size + x * 3 + 2) as usize] = c.z();
                    }
                }

                let mut buf = buf.lock().unwrap();
                buf[((block * block_height * block_size) as usize)
                    ..((((block + 1) * block_height) * block_size) as usize)]
                    .copy_from_slice(&local_buf);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let buf = buf.lock().unwrap();

        // Unwrapping buffer to a string

        let mut img = ImageBuffer::new(self.image_width as u32, self.image_height as u32);

        for (i, j, pixel) in img.enumerate_pixels_mut() {
            let idx = (j as i64 * block_size + i as i64 * 3) as usize;

            let x = buf[idx];
            let y = buf[idx + 1];
            let z = buf[idx + 2];

            *pixel = process_rgb(Vec3::new(x, y, z));
        }

        return img;
    }
}
