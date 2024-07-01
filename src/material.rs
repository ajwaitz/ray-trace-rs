use crate::vec3::{Vec3, random_unit_vec3, reflect};
use crate::world::{Ray, HitRecord};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered_ray: &mut
    Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self { return Self { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered_ray: &mut
    Ray) -> bool {
        let mut dir = hit_record.normal + random_unit_vec3();

        // Catch degenerate scatter direction
        if dir.near_zero() {
            dir = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, dir);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self { return Self { albedo } }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered_ray: &mut Ray) -> bool {
        let reflected = reflect(ray.dir, hit_record.normal);
        *scattered_ray = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo;
        return true;
    }
}