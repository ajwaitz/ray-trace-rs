mod vec3;
use vec3::{Vec3, dot};

use std::vec::Vec;

use std::fs::File;
use std::io::Write;

use std::sync::{Arc};

use rand::{thread_rng, Rng};

struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    const fn new(min: f64, max: f64) -> Self {
        return Self { min, max }
    }

    fn size(&self) -> f64 {
        return self.max - self.min;
    }

    fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
    const MAX: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);
    const FORWARD: Interval = Interval::new(0.0, f64::INFINITY);
}

struct Ray {
    origin: Vec3,
    dir: Vec3
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        return self.origin + self.dir * t;
    }
}

#[derive(Copy, Clone)]
struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
    front_face: bool
}

impl HitRecord {
    fn new() -> Self {
        return Self { t: 0.0, p: Vec3(0.0, 0.0, 0.0), normal: Vec3(0.0, 0.0, 0.0), front_face: false};
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
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}

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
    samples_per_pixel: i64
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
            samples_per_pixel: 10
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
                    color = color + get_ray_color(&ray, &world);
                }

                write_color(&mut buf, color / (self.samples_per_pixel as f64));
            }
            write_new_line(&mut buf);
        }

        return buf;
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

fn get_ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut hit_record = HitRecord::new();
    if world.hit(ray, Interval::FORWARD, &mut hit_record) {
        let normal = hit_record.normal;
        return Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }
    let unit_dir = ray.dir / ray.dir.x().abs().max(ray.dir.y().abs()).max(ray.dir.z().abs());
    let t = 0.5 * (unit_dir.y() + 1.0);
    return Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
}

fn main() {
    let mut file = File::create("test.ppm").unwrap();

    let camera = Camera::new();

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)));

    let buf = camera.render(world);

    file.write_all(buf.as_ref()).unwrap();

    println!("Done!");
}
