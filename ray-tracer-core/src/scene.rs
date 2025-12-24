use crate::material::{Lambertian, Material, Metal};
use crate::vec3::Vec3;
use crate::world::{HittableList, Sphere};
use std::sync::Arc;

/// Creates a default scene with a ground plane and three spheres.
pub fn default_scene() -> HittableList {
    let mut world = HittableList::new();

    // Materials
    let material_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.1, 0.2, 0.5), 0.1));
    let material_left: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    // Ground
    world.add(Arc::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));

    // Left sphere
    world.add(Arc::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    )));

    // Right sphere
    world.add(Arc::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    world
}

/// Returns standard materials used in scenes.
pub struct SceneMaterials {
    pub ground: Arc<dyn Material>,
    pub center: Arc<dyn Material>,
    pub left: Arc<dyn Material>,
    pub right: Arc<dyn Material>,
}

impl SceneMaterials {
    pub fn default() -> Self {
        SceneMaterials {
            ground: Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
            center: Arc::new(Metal::new(Vec3::new(0.1, 0.2, 0.5), 0.1)),
            left: Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3)),
            right: Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
        }
    }
}

