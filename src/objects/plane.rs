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
    fn hit<'a>(&'a self, ray: &Ray, hit_record: &mut Intersection<'a>) -> bool {
        let d_dot_n = self.normal.dot(&ray.direction);
        if d_dot_n == 0.0 {
            return false;
        } //in case ray paralel to plane
        let t = self.normal.dot(&(self.point - ray.origin)) / d_dot_n;
        if t <= hit_record.t_min || t >= hit_record.t {
            return false;
        }
        if t > hit_record.t{
            return false
        }
        hit_record.point = ray.at(t);
        hit_record.normal = self.normal;
        hit_record.material = Some(&self.material);
        hit_record.t = t;
        hit_record.hit_anything = true;

        true
    }
}
