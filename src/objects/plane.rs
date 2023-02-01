use crate::{color::Color, material::*, point3d::Point3D, ray::Ray};

use super::{Hittable, Intersection};

pub struct Plane {
    point: Point3D,
    normal: Point3D,
    material: Material,
}

impl Plane {
    pub fn new(point: Point3D, normal: Point3D) -> Self {
        let matte = Matte::new(Color::gray());
        let material = Material::Matte(matte);
        Self {
            point,
            normal,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut Intersection) -> bool {
        let d_dot_n = self.normal.dot(&ray.direction);
        if d_dot_n == 0.0 {
            return false;
        } //in case ray paralel to plane
        let t = self.normal.dot(&(self.point - ray.origin)) / d_dot_n;
        if t <= t_min || t >= t_max {
            return false;
        }

        hit_record.point = ray.at(t);
        hit_record.normal = self.normal;
        hit_record.material = Some(self.material);
        hit_record.t = t;

        true
        // Some(Intersection {
        //     t,
        //     point: ray.at(t),
        //     normal: self.normal,
        //     material: &self.material,
        // })
    }
}
