use std::sync::{Arc, Mutex};
use std::thread;
use rand::prelude::ThreadRng;
use rand::{Rng, thread_rng};
use crate::world::{HitRecord, HittableList, Ray, Sphere};
use crate::{write_color, write_new_line};
use crate::interval::Interval;
use crate::vec3::{random_unit_vec3, Vec3};

pub struct Camera {
    pub image_height: i64,
    pub image_width: i64,
    pub aspect_ratio: f64,
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: i64,
    pub max_depth: i64
}

impl Camera {
    // Define and return a generic camera
    pub fn new() -> Self {
        let mut cam = Camera {
            image_height: 512,
            image_width: 512,
            aspect_ratio: 1.0,
            center: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            max_depth: 50
        };

        let focal_length = 1.0;
        let vh = 2.0;
        let vw = vh * (cam.image_width as f64) / (cam.image_height as f64);

        let viewport_u = Vec3::new(vw, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -vh, 0.0);

        cam.pixel_delta_u = viewport_u / (cam.image_width as f64);
        cam.pixel_delta_v = viewport_v / (cam.image_height as f64);

        let viewport_upper_left = cam.center - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u  / 2.0
            - viewport_v / 2.0;
        cam.pixel00_loc = viewport_upper_left + (cam.pixel_delta_u + cam.pixel_delta_v) * 0.5;

        return cam;
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList, depth: i64) -> Vec3 {
        if depth < 0 {
            return Vec3::EMPTY;
        }
        let mut hit_record = HitRecord::new();
        if world.hit(ray, Interval::ALMOST_FORWARD, &mut hit_record) {
            let mut scattered = Ray::EMPTY;
            let mut attenuation = Vec3::EMPTY;

            if hit_record.material.scatter(&ray, &hit_record, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }

            return Vec3::EMPTY;
        }
        let unit_dir = ray.dir / ray.dir.x().abs().max(ray.dir.y().abs()).max(ray.dir.z().abs());
        let t = 0.5 * (unit_dir.y() + 1.0);
        return Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
    }

    // Non-parallel
    pub fn render(&self, world: HittableList) -> String {
        let mut buf = String::new();

        buf.push_str(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_str());

        let mut rng = thread_rng();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (i as f64)) + (self
                    .pixel_delta_v * (j as
                    f64));

                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for s in 0..self.samples_per_pixel {
                    let x_noise = rng.gen_range(-0.5..0.5);
                    let y_noise = rng.gen_range(-0.5..0.5);
                    let new_pixel_center = pixel_center + self.pixel_delta_u *
                        x_noise + self.pixel_delta_v * y_noise;
                    let ray_dir = new_pixel_center - self.center;
                    let ray = Ray { origin: self.center, dir: ray_dir };
                    color = color + self.ray_color(&ray, &world, self.max_depth);
                }

                write_color(&mut buf, color / (self.samples_per_pixel as f64));
            }
            write_new_line(&mut buf);
        }

        return buf;
    }

    pub fn render_pixel(&self, world: &HittableList, rng: &mut ThreadRng, i: i64, j: i64) -> Vec3 {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (i as f64)) + (self
            .pixel_delta_v * (j as
            f64));

        let mut color = Vec3::new(0.0, 0.0, 0.0);
        for s in 0..self.samples_per_pixel {
            let x_noise = rng.gen_range(-0.5..0.5);
            let y_noise = rng.gen_range(-0.5..0.5);
            let new_pixel_center = pixel_center + self.pixel_delta_u *
                x_noise + self.pixel_delta_v * y_noise;
            let ray_dir = new_pixel_center - self.center;
            let ray = Ray { origin: self.center, dir: ray_dir };
            color = color + self.ray_color(&ray, &world, self.max_depth);
        }

        return color / (self.samples_per_pixel as f64);
    }

    pub fn parallel_render(&self, y_blocks: i64, world: HittableList) -> String {
        let n = self.image_height * self.image_width * 3;
        let block_height = self.image_height / y_blocks;
        let block_size = self.image_width * 3;

        let buf = Arc::new(Mutex::new(vec![0.0; n as usize]));
        let world = Arc::new(world);

        let mut handles = vec![];
        // iterate over blocks
        for j in 0..y_blocks {
            let buf = Arc::clone(&buf);
            let world = Arc::clone(&world);
            let block = j;
            let width = self.image_width;
            let handle = thread::spawn(move || {
                let camera = Camera::new();

                let mut rng = thread_rng();

                let q = block_height * block_size;
                let mut local_buf = vec![0.0; q as usize];

                // iterate internally on block
                for y in 0..block_height {
                    for x in 0..width {
                        let c = camera.render_pixel(&world, &mut rng,
                                                    x,
                                                    block * block_height + y);
                        local_buf[(y * block_size + x * 3) as usize] = c.x();
                        local_buf[(y * block_size + x * 3 + 1) as usize] = c.y();
                        local_buf[(y * block_size + x * 3 + 2) as usize] = c.z();
                    }
                }

                let mut buf = buf.lock().unwrap();
                // not convinced this will work
                buf[((block * block_height * block_size) as usize)..((((block + 1) * block_height) *
                    block_size) as
                    usize)].copy_from_slice(&local_buf);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let buf = buf.lock().unwrap();

        // Unwrapping buffer to a string
        let mut str_buf: String = String::new();
        str_buf.push_str(format!("P3\n{} {}\n255\n", self.image_width, self.image_width).as_str
        ());

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let x = buf[(j * block_size + i * 3) as usize];
                let y = buf[(j * block_size + i * 3 + 1) as usize];
                let z = buf[(j * block_size + i * 3 + 2) as usize];
                write_color(&mut str_buf, Vec3::new(x, y, z));
            }
            write_new_line(&mut str_buf);
        }

        return str_buf;
    }
}