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
    fn hit<'a>(&'a self, ray: &Ray, hit_record: &mut Intersection<'a>) -> bool;
}

#[derive(Clone, Copy)]
pub struct Intersection <'a>{
    pub point: Point3D,
    pub normal: Point3D,
    pub t: f64,
    pub material: Option<&'a Material>,
    pub t_min:f64,
    pub hit_anything: bool,
}

impl <'a>Intersection<'a>{
    pub fn new() -> Intersection<'a> {
        Intersection {
            point: Point3D::new(0., 0., 0.),
            normal: Point3D::new(0., 0., 0.),
            t: f64::MAX,
            material: None,
            hit_anything: false,
            t_min: 0.001,
        }
    }
}
