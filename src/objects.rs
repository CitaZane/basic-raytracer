pub mod sphere;
pub mod plane;
pub mod cube;
pub use sphere::*;
pub use plane::*;
pub use cube::*;

use crate::{ray::Ray, point3d::Point3D, material::Material};

pub trait Hittable {
    fn hit(&self, ray:&Ray, t_min:f64, t_max:f64) -> Option<Intersection>;
}

pub struct Intersection<'a>{
    pub point: Point3D,
    pub normal: Point3D,
    pub t :f64,
    pub material:&'a Material,
}