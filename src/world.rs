use crate::interval::Interval;
use crate::material::{Lambertian, Material, Metal};
use crate::vec3::{dot, Vec3};
use std::sync::Arc;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, dir: Vec3) -> Self {
        return Self { origin, dir };
    }
    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin + self.dir * t;
    }

    pub const EMPTY: Self = Ray::new(Vec3::EMPTY, Vec3::EMPTY);
}

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        return Self {
            t: 0.0,
            point: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            front_face: false,
            material: Arc::new(Lambertian::new(Vec3::EMPTY)),
        };
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HittableList {
    pub vec: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        return Self { vec: Vec::new() };
    }

    pub fn add(&mut self, s: Arc<dyn Hittable>) {
        self.vec.push(s);
    }

    pub fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for s in self.vec.iter() {
            if (*s).hit(
                ray,
                Interval::new(interval.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        return Self {
            center,
            radius,
            material,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;

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
        rec.material = Arc::clone(&self.material);

        return true;
    }
}
