use crate::material::{Light, Material};
use crate::point3d::Point3D;
use crate::ray::Ray;

use super::{Hittable, Intersection};

pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
    pub fn new_light(center: Point3D, intensity: f64) -> Self {
        let l = Light::new(intensity);
        let material = Material::Light(l);
        Self {
            center,
            radius: 1.,
            material,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut Intersection) -> bool {
        // transform origin so sphere now is origin-centered
        let oc = ray.origin - self.center;
        // calc quadric coefficients
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        // check intersection
        let disciminant = half_b * half_b - a * c;
        if disciminant < 0.0 {
            return false;
        }
        let sqrtd = disciminant.sqrt();

        // find the nearest root that lies in acaptable range
        // root is a intersection point
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        // front face tracking
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;

        hit_record.point = ray.at(root);
        hit_record.normal = if front_face { normal } else { normal * -1. };
        hit_record.material = Some(self.material);
        hit_record.t = root;
        true
        // Some(Intersection {
        //     t: root,
        //     point,
        //     normal: if front_face { normal } else { normal * -1. },
        //     material: &self.material,
        // })
    }
}
