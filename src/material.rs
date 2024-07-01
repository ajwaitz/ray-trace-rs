use crate::vec3::{random_unit_vec3, reflect, Vec3, dot};
use crate::world::{HitRecord, HitResult, Ray};

pub enum ScatterResult {
    Scatter(Ray, Vec3),
    NoScatter
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord
    ) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        return Self { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord
    ) -> ScatterResult {
        let mut dir = hit_record.normal + random_unit_vec3();

        // Catch degenerate scatter direction
        if dir.near_zero() {
            dir = hit_record.normal;
        }

        let scattered_ray = Ray::new(hit_record.point, dir);
        let attenuation = self.albedo;

        return ScatterResult::Scatter(scattered_ray, attenuation);
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        return Self { albedo, fuzz };
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord
    ) -> ScatterResult {
        let reflected = reflect(ray.dir, hit_record.normal).unit() + random_unit_vec3() * self.fuzz;
        let scattered_ray = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        return if dot(reflected, hit_record.normal) > 0.0 {
            ScatterResult::Scatter(scattered_ray, attenuation)
        } else {
            ScatterResult::NoScatter
        }
    }
}
