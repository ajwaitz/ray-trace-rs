mod vec3;
mod interval;
mod buffer;

use vec3::{Vec3, dot, random_on_hemisphere_vec3};
use interval::{Interval};

use std::vec::Vec;

use std::fs::File;
use std::io::Write;

use std::sync::{Arc, Mutex};
use std::thread;

use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use std::time;

struct Ray {
    origin: Vec3,
    dir: Vec3
}

impl Ray {
    fn new(origin: Vec3, dir: Vec3) -> Self {
        return Self { origin, dir };
    }

    fn at(&self, t: f64) -> Vec3 {
        return self.origin + self.dir * t;
    }
}

#[derive(Copy, Clone)]
struct HitRecord {
    t: f64,
    point: Vec3,
    normal: Vec3,
    front_face: bool
}

impl HitRecord {
    fn new() -> Self {
        return Self { t: 0.0, point: Vec3(0.0, 0.0, 0.0), normal: Vec3(0.0, 0.0, 0.0), front_face:
        false};
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Copy, Clone)]
struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Self {
        return Self { center, radius };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let oc =  self.center - ray.origin;

        let a = ray.dir.length_squared();
        let h = dot(ray.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}

#[derive(Clone)]
struct HittableList {
    vec: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    fn new() -> Self {
        return Self { vec: Vec::new() };
    }

    fn add(&mut self, s: Arc<dyn Hittable>) {
        self.vec.push(s);
    }

    fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for s in self.vec.iter() {
            if (*s).hit(ray, Interval::new(interval.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}

struct Camera {
    image_height: i64,
    image_width: i64,
    aspect_ratio: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i64,
    max_depth: i64
}

impl Camera {
    // Define and return a generic camera
    fn new() -> Self {
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

    fn render(&self, world: HittableList) -> String {
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
                    color = color + get_ray_color(&ray, &world, self.max_depth);
                }

                write_color(&mut buf, color / (self.samples_per_pixel as f64));
            }
            write_new_line(&mut buf);
        }

        return buf;
    }

    fn render_pixel(&self, world: &HittableList, rng: &mut ThreadRng, i: i64, j: i64) -> Vec3 {
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
            color = color + get_ray_color(&ray, &world, self.max_depth);
        }

        return color / (self.samples_per_pixel as f64);
    }
}

// Assumes [0,1] input
fn write_color(buf: &mut String, color: Vec3) {
    let r: i64 = (255.0 * color.x()).trunc() as i64;
    let g: i64 = (255.0 * color.y()).trunc() as i64;
    let b: i64 = (255.0 * color.z()).trunc() as i64;
    buf.push_str(format!("{} {} {} ", r, g, b).as_str());
}

fn write_new_line(buf: &mut String) {
    buf.push_str("\n");
}

fn get_ray_color(ray: &Ray, world: &HittableList, depth: i64) -> Vec3 {
    if depth < 0 {
        return Vec3::EMPTY;
    }
    let mut hit_record = HitRecord::new();
    if world.hit(ray, Interval::FORWARD, &mut hit_record) {
        let dir = random_on_hemisphere_vec3(hit_record.normal);
        return get_ray_color(&Ray::new(hit_record.point, dir), world, depth - 1) * 0.5;
    }
    let unit_dir = ray.dir / ray.dir.x().abs().max(ray.dir.y().abs()).max(ray.dir.z().abs());
    let t = 0.5 * (unit_dir.y() + 1.0);
    return Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
}

fn main() {
    let start = time::Instant::now();
    let mut file = File::create("test.ppm").unwrap();

    let camera = Camera::new();

    let n = camera.image_height * camera.image_width * 3;
    let y_blocks = 16;
    let block_height = camera.image_height / y_blocks;
    let block_size = camera.image_width * 3;

    let buf = Arc::new(Mutex::new(vec![0.0; n as usize]));

    let mut handles = vec![];
    // iterate over blocks
    for j in 0..y_blocks {
        let buf_clone = Arc::clone(&buf);
        let block = j;
        let width = camera.image_width;
        let handle = thread::spawn(move || {
            let camera = Camera::new();
            let mut world = HittableList::new();
            world.add(Arc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
            world.add(Arc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

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

            let mut buf = buf_clone.lock().unwrap();
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
    str_buf.push_str(format!("P3\n{} {}\n255\n", camera.image_width, camera.image_width).as_str());

    for j in 0..camera.image_height {
        for i in 0..camera.image_width {
            let x = buf[(j * block_size + i * 3) as usize];
            let y = buf[(j * block_size + i * 3 + 1) as usize];
            let z = buf[(j * block_size + i * 3 + 2) as usize];
            write_color(&mut str_buf, Vec3::new(x, y, z));
        }
        write_new_line(&mut str_buf);
    }

    file.write_all(str_buf.as_ref()).unwrap();

    println!("Done! {} s", start.elapsed().as_secs());
}
