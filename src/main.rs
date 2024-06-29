use std::fs::File;
use std::io::Write;

// Operator overloading
// https://doc.rust-lang.org/rust-by-example/trait/ops.html
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone)]
struct Vec3(f64, f64, f64);

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Vec3(x, y, z);
    }

    fn x(&self) -> f64 {
        return self.0;
    }

    fn y(&self) -> f64 {
        return self.1;
    }

    fn z(&self) -> f64 {
        return self.2;
    }

    fn sum(&self) -> f64 {
        return self.0 + self.1 + self.2;
    }

    fn length_squared(&self) -> f64 {
        return self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
    }

    fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        return Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z());
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        return Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z());
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Vec3 {
        return Vec3(self.x() - rhs, self.y() - rhs, self.z() - rhs);
    }
}


impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        return Vec3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z());
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Vec3 {
        return Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Vec3 {
        return Vec3(self.x() / rhs, self.y() / rhs, self.z() / rhs);
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        return Vec3(-self.x(), -self.y(), -self.z());
    }
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    return (a * b).sum();
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;

        let a = ray.dir.length_squared();
        let h = dot(ray.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-h - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrtd) / a;
            if root < t_min || t_max < root {
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

fn get_ray_color(ray: &Ray) -> Vec3 {
    let s = Sphere::new(Vec3(0.0, 0.0, 1.0), 0.5);
    let mut hit_record = HitRecord::new();
    if s.hit(ray, 0.0, std::f64::INFINITY, &mut hit_record) {
        let normal = (ray.at(hit_record.t) - Vec3(0.0, 0.0, -1.0)) / 0.5;
        return Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }
    let unit_dir = ray.dir / ray.dir.x().abs().max(ray.dir.y().abs()).max(ray.dir.z().abs());
    let t = 0.5 * (unit_dir.y() + 1.0);
    return Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
}

fn main() {
    let mut file = File::create("test.ppm").unwrap();

    // Image configuration
    let h = 512;
    let w = 512;

    // Camera configuration
    let focal_length = 1.0;
    let vh = 2.0;
    let vw = vh * (w as f64) / (h as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(vw, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -vh, 0.0);

    let pd_u = viewport_u / (w as f64);
    let pd_v = viewport_v / (h as f64);

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length)
                                    - viewport_u  / 2.0
                                    - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pd_u + pd_v) * 0.5;

    let mut buf = String::new();

    buf.push_str(format!("P3\n{} {}\n255\n", w, h).as_str());

    for j in 0..h {
        for i in 0..w {
            let pixel_center = pixel00_loc + (pd_u * (i as f64)) + (pd_v *
                (j as f64));
            let ray_dir = pixel_center - camera_center;
            let ray = Ray { origin: camera_center, dir: ray_dir };
            // let c = Vec3(((i as f64) / (w as f64)), ((j as f64) / (h as f64)), 0.0);
            let c = get_ray_color(&ray);
            write_color(&mut buf, c);
        }
        write_new_line(&mut buf);
    }

    file.write_all(buf.as_ref()).unwrap();

    println!("Done!");
}
