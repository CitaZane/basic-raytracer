use crate::material::Material;
use crate::{point3d::Point3D};
use crate::ray::Ray;

use super::{Hittable, Intersection};

pub struct Sphere{
    pub center: Point3D,
    pub radius: f64,
    pub material: Material,
}

impl Sphere{
    pub fn new(center:Point3D, radius:f64, material: Material)-> Self{
        Self { center, radius, material }
    }
}
impl Hittable for Sphere{
    fn hit(&self, ray:&Ray, t_min:f64, t_max:f64)-> Option<Intersection>{
        // transform origin so sphere now is origin-centered
        let oc = ray.origin - self.center;
        // calc quadric coefficients
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        // check intersection
        let disciminant = half_b*half_b - a*c;
        if disciminant<0.0 {return None}
        let sqrtd = disciminant.sqrt();

        // find the nearest root that lies in acaptable range
        // root is a intersection point
        let mut root = (-half_b - sqrtd) /a;
        if root < t_min || t_max < root{
            root = (-half_b + sqrtd) / a;
            if root <t_min || t_max < root{
                return None
            }
        }
        // front face tracking
        let point = ray.at(root);
        let normal = (point - self.center)/ self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;

        let intersection = Intersection{
            t : root,
            point,
            normal: if front_face {normal} else {normal * -1.},
            material: &self.material,
        };
        Some(intersection)
    }
}

