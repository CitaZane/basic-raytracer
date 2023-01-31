use crate::material::Material;
use crate::point3d::Point3D;
use crate::ray::Ray;

use super::{Hittable, Intersection};
pub struct Cylinder {
    pub material: Material,
    pub base: Point3D,
    pub axis: Point3D,
    pub radius: f64,
    pub height: f64,
}

impl Cylinder {
    pub fn new(base: Point3D, axis: Point3D, radius: f64, material: Material, height: f64) -> Self {
        Self {
            base,
            axis,
            radius,
            material,
            height,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let ob = ray.origin - self.base;

        let a = ray.direction.dot(&ray.direction) - (ray.direction.dot(&self.axis)).powf(2.0);
        let b = 2.0 * (ray.direction.dot(&ob) - ray.direction.dot(&self.axis) * ob.dot(&self.axis));
        let c = ob.dot(&ob) - (ob.dot(&self.axis)).powf(2.0) - self.radius * self.radius;

        let discriminant = b.powf(2.) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-b + sqrtd) / (2.0 * a);

        if root < t_min || t_max < root {
            root = (-b - sqrtd) / (2.0 * a);
            if root < t_min || t_max < root {
                return None;
            }
        }
        let point = ray.at(root);
        let m = ray.direction.dot(&self.axis) * root + ob.dot(&self.axis);
        let normal = (point - self.base - self.axis * m).unit_vector();
        Some(Intersection {
            point,
            t: root,
            normal,
            material: &self.material,
        })
    }
}
