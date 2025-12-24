pub mod camera;
pub mod interval;
pub mod material;
pub mod scene;
pub mod util;
pub mod vec3;
pub mod world;

pub use camera::Camera;
pub use interval::Interval;
pub use material::{Lambertian, Material, Metal, ScatterResult};
pub use scene::{default_scene, SceneMaterials};
pub use util::{liner_to_gamma, write_color, write_new_line};
pub use vec3::Vec3;
pub use world::{HitResult, HittableList, Polygon, Ray, Sphere, Triangle};