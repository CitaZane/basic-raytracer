pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

pub use cube::*;
pub use cylinder::*;
pub use plane::*;
pub use sphere::*;

use crate::{material::Material, point3d::Point3D, ray::Ray};
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut Intersection) -> bool;
}

#[derive(Clone, Copy)]
pub struct Intersection {
    pub point: Point3D,
    pub normal: Point3D,
    pub t: f64,
    pub material: Option<Material>,
    pub hit_anything: bool,
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            point: Point3D::new(0., 0., 0.),
            normal: Point3D::new(0., 0., 0.),
            t: 0.,
            material: None,
            hit_anything: false,
        }
    }
}
