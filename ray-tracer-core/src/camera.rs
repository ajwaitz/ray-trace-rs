use crate::interval::Interval;
use crate::material::ScatterResult;
use crate::vec3::Vec3;
use crate::world::{HitResult, HittableList, Ray};
use rand::rngs::ThreadRng;
use rand::Rng;

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
            let x_noise = rng.random_range(-0.5..0.5);
            let y_noise = rng.random_range(-0.5..0.5);
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

    // Single-threaded render method for core library
    pub fn render_single_threaded(&self, world: &HittableList) -> Vec<Vec3> {
        let mut rng = rand::rng();
        let mut pixels = Vec::with_capacity((self.image_height * self.image_width) as usize);
        
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let color = self.render_pixel(world, &mut rng, i, j);
                pixels.push(color);
            }
        }
        
        pixels
    }
}