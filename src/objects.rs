pub mod cube;
pub mod plane;
pub mod sphere;

pub use cube::*;
pub use plane::*;
pub use sphere::*;

use crate::{material::Material, point3d::Point3D, ray::Ray};
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

pub struct Intersection<'a> {
    pub point: Point3D,
    pub normal: Point3D,
    pub t: f64,
    pub material: &'a Material,
}
