use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::vec3::Vec3;
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
        self.front_face = Vec3::dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub enum HitResult {
    Hit(HitRecord),
    Miss,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Interval) -> HitResult;
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

    pub fn hit(&self, ray: &Ray, interval: Interval) -> HitResult {
        let mut rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for s in self.vec.iter() {
            let hit = (*s).hit(ray, Interval::new(interval.min, closest_so_far));
            if let HitResult::Hit(temp_rec) = hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec.clone();
            }
        }

        return if hit_anything {
            HitResult::Hit(rec)
        } else {
            HitResult::Miss
        };
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: &Arc<dyn Material>) -> Self {
        return Self {
            center,
            radius,
            material: Arc::clone(material),
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> HitResult {
        let oc = self.center - ray.origin;

        let a = ray.dir.length_squared();
        let h = Vec3::dot(ray.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return HitResult::Miss;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return HitResult::Miss;
            }
        }

        let mut rec = HitRecord::new();

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Arc::clone(&self.material);

        return HitResult::Hit(rec);
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: &Arc<dyn Material>) -> Self {
        return Self {
            a,
            b,
            c,
            material: Arc::clone(material),
        };
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, interval: Interval) -> HitResult {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let normal = Vec3::cross(ab, ac).unit();

        if Vec3::dot(ray.dir, normal) == 0.0 {
            return HitResult::Miss;
        }

        let b = ray.origin - self.a;

        let det_a = Vec3::det(-ray.dir, ab, ac);
        let t = Vec3::det(b, ab, ac) / det_a;
        let u = Vec3::det(-ray.dir, b, ac) / det_a;
        let v = Vec3::det(-ray.dir, ab, b) / det_a;

        if u < 0.0 || v < 0.0 || u + v > 1.0 || !interval.contains(t) {
            return HitResult::Miss;
        }

        let mut rec = HitRecord::new();
        rec.t = t;
        rec.point = ray.at(rec.t);
        rec.set_face_normal(ray, normal);
        rec.material = Arc::clone(&self.material);

        return HitResult::Hit(rec);
    }
}
